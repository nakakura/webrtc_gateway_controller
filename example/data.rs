mod terminal;

use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use either;
use either::Either;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::prelude::*;
use log::{error, info, warn};
use serde_derive::Deserialize;

use webrtc_gateway_controller::data::{CreatedResponse, DataIdWrapper};
use webrtc_gateway_controller::prelude::*;
use webrtc_gateway_controller::*;

//==================== for parsing data.toml ====================

// It shows config toml formats
#[derive(Debug, Deserialize)]
struct Config {
    peer: PeerConfig,
    gateway: SocketConfig,
    redirects: Vec<SocketConfig>,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct PeerConfig {
    peer_id: String,
    domain: String,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct SocketConfig {
    ip: String,
    port: u16,
}

// read config from toml file
fn read_config(path: &'static str) -> Config {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .expect("file open error");

    fr.read_to_string(&mut file_content)
        .expect("file read error");
    toml::from_str(&file_content).expect("toml parse error")
}

//==================== main ====================

// Main function creates a peer object and start listening peer events and keyboard events.
// Further processes are triggered by these events.
#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    //load and set parameters
    let config = read_config("example/data.toml");
    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let domain = config.peer.domain;
    let peer_id = PeerId::new(config.peer.peer_id);
    let base_url: String = format!("http://{}:{}", config.gateway.ip, config.gateway.port);
    webrtc_gateway_controller::initialize(base_url);

    //observe keyboard events
    let (key_notifier, key_observer) = mpsc::channel::<String>(0);
    let key_fut = terminal::read(key_notifier);

    //create peer and observe peer events
    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    let peer_info = create_peer_future.await.expect("create peer failed");
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(0);
    let event_future = peer::listen_events(&peer_info, peer_event_notifier);

    //this program reacts only to user input and peer events.
    //merge these two event streams, and fold current status
    let peer_event_stream = futures::stream::select(
        peer_event_observer.map(|e| Either::Left(e)),
        key_observer.map(|e| Either::Right(e)),
    );
    let fold_fut = peer_event_stream
        .fold(
            // before receiving Peer::OPEN event, peer object might not be created.
            // So I set None for PeerInfo.
            PeerFoldState((None, config.redirects, vec![])),
            |status, event| async move {
                match event {
                    Either::Left(api_events) => on_peer_api_events(status, api_events)
                        .await
                        .expect("error in on_peer_api_events"),
                    Either::Right(key_events) => on_peer_key_events(status, key_events)
                        .await
                        .expect("error in on_peer_key_events"),
                }
            },
        )
        .map(|_| futures::future::ok::<(), error::Error>(()));

    //execute all the futures
    let (fold_fut_result, event_fut_result, key_fut_reult) =
        futures::future::join3(fold_fut, event_future, key_fut).await;
    info!("All the futures are finished. They stopped with these status\nfold: {:?}\nevent: {:?}\nkey:{:?}", fold_fut_result, event_fut_result, key_fut_reult);
}

//==================== materials for PeerFold ====================

// Wrap user input strings with New-Type pattern
#[derive(Debug)]
struct ControlMessage(String);

// This program reacts only with two streams, user control from stdin and peer events.
// This struct holds previous state to process the events properly.
#[derive(Debug)]
struct PeerFoldState(
    (
        Option<PeerInfo>,
        Vec<SocketConfig>,
        Vec<mpsc::Sender<ControlMessage>>,
    ),
);

// PeerFoldState has only setter and getter.
impl PeerFoldState {
    pub fn peer_info(&self) -> &Option<PeerInfo> {
        &(self.0).0
    }

    pub fn set_peer_info(self, peer_info: Option<PeerInfo>) -> Self {
        let PeerFoldState((_, redirects, sender)) = self;
        PeerFoldState((peer_info, redirects, sender))
    }

    pub fn socket_config(&mut self) -> Option<SocketConfig> {
        (self.0).1.pop()
    }

    pub fn control_message_notifier(&mut self) -> &mut Vec<mpsc::Sender<ControlMessage>> {
        &mut (self.0).2
    }

