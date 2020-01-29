mod terminal;

use futures::channel::mpsc;
use futures::prelude::*;
use log::info;

use peer::PeerEventEnum;
use webrtc_gateway_controller::prelude::*;
use webrtc_gateway_controller::*;

#[derive(Debug)]
enum EventEnum {
    Api(PeerEventEnum),
    Keyboard(String),
}

#[derive(Debug)]
struct PeerFoldParameters((Option<PeerInfo>, Vec<mpsc::Sender<String>>));

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let domain = ::std::env::var("DOMAIN").expect("DOMAIN is not set in environment variables");
    let peer_id = ::std::env::var("PEER_ID").expect("PEER_ID is not set in environment variables");
    let peer_id = PeerId::new(peer_id);
    let base_url = ::std::env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());
    webrtc_gateway_controller::initialize(base_url);

    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    let peer_info = create_peer_future.await.expect("create peer failed");
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(0);
    let event_future = peer::listen_events(&peer_info, peer_event_notifier);

    let (key_notifier, key_observer) = mpsc::channel::<String>(0);
    let key_fut = terminal::read(key_notifier);
    let peer_event_stream = futures::stream::select(
        key_observer.map(|e| EventEnum::Keyboard(e)),
        peer_event_observer.map(|e| EventEnum::Api(e)),
    );
    let fold_fut =
        peer_event_stream.fold(PeerFoldParameters((None, vec![])), |sum, acc| async move {
            let sum = on_peer_events(sum, acc).await.expect("error");
            sum
        });
    let (_, _, _) = futures::future::join3(fold_fut, event_future, key_fut).await;
}

async fn on_peer_events(
    status: PeerFoldParameters,
    event: EventEnum,
) -> Result<PeerFoldParameters, error::Error> {
    match event {
        EventEnum::Api(api_events) => on_peer_api_events(status, api_events).await,
        EventEnum::Keyboard(key_events) => on_peer_key_events(status, key_events).await,
    }
}

async fn on_peer_api_events(
    PeerFoldParameters((peer_info, vec)): PeerFoldParameters,
    event: PeerEventEnum,
) -> Result<PeerFoldParameters, error::Error> {
    match event {
        PeerEventEnum::OPEN(event) => {
            info!("Peer {:?} is created", event.params);
            Ok(PeerFoldParameters((Some(event.params), vec)))
        }
        PeerEventEnum::CLOSE(event) => {
            info!("Peer {:?} is closed", event.params);
            Ok(PeerFoldParameters((None, vec)))
        }
        _ => Ok(PeerFoldParameters((peer_info, vec))),
    }
}

async fn on_peer_key_events(
    PeerFoldParameters((peer_info, vec)): PeerFoldParameters,
    message: String,
) -> Result<PeerFoldParameters, error::Error> {
    match message.as_str() {
        "exit" => {
            if let Some(peer_info) = peer_info {
                let _unit = peer::delete(&peer_info).await?;
            }
            Ok(PeerFoldParameters((None, vec)))
        }
        "status" => {
            if let Some(ref peer_info) = peer_info {
                let status = peer::status(peer_info).await?;
                info!("Peer {:?} is now {:?}", peer_info, status);
            }
            Ok(PeerFoldParameters((peer_info, vec)))
        }
        _ => Ok(PeerFoldParameters((peer_info, vec))),
    }
}
