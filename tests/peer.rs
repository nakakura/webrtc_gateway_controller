/*
use futures::*;
use serde_json::json;

use helper::server;
use webrtc_gateway_controller::peer::*;
#[tokio::test]
async fn test_listen_events() {
    let peer_id = "peer_id";
    use std::sync::{Arc, Mutex};
    let x = Arc::new(Mutex::new(0usize));

    let server = server::http(move |req| {
        let x_arc = x.clone();
        async move {
            let events_uri = format!("/peers/{}/events?token={}", "peer_id", "token_test");
            if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "PEERS_CREATE",
                    "params": {
                        "peer_id": peer_id,
                        "token": "token_test",
                    }
                });
                http::Response::builder()
                    .status(hyper::StatusCode::CREATED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else if req.uri().to_string() == events_uri && req.method() == reqwest::Method::GET {
                (*x_arc.lock().unwrap()) += 1;
                let json;

                if *x_arc.lock().unwrap() == 1 {
                    json = json!({
                        "event": "OPEN",
                        "params": {
                            "peer_id": "peer_id",
                            "token": "token_test"
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else if *x_arc.lock().unwrap() == 2 {
                    json = json!({
                        "event": "CONNECTION",
                        "data_params": {
                            "data_connection_id": "dc-test"
                        },
                        "params": {
                            "peer_id": "peer_id",
                            "token": "token_test"
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else if *x_arc.lock().unwrap() == 3 {
                    json = json!({
                        "event": "CALL",
                        "call_params": {
                            "media_connection_id": "mc-test"
                        },
                        "params": {
                            "peer_id": "peer_id",
                            "token": "token_test"
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else if *x_arc.lock().unwrap() == 4 {
                    json = json!({
                        "event": "CLOSE",
                        "params": {
                            "peer_id": "peer_id",
                            "token": "token_test"
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else if *x_arc.lock().unwrap() == 5 {
                    json = json!({});
                    http::Response::builder()
                        .status(hyper::StatusCode::REQUEST_TIMEOUT)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    json = json!({
                        "event": "ERROR",
                        "params": {
                            "peer_id": "peer_id",
                            "token": "token_test"
                        },
                        "error_message": "error"
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                }
            } else {
                unreachable!();
            }
        }
    });

    let base_url = format!("http://{}", server.addr());
    let peer_info = api::create_peer(&base_url, peer_id, true)
        .await
        .expect("create peer failed")
        .params;

    let (on_open_tx, on_open_rx) = futures::channel::mpsc::channel::<formats::PeerOpenEvent>(0);
    let on_open_future = on_open_rx.for_each(on_open);

    let (on_call_tx, on_call_rx) = futures::channel::mpsc::channel::<formats::PeerCallEvent>(0);
    let on_call_future = on_call_rx.for_each(on_call);

    let (on_connect_tx, on_connect_rx) =
        futures::channel::mpsc::channel::<formats::PeerConnectionEvent>(0);
    let on_connect_future = on_connect_rx.for_each(on_connect);

    let (on_close_tx, on_close_rx) = futures::channel::mpsc::channel::<formats::PeerCloseEvent>(0);
    let on_close_future = on_close_rx.for_each(on_close);

    let (on_error_tx, on_error_rx) = futures::channel::mpsc::channel::<formats::PeerErrorEvent>(0);
    let on_error_future = on_error_rx.for_each(on_error);

    let listen_event_future = listen_events(
        &base_url,
        peer_info.clone(),
        on_open_tx,
        on_call_tx,
        on_connect_tx,
        on_close_tx,
        on_error_tx,
    );
    tokio::spawn(on_open_future.map(|_| ()));
    tokio::spawn(on_call_future.map(|_| ()));
    tokio::spawn(on_connect_future.map(|_| ()));
    tokio::spawn(on_close_future.map(|_| ()));
    tokio::spawn(on_error_future.map(|_| ()));
    let _ = listen_event_future.await;

    fn on_open(_event: formats::PeerOpenEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    fn on_call(_event: formats::PeerCallEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    fn on_connect(_event: formats::PeerConnectionEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    fn on_close(_event: formats::PeerCloseEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    fn on_error(_event: formats::PeerErrorEvent) -> impl Future<Output = ()> {
        future::ready(())
    }
}
*/