    pub fn set_control_message_notifier(&mut self, tx: mpsc::Sender<ControlMessage>) {
        (&mut (self.0).2).push(tx);
    }
}

//==================== process events for peer ====================

// This function process events from Peer Object
async fn on_peer_api_events(
    status: PeerFoldState,
    event: PeerEventEnum,
) -> Result<PeerFoldState, error::Error> {
    let status = match event {
        PeerEventEnum::OPEN(event) => {
            // PeerObject notify that it has been successfully created.
            // Hold PeerInfo for further process.
            info!(
                "====================\nPeerObject is created. Its PeerId is {} and Token is {}",
                event.params.peer_id.as_str(),
                event.params.token.as_str()
            );
            let params = status.set_peer_info(Some(event.params));
            Ok(params)
        }
        PeerEventEnum::CLOSE(event) => {
            // PeerObject notify that it has already been deleted.
            // Erase old PeerInfo.
            info!(
                "====================\nPeerObject of {} is closed",
                event.params.peer_id.as_str()
            );
            let params = status.set_peer_info(None);
            Ok(params)
        }
        PeerEventEnum::CONNECTION(event) => {
            // In this timing, DataChannel itself has already been established.
            // To send and recv data, call redirect API.
            info!(
                "====================\nconnection is established by neighbour. Its DataConnectionId is {}",
                event.data_params.data_connection_id.as_str()
            );
            redirect(status, event.data_params.data_connection_id).await
        }
        PeerEventEnum::ERROR(event) => {
            error!("error {:?} occurs in on_peer_api_events", event);
            Ok(status)
        }
        _ => Ok(status),
    };
    print_commands();
    status
}

// This function works according to User Keyboard Input
async fn on_peer_key_events(
    mut status: PeerFoldState,
    message: String,
) -> Result<PeerFoldState, error::Error> {
    let status = match message.as_str() {
        "exit" => {
            // When an user wants to close this program, it needs to close P2P links and delete Peer Object.
            // Content Socket will be automaticall released, so it is not necessary to release them manually.
            // https://github.com/skyway/skyway-webrtc-gateway/blob/master/docs/release_process.md
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier
                    .send(ControlMessage(String::from("disconnect_all")))
                    .await;
            }
            if let Some(peer_info) = status.peer_info() {
                let _unit = peer::delete(&peer_info).await?;
            }
            let params = status.set_peer_info(None);
            Ok(params)
        }
        "status" => {
            // Show status of PeerObject and DataConnection
            if let Some(ref peer_info) = status.peer_info() {
                let status = peer::status(peer_info).await?;
                let mut message = String::from("====================\nShow Status");
                message = format!(
                    "{}\nMy PeerId is {} and Token is {}. It is {} now.",
                    message,
                    peer_info.peer_id.as_str(),
                    peer_info.token.as_str(),
                    if !status.disconnected {
                        "connected to Server."
                    } else {
                        "not connected to Server."
                    }
                );
                info!("{}", message);
            }
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier.send(ControlMessage(String::from("status"))).await;
            }
            Ok(status)
        }
        message if message.starts_with("disconnect ") => {
            // Disconnect P2P link
            // This function expects "connect DATA_CONNECTION_ID".
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier.send(ControlMessage(String::from(message))).await;
            }
            Ok(status)
        }
        message if message.starts_with("connect ") => {
            // Establish P2P datachannel to an neighbour.
            // This function expects "connect TARGET_ID".
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(target_id) = args.next() {
                let target_id = PeerId::new(target_id);
                Ok(connect(status, target_id).await.expect("error at line 163"))
            } else {
                warn!("input \"connect TARGET_PEER_ID\"");
                Ok(status)
            }
        }
        _ => Ok(status),
    };
    print_commands();
    status
}

//==================== connect and redirect to establish data connection ====================

