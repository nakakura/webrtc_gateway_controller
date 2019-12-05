use futures::channel::mpsc::*;
use futures::*;

use webrtc_gateway_controller::data::formats::CreatedResponse;
use webrtc_gateway_controller::peer::formats::*;
use webrtc_gateway_controller::*;

async fn peer_open_and_listen_events(
    base_url: &str,
    peer_id: &str,
    on_open_tx: Sender<PeerOpenEvent>,
    on_connect_tx: Sender<PeerConnectionEvent>,
    on_call_tx: Sender<PeerCallEvent>,
    on_close_tx: Sender<PeerCloseEvent>,
    on_error_tx: Sender<PeerErrorEvent>,
) {
    let peer_info = peer::api::create_peer(base_url, peer_id, true)
        .await
        .expect("create peer failed")
        .params;
    // Start Long Polling events
    let listen_event_future = peer::listen_events(
        base_url,
        peer_info.clone(),
        on_open_tx,
        on_call_tx,
        on_connect_tx,
        on_close_tx,
        on_error_tx,
    );
    let _ = listen_event_future.await;
}

#[cfg(not(test))]
#[tokio::main]
async fn main() {
    let base_url = &*BASE_URL;
    // FIRES when GET /peer/{peer_id}/events returns OPEN event
    let (on_open_tx, on_open_rx) = channel::<peer::formats::PeerOpenEvent>(0);
    // On Open Event is used in some flows, so redirect it
    // For data connection
    let (sub_on_open_tx_1, mut sub_on_open_rx_1) = channel::<peer::formats::PeerOpenEvent>(0);
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

    // DataObject can be created without PeerObject
    let created_response = data::api::create_data(base_url);
    /*
    After peer is open and data is created, start connection
    FIXME
    In this flow, GET /data/connections/{data_connection_id}/events should be also polled.
    */
    let data_flow_future = future::join(created_response, sub_on_open_rx_1.next()).then(|d| {
        async move {
            if let (Ok(response), Some(event)) = d {
                let _ = connect(base_url, response, event).await;
                future::ok(())
            } else {
                future::err::<(), error::ErrorEnum>(error::ErrorEnum::create_myerror(
                    "not ready for connect",
                ))
            }
        }
    });

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

    let peer_future = peer_open_and_listen_events(
        base_url,
        &*PEER_ID,
        on_open_tx,
        on_connect_tx,
        on_call_tx,
        on_close_tx,
        on_error_tx,
    );
    // Start each futures
    tokio::spawn(on_open_future.map(|_| ()));
    tokio::spawn(on_call_future);
    tokio::spawn(on_connect_future);
    tokio::spawn(on_close_future);
    tokio::spawn(on_error_future);
    let _ = future::join(data_flow_future, peer_future).await;
}

// FIXME
// FIRES when GET /peer/{peer_id}/events returns OPEN event
async fn connect(base_url: &str, response: CreatedResponse, event: PeerOpenEvent) {
    let data_id = data::formats::DataId {
        data_id: response.data_id,
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
