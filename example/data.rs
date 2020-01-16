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
use webrtc_gateway_controller::data::formats::DataIdWrapper;
use webrtc_gateway_controller::*;

#[derive(Debug)]
struct ControlMessage(String);

#[derive(Debug)]
struct PeerFoldParameters(
    (
        Option<PeerInfo>,
        Vec<SocketConfig>,
        Vec<mpsc::Sender<ControlMessage>>,
    ),
);

impl PeerFoldParameters {
    pub fn peer_info(&self) -> &Option<PeerInfo> {
        &(self.0).0
    }

    pub fn set_peer_info(self, peer_info: Option<PeerInfo>) -> Self {
        let PeerFoldParameters((_, redirects, sender)) = self;
        PeerFoldParameters((peer_info, redirects, sender))
    }

    pub fn control_message_notifier(&mut self) -> &mut Vec<mpsc::Sender<ControlMessage>> {
        &mut (self.0).2
    }

    pub fn set_control_message_notifier(&mut self, tx: mpsc::Sender<ControlMessage>) {
        (&mut (self.0).2).push(tx);
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    peer: PeerConfig,
    gateway: SocketConfig,
    redirects: Vec<SocketConfig>,
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    peer_id: String,
    domain: String,
}

#[derive(Debug, Deserialize)]
struct SocketConfig {
    ip: String,
    port: u16,
}

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
    let config = read_config("example/data.toml");

    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let domain = config.peer.domain;
    let peer_id = common::PeerId::new(config.peer.peer_id);
    let base_url: String = format!("http://{}:{}", config.gateway.ip, config.gateway.port);
    webrtc_gateway_controller::initialize(base_url);

    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    let peer_info = create_peer_future.await.expect("create peer failed");
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(0);
    let event_future = peer::listen_events(&peer_info, peer_event_notifier);

    let (key_notifier, key_observer) = mpsc::channel::<String>(0);
    let key_fut = terminal::read(key_notifier);
    let peer_event_stream = futures::stream::select(
        peer_event_observer.map(|e| Either::Left(e)),
        key_observer.map(|e| Either::Right(e)),
    );
    let fold_fut = peer_event_stream.fold(
        PeerFoldParameters((None, config.redirects, vec![])),
        |sum, acc| async move {
            let sum = on_peer_events(sum, acc).await.expect("error");
            sum
        },
    );
    let (_, _, _) = futures::future::join3(fold_fut, event_future, key_fut).await;
}

async fn on_peer_events(
    status: PeerFoldParameters,
    event: Either<PeerEventEnum, String>,
) -> Result<PeerFoldParameters, error::ErrorEnum> {
    match event {
        Either::Left(api_events) => on_peer_api_events(status, api_events).await,
        Either::Right(key_events) => on_peer_key_events(status, key_events).await,
    }
}

async fn on_peer_api_events(
    params: PeerFoldParameters,
    event: PeerEventEnum,
) -> Result<PeerFoldParameters, error::ErrorEnum> {
    match event {
        PeerEventEnum::OPEN(event) => {
            info!("Peer {:?} is created", event.params);
            let params = params.set_peer_info(Some(event.params));
            Ok(params)
        }
        PeerEventEnum::CLOSE(event) => {
            info!("Peer {:?} is closed", event.params);
            let params = params.set_peer_info(None);
            Ok(params)
        }
        _ => Ok(params),
    }
}

async fn on_peer_key_events(
    mut params: PeerFoldParameters,
    message: String,
) -> Result<PeerFoldParameters, error::ErrorEnum> {
    match message.as_str() {
        "exit" => {
            if let Some(peer_info) = params.peer_info() {
                let _unit = peer::delete(&peer_info).await?;
            }
            let params = params.set_peer_info(None);
            Ok(params)
        }
        "status" => {
            if let Some(ref peer_info) = params.peer_info() {
                let status = peer::status(peer_info).await?;
                info!("Peer {:?} is now {:?}", peer_info, status);
            }
            Ok(params)
        }
        "disconnect" => {
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier
                    .send(ControlMessage(String::from("disconnect")))
                    .await;
            }
            Ok(params)
        }
        message if message.starts_with("connect") => {
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(target_id) = args.next() {
                let target_id = PeerId::new(target_id);
                let result = connect(params, target_id).await.expect("error at line 163");
                Ok(result)
            } else {
                warn!("input \"connect TARGET_PEER_ID\"");
                Ok(params)
            }
        }
        _ => Ok(params),
    }
}

struct DataConnectionState((Option<DataConnectionId>));

impl DataConnectionState {
    pub fn data_connection_id(&self) -> Option<DataConnectionId> {
        self.0.clone()
    }
}

async fn connect(
    mut params: PeerFoldParameters,
    target_id: PeerId,
) -> Result<PeerFoldParameters, error::ErrorEnum> {
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");
    let result = data::open_source_socket().await?;
    let data_id = result.data_id;
    let query = data::formats::CreateDataConnectionQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        options: None,
        target_id: target_id,
        params: DataIdWrapper { data_id: data_id },
        redirect_params: None,
    };
    let data_connection_id = data::connect(query).await?;

    let (mut control_message_notifier, control_message_observer) =
        mpsc::channel::<ControlMessage>(0);
    let (dc_event_notifier, dc_event_observer) = mpsc::channel::<data::DataConnectionEventEnum>(0);
    let event_listen_fut = data::listen_events(data_connection_id.clone(), dc_event_notifier);
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

    tokio::spawn(event_listen_fut);
    tokio::spawn(fold_fut);
    params.set_control_message_notifier(control_message_notifier);
    Ok(params)
}

async fn on_data_events(
    sum: DataConnectionState,
    event: Either<data::DataConnectionEventEnum, ControlMessage>,
) -> Result<DataConnectionState, error::ErrorEnum> {
    match event {
        Either::Left(event) => on_data_api_events(sum, event).await,
        Either::Right(event) => on_data_key_events(sum, event).await,
    }
}

async fn on_data_api_events(
    state: DataConnectionState,
    event: data::DataConnectionEventEnum,
) -> Result<DataConnectionState, error::ErrorEnum> {
    //FIXME not enough
    match event {
        data::DataConnectionEventEnum::OPEN(date_connection_id) => {
            info!("{:?} is opend", date_connection_id);
            Ok(state)
        }
        data::DataConnectionEventEnum::CLOSE(date_connection_id) => {
            info!("{:?} is closed", date_connection_id);
            Ok(state)
        }
        _ => Ok(state),
    }
}

async fn on_data_key_events(
    state: DataConnectionState,
    ControlMessage(message): ControlMessage,
) -> Result<DataConnectionState, error::ErrorEnum> {
    match message.as_str() {
        //FIXME not enough
        "disconnect" => {
            let data_connection_id = state.data_connection_id();
            if let Some(data_connection_id) = data_connection_id {
                let result = data::disconnect(data_connection_id).await?;
            }
            Ok(DataConnectionState((None)))
        }
        _ => Ok(state),
    }
}