// start establishing DataConnection to an neighbour
async fn connect(
    mut params: PeerFoldState,
    target_id: PeerId,
) -> Result<PeerFoldState, error::Error> {
    // Notify which peer object needs to establish P2P link to WebRTC Gateway.
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");

    // Data received from this content socket will be redirected to neighbour with DataConnection.
    let data_socket_created_response = data::open_source_socket().await?;

    // Data received from DataConnection will be redirected according to this information.
    let redirect_info = params.socket_config().map(|socket_config| {
        SocketInfo::<PhantomId>::try_create(None, &socket_config.ip, socket_config.port)
            .expect("invalid data port")
    });

    // set up query and access to connect API.
    let query = data::CreateDataConnectionQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        options: None,
        target_id: target_id,
        params: Some(DataIdWrapper {
            data_id: data_socket_created_response.data_id.clone(),
        }),
        redirect_params: redirect_info.clone(),
    };
    let data_connection_id = data::connect(query).await?;

    // Notify keyboard inputs to the sub-task with this channel
    let (control_message_notifier, control_message_observer) = mpsc::channel::<ControlMessage>(0);
    // hold notifier
    params.set_control_message_notifier(control_message_notifier);

    // listen DataConnection events and send them with this channel
    let (dc_event_notifier, dc_event_observer) = mpsc::channel::<data::DataConnectionEventEnum>(0);
    let event_listen_fut = data::listen_events(data_connection_id.clone(), dc_event_notifier);
    tokio::spawn(event_listen_fut);

    // DataConnection process will work according to DataConnection events and keyboard inputs
    let stream = futures::stream::select(
        dc_event_observer.map(|event| Either::Left(event)),
        control_message_observer.map(|event| Either::Right(event)),
    );
    let mut state = DataConnectionState::default();
    state.insert_data_connection_id(
        data_connection_id,
        (Some(data_socket_created_response), redirect_info),
    );
    let fold_fut = stream.fold(state, |state, event| async move {
        match event {
            Either::Left(event) => on_data_api_events(state, event)
                .await
                .expect("error in on_data_api_events"),
            Either::Right(event) => on_data_key_events(state, event)
                .await
                .expect("error in on_data_key_events"),
        }
    });
    tokio::spawn(fold_fut);

    Ok(params)
}

// set input and output port to DataConnection which has already been established.
async fn redirect(
    mut params: PeerFoldState,
    data_connection_id: DataConnectionId,
) -> Result<PeerFoldState, error::Error> {
    // Data received from this content socket will be redirected to neighbour with DataConnection.
    let data_socket_created_response = data::open_source_socket().await?;
    // Data received from DataConnection will be redirected according to this information.
    // If there is no redirect infor in config.toml, redirect info will be None.
    // In this case, the data channel is virtually sendonly.
    let redirect_info = params.socket_config().map(|socket_config| {
        SocketInfo::<PhantomId>::try_create(None, &socket_config.ip, socket_config.port)
            .expect("invalid data port")
    });
    let redirect_params = data::RedirectDataParams {
        feed_params: Some(DataIdWrapper {
            data_id: data_socket_created_response.data_id.clone(),
        }),
        redirect_params: redirect_info.clone(),
    };
    let _ = data::redirect(&data_connection_id, &redirect_params)
        .await
        .expect("redirect data failed");

    // Notify keyboard inputs to the sub-task with this channel
    let (control_message_notifier, control_message_observer) = mpsc::channel::<ControlMessage>(0);
    // hold the notifier
    params.set_control_message_notifier(control_message_notifier);

    // listen DataConnection events and send them with this channel
    let (dc_event_notifier, dc_event_observer) = mpsc::channel::<data::DataConnectionEventEnum>(0);
    let event_listen_fut = data::listen_events(data_connection_id.clone(), dc_event_notifier);
    tokio::spawn(event_listen_fut);

    // DataConnection process will work according to DataConnection events and keyboard inputs
    let stream = futures::stream::select(
        dc_event_observer.map(|event| Either::Left(event)),
        control_message_observer.map(|event| Either::Right(event)),
    );
    let mut state = DataConnectionState::default();
    state.insert_data_connection_id(
        data_connection_id,
        (Some(data_socket_created_response), redirect_info),
    );
    let fold_fut = stream.fold(state, |state, event| async move {
        match event {
            Either::Left(event) => on_data_api_events(state, event)
                .await
                .expect("error in on_data_api_events"),
            Either::Right(event) => on_data_key_events(state, event)
                .await
                .expect("error in on_data_key_events"),
        }
    });
    tokio::spawn(fold_fut);

    Ok(params)
}

