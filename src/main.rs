#![warn(rust_2018_idioms)]

use futures::channel::mpsc::*;
use futures::*;
use log::{info, warn};

use webrtc_gateway_controller::data::formats::{
    OnCloseTxParameters, OnErrorTxParameters, OnOpenTxParameters,
};
use webrtc_gateway_controller::peer::formats::*;
use webrtc_gateway_controller::*;

struct PeerEventObservers {
    on_open_tx: Vec<Sender<PeerOpenEvent>>,
    on_call_tx: Vec<Sender<PeerCallEvent>>,
    on_connect_tx: Vec<Sender<PeerConnectionEvent>>,
    on_close_tx: Vec<Sender<PeerCloseEvent>>,
    on_error_tx: Vec<Sender<PeerErrorEvent>>,
}

// FIXME: write test
// create peer object and notify each event to observers
async fn peer_runner(observers: PeerEventObservers) {
    // FIRES when GET /peer/{peer_id}/events returns OPEN event
    // redirect the event message to observers
    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    let on_open_notify_future = on_open_rx.fold(observers.on_open_tx, |mut sum, item| async move {
        for tx in sum.iter_mut() {
            let _ = tx.send(item.clone()).await;
        }
        sum
    });
    tokio::spawn(on_open_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CALL event
    let (on_call_tx, on_call_rx) = channel::<peer::formats::PeerCallEvent>(0);
    let on_call_notify_future = on_call_rx.fold(observers.on_call_tx, |mut sum, item| async move {
        for tx in sum.iter_mut() {
            let _ = tx.send(item.clone()).await;
        }
        sum
    });
    tokio::spawn(on_call_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CONNECT event
    let (on_connect_tx, on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    let on_connect_notify_future =
        on_connect_rx.fold(observers.on_connect_tx, |mut sum, item| async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        });
    tokio::spawn(on_connect_notify_future.map(|_| ()));

    // FIRES when GET /peer/{peer_id}/events returns CLOSE event
    let (on_close_tx, on_close_rx) = channel::<peer::formats::PeerCloseEvent>(0);
    let on_close_notify_future =
        on_close_rx.fold(observers.on_close_tx, |mut sum, item| async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
        });
    tokio::spawn(on_close_notify_future.map(|_| ()));
    // FIRES when GET /peer/{peer_id}/events returns ERROR event
    let (on_error_tx, on_error_rx) = channel::<peer::formats::PeerErrorEvent>(0);
    let on_error_notify_future =
        on_error_rx.fold(observers.on_error_tx, |mut sum, item| async move {
            for tx in sum.iter_mut() {
                let _ = tx.send(item.clone()).await;
            }
            sum
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

// FIXME: write test
// PeerObject is ready for Call, Connect
async fn peer_on_open(event: PeerOpenEvent) {
    let peer_info = event.params;
    info!(
        "Peer {} is created. Its token is {}",
        peer_info.peer_id.as_str(),
        peer_info.token.as_str()
    );
    // if use set CONNECT_FLAG, connect to another peer
    if *CONNECT_FLAG {
        let (data_connection_on_open_tx, data_connection_on_open_rx) =
            channel::<data::formats::OnOpenTxParameters>(0);
        let data_connection_on_open_rx =
            data_connection_on_open_rx.for_each(data_connection_on_open);
        tokio::spawn(data_connection_on_open_rx);

        let (data_connection_on_close_tx, data_connection_on_close_rx) =
            channel::<data::formats::OnCloseTxParameters>(0);
        let data_connection_on_close_rx =
            data_connection_on_close_rx.for_each(data_connection_on_close);
        tokio::spawn(data_connection_on_close_rx);

        let (data_connection_on_error_tx, data_connection_on_error_rx) =
            channel::<data::formats::OnErrorTxParameters>(0);
        let data_connection_on_error_rx =
            data_connection_on_error_rx.for_each(data_connection_on_error);
        tokio::spawn(data_connection_on_error_rx);

        let _ = data::connect_flow(
            &*BASE_URL,
            peer_info,
            Some(data_connection_on_open_tx),
            Some(data_connection_on_close_tx),
            Some(data_connection_on_error_tx),
        )
        .await;
    }
}

// FIXME: write test
// When PeerObject receives CONNECTION event, it fires.
// This function need to listen DataConnection events and redirect data from neighbour.
async fn peer_on_connect(event: PeerConnectionEvent) {
    let peer_info = event.params;
    let data_connection_id = event.data_params.data_connection_id;
    info!(
        "Peer {} receives connection. The connection's id is {}",
        peer_info.peer_id.as_str(),
        data_connection_id.as_str()
    );

    let (data_connection_on_open_tx, data_connection_on_open_rx) =
        channel::<data::formats::OnOpenTxParameters>(0);
    let data_connection_on_open_rx = data_connection_on_open_rx.for_each(data_connection_on_open);
    tokio::spawn(data_connection_on_open_rx);

    let (data_connection_on_close_tx, data_connection_on_close_rx) =
        channel::<data::formats::OnCloseTxParameters>(0);
    let data_connection_on_close_rx =
        data_connection_on_close_rx.for_each(data_connection_on_close);
    tokio::spawn(data_connection_on_close_rx);

    let (data_connection_on_error_tx, data_connection_on_error_rx) =
        channel::<data::formats::OnErrorTxParameters>(0);
    let data_connection_on_error_rx =
        data_connection_on_error_rx.for_each(data_connection_on_error);
    tokio::spawn(data_connection_on_error_rx);

    let _ = data::redirect_flow(
        &*BASE_URL,
        data_connection_id,
        Some(data_connection_on_open_tx),
        Some(data_connection_on_close_tx),
        Some(data_connection_on_error_tx),
    )
    .await;
}

// FIXME: this application should exit after this event fires
async fn peer_on_close(event: PeerCloseEvent) {
    let peer_info = event.params;
    info!("Peer {} has closed", peer_info.peer_id.as_str());
}

// FIXME: error handling and this application should exit after this event fires
async fn peer_on_error(event: PeerErrorEvent) {
    let peer_info = event.params;
    let error_message = event.error_message;
    warn!(
        "Peer {} returns error. {}",
        peer_info.peer_id.as_str(),
        error_message
    );
}

// FIXME: notify end-user-programs that they can start sending data
async fn data_connection_on_open(event: OnOpenTxParameters) {
    info!("DataConnection {} is opened", event.0.as_str());
}

// FIXME: notify end-user-programs that they must stop sending data
async fn data_connection_on_close(event: OnCloseTxParameters) {
    info!("DataConnection {} is closed", event.0.as_str());
    // Delete DataConnection
    // When DataConnection object is deleted, WebRTC Gateway release Data object used by DataConnection.
    // So this code don't have to call delete_data API here.
    let _ = data::delete_data_connection(&*BASE_URL, event.0).await;
}

// FIXME: error handling and rotify end-user-programs that they should stop sending data
async fn data_connection_on_error(event: OnErrorTxParameters) {
    let data_connection_id = event.0;
    let error_message = event.1;
    warn!(
        "DataConnection {} returns error. {}",
        data_connection_id.as_str(),
        error_message
    );
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let (peer_on_open_tx, peer_on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    let peer_on_open_rx = peer_on_open_rx.for_each(peer_on_open);
    tokio::spawn(peer_on_open_rx);

    let (peer_on_connect_tx, peer_on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    let peer_on_connect_rx = peer_on_connect_rx.for_each(peer_on_connect);
    tokio::spawn(peer_on_connect_rx);

    let (peer_on_close_tx, peer_on_close_rx) = channel::<peer::formats::PeerCloseEvent>(0);
    let peer_on_close_rx = peer_on_close_rx.for_each(peer_on_close);
    tokio::spawn(peer_on_close_rx);

    let (peer_on_error_tx, peer_on_error_rx) = channel::<peer::formats::PeerErrorEvent>(0);
    let peer_on_error_rx = peer_on_error_rx.for_each(peer_on_error);
    tokio::spawn(peer_on_error_rx);

    let observers = PeerEventObservers {
        on_open_tx: vec![peer_on_open_tx],
        on_call_tx: vec![],
        on_connect_tx: vec![peer_on_connect_tx],
        on_close_tx: vec![peer_on_close_tx],
        on_error_tx: vec![peer_on_error_tx],
    };

    let _ = peer_runner(observers).await;
}
