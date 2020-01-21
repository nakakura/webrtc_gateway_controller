mod terminal;

use std::fs;
use std::io::{BufReader, Read};

use either;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::prelude::*;
use futures::*;
use log::{info, warn};
use serde_derive::Deserialize;

use either::Either;
use peer::formats::PeerEventEnum;
use webrtc_gateway_controller::common::{DataConnectionId, PeerId, PeerInfo};
use webrtc_gateway_controller::data::formats::{DataIdWrapper, RedirectParams};
use webrtc_gateway_controller::*;

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

#[tokio::main]
async fn main() {
    env_logger::init();

    //load and set parameters
    let config = read_config("example/data.toml");
    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let domain = config.peer.domain;
    let peer_id = common::PeerId::new(config.peer.peer_id);
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
    let fold_fut = peer_event_stream.fold(
        // before receiving Peer::OPEN event, peer object might not be created.
        // So I set None for PeerInfo.
        PeerFoldState((None, config.redirects, vec![])),
        |sum, acc| async move {
            let sum = on_peer_events(sum, acc).await.expect("error");
            sum
        },
    );

    //execute all the futures
    let (fold_fut_result, event_fut_result, key_fut_reult) =
        futures::future::join3(fold_fut, event_future, key_fut).await;
    info!("All the futures are finished. They stopped with these status\nfold: {:?}\nevent: {:?}\nkey:{:?}", fold_fut_result, event_fut_result, key_fut_reult);
}

// This function is called in a fold of User Input and Peer Event streams.
// It parse the stream and process them with its internal functions
async fn on_peer_events(
    status: PeerFoldState,
    event: Either<PeerEventEnum, String>,
) -> Result<PeerFoldState, error::ErrorEnum> {
    match event {
        Either::Left(api_events) => on_peer_api_events(status, api_events).await,
        Either::Right(key_events) => on_peer_key_events(status, key_events).await,
    }
}

// This function process events from Peer Object
async fn on_peer_api_events(
    params: PeerFoldState,
    event: PeerEventEnum,
) -> Result<PeerFoldState, error::ErrorEnum> {
    match event {
        PeerEventEnum::OPEN(event) => {
            // PeerObject notify that it has been successfully created.
            // Hold PeerInfo for further process.
            info!("Peer {:?} is created", event.params);
            let params = params.set_peer_info(Some(event.params));
            Ok(params)
        }
        PeerEventEnum::CLOSE(event) => {
            // PeerObject notify that it has already been deleted.
            // Erase old PeerInfo.
            info!("Peer {:?} is closed", event.params);
            let params = params.set_peer_info(None);
            Ok(params)
        }
        PeerEventEnum::CONNECTION(event) => {
            // In this timing, DataChannel itself has already been established.
            // To send and recv data, call redirect API.
            println!("event {:?}", event);
            redirect(params, event.data_params.data_connection_id).await
        }
        _ => Ok(params),
    }
}

// This function works according to User Keyboard Input
async fn on_peer_key_events(
    mut params: PeerFoldState,
    message: String,
) -> Result<PeerFoldState, error::ErrorEnum> {
    match message.as_str() {
        "exit" => {
            // FIXME: not yet
            // When an user wants to close this program, it needs to close P2P links and delete Peer Object.
            // Content Socket will be automaticall released, so it is not necessary to release them manually.
            // https://github.com/skyway/skyway-webrtc-gateway/blob/master/docs/release_process.md
            if let Some(peer_info) = params.peer_info() {
                let _unit = peer::delete(&peer_info).await?;
            }
            let params = params.set_peer_info(None);
            Ok(params)
        }
        "status" => {
            // Show status of PeerObject and DataConnection
            if let Some(ref peer_info) = params.peer_info() {
                let status = peer::status(peer_info).await?;
                info!("Peer {:?} is now {:?}", peer_info, status);
            }
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier.send(ControlMessage(String::from("status"))).await;
            }
            Ok(params)
        }
        "disconnect" => {
            // Disconnect P2P link
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier
                    .send(ControlMessage(String::from("disconnect")))
                    .await;
            }
            Ok(params)
        }
        message if message.starts_with("connect") => {
            // Establish P2P datachannel to an neighbour.
            // This function expects "connect TARGET_ID".
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(target_id) = args.next() {
                let target_id = PeerId::new(target_id);
                Ok(connect(params, target_id).await.expect("error at line 163"))
            } else {
                warn!("input \"connect TARGET_PEER_ID\"");
                Ok(params)
            }
        }
        _ => Ok(params),
    }
}

// Process for DataConnection reacts to fold stream of DataConnection events and UserInput streams.
// This struct shows the previous state.
struct DataConnectionState((Option<DataConnectionId>));

// This struct has only setter and getter.
impl DataConnectionState {
    pub fn data_connection_id(&self) -> Option<DataConnectionId> {
        self.0.clone()
    }
}

