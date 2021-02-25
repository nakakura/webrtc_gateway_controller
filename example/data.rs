mod terminal;

use std::env;
use std::fs;
use std::io::{BufReader, Read};
use std::sync::Mutex;

use futures::channel::mpsc;
use futures::prelude::*;
use futures::*;
use log::{info, warn};
use once_cell::sync::OnceCell;
use serde_derive::Deserialize;
use skyway_webrtc_gateway_api::data::*;
use skyway_webrtc_gateway_api::peer::*;
use skyway_webrtc_gateway_api::prelude::*;
use skyway_webrtc_gateway_api::*;

// Config of media used in call and answer.
static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

//==================== for parsing data.toml ====================

// It shows config toml formats
#[derive(Debug, Deserialize)]
struct Config {
    data: Vec<DataConfig>,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct DataConfig {
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

#[tokio::main]
async fn main() {
    // set log level
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Initialize values for create peer
    // SkyWay Service API Key
    let api_key = env::var("API_KEY").expect("API_KEY is not set in environment variables");
    // a domain already registered with skyway
    let domain = env::var("DOMAIN").unwrap_or("localhost".into());
    // Identifier of your peer
    let peer_id = env::var("PEER_ID").unwrap_or("peer_id".into());
    let peer_id = PeerId::new(peer_id);
    // URL to access your WebRTC GW.
    let base_url = env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());

    // read config file
    let data_config = read_config("example/data.toml");
    CONFIG.set(Mutex::new(data_config)).unwrap();

    // initialize crate
    skyway_webrtc_gateway_api::initialize(base_url);

    // call create peer api
    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    // When create api is called, the WebRTC GW first creates a PeerObject internally.
    // Next, it start registering the PeerObject with the SkyWay server.
    // When WebRTC GW starts the sequence, this crate returns Ok(PeerInfo).
    // The PeerInfo contains the Peer ID and the token needed to control the PeerObject.
    let peer_info = create_peer_future.await.expect("create peer failed");
    info!("peer_info is {:?}", peer_info);

    // The WebRTC GW interacts with SkyWay and notifies the end user of the result as an event.
    // Generate a future for event monitoring here.
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(10);
    let event_future = peer::listen_events(peer_info.clone(), peer_event_notifier);
    tokio::spawn(event_future);

    // Listen Keyboard Inputs
    let (keyboard_notifier, keyboard_observer) = tokio::sync::mpsc::channel::<String>(10);
    let key_observe_fut = terminal::read_stdin(keyboard_notifier);
    tokio::spawn(key_observe_fut);

    // Routes Keyboard Events
    let key_events_fut = on_keyboard_events(peer_info.clone(), keyboard_observer);

    // Routes Peer Events
    let peer_events_fut = on_peer_events(peer_info, peer_event_observer);

    // run futures
    join!(key_events_fut, peer_events_fut);
}

async fn on_peer_events(peer_info: PeerInfo, mut observer: mpsc::Receiver<PeerEventEnum>) {
    while let Some(result) = observer.next().await {
        match result {
            PeerEventEnum::OPEN(open) => {
                info!(
                    "Peer({}) is created. Now you can CALL/CONNECT.\n{:?}",
                    peer_info.peer_id.as_str(),
                    open
                );
            }
            PeerEventEnum::CONNECTION(connection) => {
                info!(
                    "Peer({}) received connection as {:?}",
                    peer_info.peer_id.as_str(),
                    connection
                );
                let config = &mut *CONFIG.get().unwrap().lock().unwrap();
                let val = config.data.pop().unwrap();
                tokio::spawn(redirect(val, connection));
            }
            PeerEventEnum::CLOSE(_close) => {
                info!("Peer({}) is deleted", peer_info.peer_id.as_str());
                break;
            }
            _ => {
                info!(
                    "Peer({}) notifies an Event \n{:?}",
                    peer_info.peer_id.as_str(),
                    result
                );
            }
        }
    }
}

async fn on_keyboard_events(
    peer_info: PeerInfo,
    mut observer: tokio::sync::mpsc::Receiver<String>,
) {
    println!(
        r#"print COMMAND
    exit
    status
    connect TARGET_ID
    "#
    );
    while let Some(message) = observer.recv().await {
        match message.as_str() {
            "exit" => {
                info!("start closing Peer({})", peer_info.peer_id.as_str());
                // when a PeerObject is closed, MediaConnections associated with it will be automatically closed.
                // So it's not necessary to close MediaConnections here.
                let _ = peer::delete(&peer_info).await;
                break;
            }
            "status" => {
                let peer_status = peer::status(&peer_info).await;
                info!(
                    "Peer({})'s status is \n{:?}",
                    peer_info.peer_id.as_str(),
                    peer_status
                );
            }
            x if x.starts_with("connect") => {
                let params: Vec<&str> = x.split(' ').collect();
                if params.len() < 2 {
                    warn!("input \"connect TARGET_ID\"");
                    continue;
                }

                let config = &mut *CONFIG.get().unwrap().lock().unwrap();
                let val = config.data.pop().unwrap();
                tokio::spawn(connection(peer_info.clone(), val, PeerId(params[1].into())));
            }
            _ => {
                info!("please type valid commands.");
            }
        }

        println!(
            r#"print COMMAND
    exit
    status
    connect TARGET_ID
    "#
        );
    }
}

// establish a connection to neighbour
async fn connection(
    peer_info: PeerInfo,
    data_config: DataConfig,
    target_id: PeerId,
) -> Result<(), error::Error> {
    // Open data socket to feed data
    let data_socket = data::open_data_socket().await?;
    // Data received from DataConnection will be redirected according to this information.
    let redirect_info =
        SocketInfo::<PhantomId>::try_create(None, &data_config.ip, data_config.port)
            .expect("invalid data port");
    // set up query and access to connect API.
    let query = data::ConnectQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        options: None,
        target_id: target_id,
        params: Some(DataIdWrapper {
            data_id: data_socket.get_id().clone().unwrap(),
        }),
        redirect_params: Some(redirect_info.clone()),
    };
    let data_connection_id = data::connect(query).await?;

