use futures::channel::mpsc::channel;
use futures::*;

use webrtc_gateway_controller::*;

#[cfg(not(test))]
#[tokio::main]
async fn main() {
    let peer_info = peer::api::create_peer(&*BASE_URL, &*PEER_ID, true)
        .await
        .expect("create peer failed")
        .params;
    // FIRES when GET /peer/{peer_id}/events returns OPEN event
    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    let on_open_future = on_open_rx.for_each(on_open);

    // FIRES when GET /peer/{peer_id}/events returns CALL event
    let (on_call_tx, on_call_rx) = channel::<peer::formats::PeerCallEvent>(0);
    let on_call_future = on_call_rx.for_each(on_call);

    // FIRES when GET /peer/{peer_id}/events returns CONNECT event
    let (on_connect_tx, on_connect_rx) = channel::<peer::formats::PeerConnectionEvent>(0);
    let on_connect_future = on_connect_rx.for_each(on_connect);

    // FIRES when GET /peer/{peer_id}/events returns CLOSE event
    let (on_close_tx, on_close_rx) = channel::<peer::formats::PeerCloseEvent>(0);
    let on_close_future = on_close_rx.for_each(on_close);

    // FIRES when GET /peer/{peer_id}/events returns ERROR event
    let (on_error_tx, on_error_rx) = channel::<peer::formats::PeerErrorEvent>(0);
    let on_error_future = on_error_rx.for_each(on_error);

    // Start Long Polling events
    let listen_event_future = peer::listen_events(
        &*BASE_URL,
        peer_info.clone(),
        on_open_tx,
        on_call_tx,
        on_connect_tx,
        on_close_tx,
        on_error_tx,
    );

    // Start each futures
    tokio::spawn(on_open_future);
    tokio::spawn(on_call_future);
    tokio::spawn(on_connect_future);
    tokio::spawn(on_close_future);
    tokio::spawn(on_error_future);
    let _ = listen_event_future.await;
}

// FIXME
// FIRES when GET /peer/{peer_id}/events returns OPEN event
async fn on_open(event: peer::formats::PeerOpenEvent) {
    let base_url = &*BASE_URL;
    let created_response = data::api::create_data(base_url)
        .await
        .unwrap_or_else(|e| panic!("err in on_open: {}", e));
    let data_id = data::formats::DataId {
        data_id: created_response.data_id,
    };
    let query = data::formats::CreateDataConnectionQuery {
        peer_id: event.params.peer_id,
        token: event.params.token,
        options: None,                        //FIXME
        target_id: "data_callee".to_string(), //FIXME
        params: data_id,
        redirect_params: None, //FIXME
    };
    let created_data_connection = data::api::create_data_connection(base_url, &query).await;
    println!("created data connection {:?}", created_data_connection);
    ()
}

// FIXME
fn on_call(_event: peer::formats::PeerCallEvent) -> impl Future<Output = ()> {
    future::ready(())
}

// FIXME
fn on_connect(_event: peer::formats::PeerConnectionEvent) -> impl Future<Output = ()> {
    future::ready(())
}

// FIXME
fn on_close(_event: peer::formats::PeerCloseEvent) -> impl Future<Output = ()> {
    future::ready(())
}

// FIXME
fn on_error(_event: peer::formats::PeerErrorEvent) -> impl Future<Output = ()> {
    future::ready(())
}
