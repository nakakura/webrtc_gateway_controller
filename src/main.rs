use futures::channel::mpsc::*;
use futures::*;
use log::{debug, warn};

use webrtc_gateway_controller::data::formats::CreatedResponse;
use webrtc_gateway_controller::peer::formats::*;
use webrtc_gateway_controller::*;

#[cfg(not(test))]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    env_logger::init();

    // setup callback functions for PeerObject
    // FIRES when GET /peer/{peer_id}/events returns OPEN event
    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    // FIRES when GET /peer/{peer_id}/events returns CALL event
    let (on_call_tx, on_call_rx) = channel::<peer::formats::PeerCallEvent>(0);
    // FIRES when GET /peer/{peer_id}/events returns CONNECT event
    let (on_connect_tx, on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    // FIRES when GET /peer/{peer_id}/events returns CLOSE event
    let (on_close_tx, on_close_rx) = channel::<peer::formats::PeerCloseEvent>(0);
    // FIRES when GET /peer/{peer_id}/events returns ERROR event
    let (on_error_tx, on_error_rx) = channel::<peer::formats::PeerErrorEvent>(0);

    // On Open Event is used in some flows, so redirect it
    // For data connection
    let (sub_on_open_tx_1, sub_on_open_rx_1) = channel::<peer::formats::PeerOpenEvent>(0);
    // For media connection
    let (sub_on_open_tx_2, _sub_on_open_rx_2) = channel::<peer::formats::PeerOpenEvent>(0);
    let tx_array = vec![sub_on_open_tx_1, sub_on_open_tx_2];
    // On Open Event Redirect
    let on_open_future = on_open_rx.fold(tx_array, |mut sum, item| {
        async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        }
    });

    let on_call_future = on_call_rx.for_each(on_peer_call);
    let on_close_future = on_close_rx.for_each(on_peer_close);
    let on_error_future = on_error_rx.for_each(on_peer_error);
    // Start subscribing each events
    tokio::spawn(on_open_future.map(|_| ()));
    tokio::spawn(on_call_future);
    tokio::spawn(on_close_future);
    tokio::spawn(on_error_future);

    if *CONNECT_FLAG {
        let connect_future = sub_on_open_rx_1.for_each(|event: PeerOpenEvent| {
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

// Peer Event Callbacks
// FIXME
#[allow(dead_code)]
fn on_peer_call(event: peer::formats::PeerCallEvent) -> impl Future<Output = ()> {
    debug!("on_peer_call: {:?}", event);
    future::ready(())
}

// FIXME
#[allow(dead_code)]
fn on_peer_connect(event: peer::formats::PeerConnectionEvent) -> impl Future<Output = ()> {
    debug!("on_peer_connect: {:?}", event);
    future::ready(())
}

// FIXME
#[allow(dead_code)]
fn on_peer_close(event: peer::formats::PeerCloseEvent) -> impl Future<Output = ()> {
    debug!("on_peer_close: {:?}", event);
    future::ready(())
}

// FIXME
#[allow(dead_code)]
fn on_peer_error(event: peer::formats::PeerErrorEvent) -> impl Future<Output = ()> {
    warn!("on_peer_error: {:?}", event);
    future::ready(())
}

// DataConnection Event Callbacks
// FIXME
#[allow(dead_code)]
fn on_data_open(data_connection_id: String) -> impl Future<Output = ()> {
    debug!("on_data_open: {:?}", data_connection_id);
    future::ready(())
}

// FIXME
#[allow(dead_code)]
fn on_data_close(data_connection_id: String) -> impl Future<Output = ()> {
    debug!("on_data_close: {:?}", data_connection_id);
    future::ready(())
}

// FIXME
#[allow(dead_code)]
fn on_data_error(
    (data_connection_id, error_message): (String, String),
) -> impl Future<Output = ()> {
    warn!(
        "on_data_error: {:?} {:?}",
        data_connection_id, error_message
    );
    future::ready(())
}