    // listen DataConnection events and send them with this channel
    let (event_notifier, mut event_observer) = mpsc::channel::<data::DataConnectionEventEnum>(0);
    let event_listen_fut = data::listen_events(data_connection_id.clone(), event_notifier);
    tokio::spawn(event_listen_fut);

    while let Some(data_event) = event_observer.next().await {
        match data_event {
            DataConnectionEventEnum::OPEN(open) => {
                info!(
                    "DataConnection({}) has opened {:?}",
                    data_connection_id.as_str(),
                    open
                );
                info!("Now you can send data with {:?}", data_socket);
            }
            DataConnectionEventEnum::CLOSE(close) => {
                info!(
                    "DataConnection({}) has closed {:?}",
                    data_connection_id.as_str(),
                    close
                );
                break;
            }
            DataConnectionEventEnum::ERROR(error) => {
                info!(
                    "DataConnection({}) gets error {:?}",
                    data_connection_id.as_str(),
                    error
                );
            }
            _ => {}
        }
    }

    Ok(())
}

// set redirect info to DataConnection
async fn redirect(
    data_config: DataConfig,
    connection_event: PeerConnectionEvent,
) -> Result<(), error::Error> {
    // Open data socket to feed data
    let data_socket = data::open_data_socket().await?;
    // Data received from DataConnection will be redirected according to this information.
    let redirect_info =
        SocketInfo::<PhantomId>::try_create(None, &data_config.ip, data_config.port)
            .expect("invalid data port");

    let redirect_params = data::RedirectDataParams {
        feed_params: Some(DataIdWrapper {
            data_id: data_socket.get_id().clone().unwrap(),
        }),
        redirect_params: Some(redirect_info.clone()),
    };
    let data_connection_id = connection_event.data_params.data_connection_id.clone();

    let _ = data::redirect(&data_connection_id, &redirect_params)
        .await
        .expect("redirect data failed");

    // listen DataConnection events and send them with this channel
    let (event_notifier, mut event_observer) = mpsc::channel::<data::DataConnectionEventEnum>(0);
    let event_listen_fut = data::listen_events(data_connection_id.clone(), event_notifier);
    tokio::spawn(event_listen_fut);

    while let Some(data_event) = event_observer.next().await {
        match data_event {
            DataConnectionEventEnum::OPEN(open) => {
                info!(
                    "DataConnection({}) has opened {:?}",
                    data_connection_id.as_str(),
                    open
                );
                info!("Now you can send data with {:?}", data_socket);
            }
            DataConnectionEventEnum::CLOSE(close) => {
                info!(
                    "DataConnection({}) has closed {:?}",
                    data_connection_id.as_str(),
                    close
                );
                break;
            }
            DataConnectionEventEnum::ERROR(error) => {
                info!(
                    "DataConnection({}) gets error {:?}",
                    data_connection_id.as_str(),
                    error
                );
            }
            _ => {}
        }
    }

    Ok(())
}