// start establishing DataConnection to an neighbour
async fn connect(
    mut params: PeerFoldState,
    target_id: PeerId,
) -> Result<PeerFoldState, error::ErrorEnum> {
    // Notify which peer object needs to establish P2P link to WebRTC Gateway.
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");

    // Data received from this content socket will be redirected to neighbour with DataConnection.
    let result = data::open_source_socket().await?;
    let data_id = result.data_id;

    // Data received from DataConnection will be redirected according to this information.
    let redirect_info = params.socket_config().map(|socket_config| {
        RedirectParams {
            ip_v4: Some(socket_config.ip),
            ip_v6: None, //FIXME
            port: socket_config.port,
        }
    });

    // set up query and access to connect API.
    let query = data::formats::CreateDataConnectionQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        options: None,
        target_id: target_id,
        params: Some(DataIdWrapper { data_id: data_id }),
        redirect_params: redirect_info,
    };
    let data_connection_id = data::connect(query).await?;

    // Notify keyboard inputs to the sub-task with this channel
    let (mut control_message_notifier, control_message_observer) =
        mpsc::channel::<ControlMessage>(0);
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
    let fold_fut = stream.fold(
        DataConnectionState((Some(data_connection_id))),
        |sum, acc| async move { on_data_events(sum, acc).await.expect("error") },
    );
    tokio::spawn(fold_fut);

    Ok(params)
}

// set input and output port to DataConnection which has already been established.
async fn redirect(
    mut params: PeerFoldState,
    data_connection_id: DataConnectionId,
) -> Result<PeerFoldState, error::ErrorEnum> {
    // to show which peer's DataConnection to WebRTC Gateway
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");
    // Data received from this content socket will be redirected to neighbour with DataConnection.
    let result = data::open_source_socket().await?;
    let data_id = result.data_id;
    // Data received from DataConnection will be redirected according to this information.
    // If there is no redirect infor in config.toml, redirect info will be None.
    // In this case, the data channel is virtually sendonly.
    let redirect_info = params.socket_config().map(|socket_config| {
        RedirectParams {
            ip_v4: Some(socket_config.ip),
            ip_v6: None, //FIXME
            port: socket_config.port,
        }
    });
    let redirect_params = data::formats::RedirectDataParams {
        feed_params: Some(DataIdWrapper { data_id: data_id }),
        redirect_params: redirect_info,
    };
    let result = data::redirect(&data_connection_id, &redirect_params).await;

    // Notify keyboard inputs to the sub-task with this channel
    let (mut control_message_notifier, control_message_observer) =
        mpsc::channel::<ControlMessage>(0);
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
    let fold_fut = stream.fold(
        DataConnectionState((Some(data_connection_id))),
        |sum, acc| async move {
            let result = on_data_events(sum, acc).await.expect("error");
            result
        },
    );
    tokio::spawn(fold_fut);

    Ok(params)
}

// This function is called in a fold of User Input and DataConnection Event streams.
// It parse the stream and process them with its internal functions
async fn on_data_events(
    sum: DataConnectionState,
    event: Either<data::DataConnectionEventEnum, ControlMessage>,
) -> Result<DataConnectionState, error::ErrorEnum> {
    match event {
        Either::Left(event) => on_data_api_events(sum, event).await,
        Either::Right(event) => on_data_key_events(sum, event).await,
    }
}

// This function process DataConnection events
async fn on_data_api_events(
    state: DataConnectionState,
    event: data::DataConnectionEventEnum,
) -> Result<DataConnectionState, error::ErrorEnum> {
    //FIXME not enough
    match event {
        data::DataConnectionEventEnum::OPEN(date_connection_id) => {
            // FIXME: notify the open event to user
            info!("{:?} is opend", date_connection_id);
            Ok(state)
        }
        data::DataConnectionEventEnum::CLOSE(date_connection_id) => {
            // FIXME: notify the close event to user
            info!("{:?} is closed", date_connection_id);
            Ok(state)
        }
        _ => Ok(state),
    }
}

// This function process Keyboard Inputs
async fn on_data_key_events(
    state: DataConnectionState,
    ControlMessage(message): ControlMessage,
) -> Result<DataConnectionState, error::ErrorEnum> {
    //FIXME not enough
    match message.as_str() {
        "status" => {
            // prinnts DataConnection status
            let data_connection_id = state.data_connection_id();
            if let Some(data_connection_id) = data_connection_id {
                let status = data::status(data_connection_id.clone()).await?;
                info!(
                    "DataConnection {:?} is now {:?}",
                    data_connection_id, status
                );
            }
            Ok(state)
        }
        "disconnect" => {
            // close P2P link
            let data_connection_id = state.data_connection_id();
            if let Some(data_connection_id) = data_connection_id {
                let result = data::disconnect(data_connection_id).await?;
            }
            Ok(DataConnectionState((None)))
        }
        _ => Ok(state),
    }
}
