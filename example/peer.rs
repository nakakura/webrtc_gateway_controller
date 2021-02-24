mod terminal;

use std::env;

use futures::channel::mpsc;
use futures::prelude::*;
use futures::*;
use log::info;

use peer::PeerEventEnum;
use skyway_webrtc_gateway_api::prelude::*;
use skyway_webrtc_gateway_api::*;

#[derive(Debug)]
struct PeerFoldParameters((Option<PeerInfo>, Vec<mpsc::Sender<String>>));

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
    while let Some(message) = observer.recv().await {
        match message.as_str() {
            "exit" => {
                info!("start closing Peer({})", peer_info.peer_id.as_str());
                let _ = peer::delete(&peer_info).await;
                break;
            }
            "status" => {
                let status = peer::status(&peer_info).await;
                info!(
                    "Peer({})'s status is \n{:?}",
                    peer_info.peer_id.as_str(),
                    status
                );
            }
            _ => {
                info!("please type valid commands. \nstatus\nexit")
            }
        }
    }
}