//==================== process events for data ====================

// Process for DataConnection reacts to fold stream of DataConnection events and UserInput streams.
// This struct shows the previous state.
#[derive(Clone, Default)]
struct DataConnectionState(
    HashMap<DataConnectionId, (Option<CreatedResponse>, Option<SocketInfo<PhantomId>>)>,
);

// This struct has only setter and getter.
impl DataConnectionState {
    pub fn data_connection_id_iter(
        &self,
    ) -> Keys<DataConnectionId, (Option<CreatedResponse>, Option<SocketInfo<PhantomId>>)> {
        self.0.keys()
    }

    pub fn insert_data_connection_id(
        &mut self,
        data_connection_id: DataConnectionId,
        value: (Option<CreatedResponse>, Option<SocketInfo<PhantomId>>),
    ) {
        let _ = self.0.insert(data_connection_id, value);
    }

    pub fn remove_data_connection_id(&mut self, data_connection_id: &DataConnectionId) {
        let _ = self.0.remove(&data_connection_id);
    }

    pub fn contains(&self, data_connection_id: &DataConnectionId) -> bool {
        self.0.contains_key(data_connection_id)
    }

    pub fn get(
        &self,
        data_connection_id: &DataConnectionId,
    ) -> Option<&(Option<CreatedResponse>, Option<SocketInfo<PhantomId>>)> {
        self.0.get(data_connection_id)
    }
}

// This function process DataConnection events
async fn on_data_api_events(
    state: DataConnectionState,
    event: data::DataConnectionEventEnum,
) -> Result<DataConnectionState, error::Error> {
    //FIXME not enough
    let status = match event {
        data::DataConnectionEventEnum::OPEN(date_connection_id) => {
            info!("{:?} is opend", date_connection_id);
            let value = state.get(&date_connection_id).expect("socket info not set");
            info!("it's source port is {:?}", value.0);
            info!("it's destination socket is {:?}", value.1);
            Ok(state)
        }
        data::DataConnectionEventEnum::CLOSE(date_connection_id) => {
            // FIXME: notify the close event to user
            info!("{:?} is closed", date_connection_id);
            Ok(state)
        }
        _ => Ok(state),
    };
    print_commands();
    status
}

// This function process Keyboard Inputs
async fn on_data_key_events(
    mut state: DataConnectionState,
    ControlMessage(message): ControlMessage,
) -> Result<DataConnectionState, error::Error> {
    //FIXME not enough
    let status = match message.as_str() {
        "status" => {
            // prinnts all DataConnection status
            for data_connection_id in state.data_connection_id_iter() {
                let status = data::status(&data_connection_id).await?;
                info!(
                    "##################\nDataConnection {:?} is now {:?}",
                    data_connection_id, status
                );
                let value = state.get(&data_connection_id).expect("socket info not set");
                info!("it's source port is {:?}", value.0);
                info!("it's destination socket is {:?}", value.1);
            }
            Ok(state)
        }
        message if message.starts_with("disconnect ") => {
            // close P2P link
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(data_connection_id) = args.next() {
                let data_connection_id = DataConnectionId::new(data_connection_id);
                if state.contains(&data_connection_id) {
                    let _ = data::disconnect(&data_connection_id).await?;
                    state.remove_data_connection_id(&data_connection_id);
                } else {
                    warn!("{:?} is not a valid Data Connection Id", data_connection_id);
                }
            } else {
                warn!("input \"disconnect DATA_CONNECTION_ID\"");
            }
            Ok(state)
        }
        "disconnect_all" => {
            for data_connection_id in state.clone().data_connection_id_iter() {
                let _ = data::disconnect(data_connection_id).await?;
                state.remove_data_connection_id(data_connection_id);
            }

            Ok(state)
        }
        _ => Ok(state),
    };
    print_commands();
    status
}

//==================== helper ====================

fn print_commands() {
    let message = "exit\n\
    connect PEER_ID\n\
    status\n\
    disconnect MEDIA_CONNECTION_ID";

    println!(
        "====================\ninput following commands\n{}\n====================",
        message
    );
}
