use futures::channel::mpsc::*;
use futures::*;
use log::{debug, warn};

use webrtc_gateway_controller::data::formats::CreatedResponse;
use webrtc_gateway_controller::peer::formats::*;
use webrtc_gateway_controller::*;

struct PeerEventObservers {
    on_open_tx: Vec<Sender<PeerOpenEvent>>,
    on_call_tx: Vec<Sender<PeerCallEvent>>,
    on_connect_tx: Vec<Sender<PeerConnectionEvent>>,
    on_close_tx: Vec<Sender<PeerCloseEvent>>,
    on_error_tx: Vec<Sender<PeerErrorEvent>>,
}

// create peer object and notify each event to observers
async fn peer_runner(observers: PeerEventObservers) {
    // FIRES when GET /peer/{peer_id}/events returns OPEN event
    // redirect the event message to observers
    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    let on_open_notify_future = on_open_rx.fold(observers.on_open_tx, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });
    tokio::spawn(on_open_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CALL event
    let (on_call_tx, on_call_rx) = channel::<peer::formats::PeerCallEvent>(0);
    let on_call_notify_future = on_call_rx.fold(observers.on_call_tx, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });
    tokio::spawn(on_call_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CONNECT event
    let (on_connect_tx, on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    let on_connect_notify_future = on_connect_rx.fold(observers.on_connect_tx, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });
    tokio::spawn(on_connect_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CLOSE event
    let (on_close_tx, on_close_rx) = channel::<peer::formats::PeerCloseEvent>(0);
    let on_close_notify_future = on_close_rx.fold(observers.on_close_tx, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });
    tokio::spawn(on_close_notify_future.map(|_| ()));
    // FIRES when GET /peer/{peer_id}/events returns ERROR event
    let (on_error_tx, on_error_rx) = channel::<peer::formats::PeerErrorEvent>(0);
    let on_error_notify_future = on_error_rx.fold(observers.on_error_tx, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });
    tokio::spawn(on_error_notify_future.map(|_| ()));

    let _ = peer::peer_create_and_listen_events(
        &*BASE_URL,
        &*PEER_ID,
        true,
        Some(on_open_tx),
        Some(on_call_tx),
        Some(on_connect_tx),
        Some(on_close_tx),
        Some(on_error_tx),
    )
    .await;
}

#[cfg(not(test))]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    let (on_connect_tx, on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    let observers = PeerEventObservers {
        on_open_tx: vec![on_open_tx],
        on_call_tx: vec![],
        on_connect_tx: vec![on_connect_tx],
        on_close_tx: vec![],
        on_error_tx: vec![],
    };

    if *CONNECT_FLAG {
        let connect_future = on_open_rx.for_each(|event: PeerOpenEvent| {
            async move {
                let (on_connection_open_tx, on_connection_open_rx) = channel::<String>(0);
                let on_connection_open_rx = on_connection_open_rx.for_each(|message| {
                    async move {
                        println!("on connect {:?}", message);
                        ()
                    }
                });
                tokio::spawn(on_connection_open_rx);

                let _ = data::connect_flow(
                    &*BASE_URL,
                    event.params,
                    Some(on_connection_open_tx),
                    None,
                    None,
                )
                .await;
            }
        });
        tokio::spawn(connect_future);
    } else {
        let on_connect_future = on_connect_rx.for_each(|event: PeerConnectionEvent| {
            async move {
                let (on_connection_open_tx, on_connection_open_rx) = channel::<String>(0);
                let on_connection_open_rx = on_connection_open_rx.for_each(|message| {
                    async move {
                        println!("on connect {:?}", message);
                        ()
                    }
                });
                tokio::spawn(on_connection_open_rx);
                let _ = data::redirect_flow(
                    &*BASE_URL,
                    &event.data_params.data_connection_id,
                    Some(on_connection_open_tx),
                    None,
                    None,
                )
                .await;
            }
        });
        tokio::spawn(on_connect_future);
    }

    let _ = peer_runner(observers).await;
}
