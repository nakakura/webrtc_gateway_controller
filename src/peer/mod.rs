mod data;

use reqwest;
use reqwest::Client;

use crate::error;

use data::*;

/// It access to the POST /peer endpoint, and return its response.
/// Server returns values with 201 Created and 403 Forbidden.
/// In this case, create_peer returns CreatedResponse
/// If server returns 400, 405, 406, 408,
/// or if server is not working,
/// this function returns error
/// Also, if server returns json which command_type is not "PEERS_CREATE", it returns error.
/// http://35.200.46.204/#/1.peers/peer
pub async fn create_peer(
    url: &str,
    peer_id: &str,
    turn: bool,
) -> Result<CreatedResponse, error::ErrorEnum> {
    let key = &*crate::API_KEY;
    let peer_options = PeerOptions {
        key: key.to_string(),
        domain: (*crate::DOMAIN).clone(),
        peer_id: peer_id.to_string(),
        turn: turn,
    };

    let base_url = format!("{}/peers", url);
    let res = Client::new()
        .post(&base_url)
        .json(&peer_options)
        .send()
        .await?;
    // FIXME check res.status()
    res.json::<data::CreatedResponse>()
        .await
        .map_err(Into::into)
        .and_then(|response| match response {
            CreatedResponse::Success(s) => {
                if s.command_type == "PEERS_CREATE" {
                    Ok(CreatedResponse::Success(s))
                } else {
                    Err(error::ErrorEnum::create_myerror(
                        "webrtc gateway might be old version",
                    ))
                }
            }
            CreatedResponse::Error(e) => Ok(CreatedResponse::Error(e)),
        })
}

/// It access to the GET /peer/{peer_id}/event?token={token} endpoint, and return its response.
/// When a server returns values with 200 Ok,
/// listen_event returns PeerEventEnum(OPEN, CONNECTION, CALL, STREAM, CLOSE, ERROR)
/// When a server returns 403, 404, 405, 406, 408
/// this function returns error
/// Also, if server returns json which command_type is not "PEERS_EVENTS", it returns error.
/// http://35.200.46.204/#/1.peers/peer_event
pub async fn listen_event(
    url: &str,
    peer_info: &PeerInfo,
) -> Result<PeerEventEnum, error::ErrorEnum> {
    let base_url = format!(
        "{}/peers/{}/events?token={}",
        url, peer_info.peer_id, peer_info.token
    );

    let res = Client::new()
        .get(&base_url)
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .send()
        .await?;
    match res.status() {
        http::status::StatusCode::OK => res.json::<PeerEventEnum>().await.map_err(Into::into),
        http::status::StatusCode::BAD_REQUEST => res
            .json::<PeerErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: PeerErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        http::status::StatusCode::FORBIDDEN => {
            Err(error::ErrorEnum::create_myerror("recv Forbidden"))
        }
        http::status::StatusCode::NOT_FOUND => {
            Err(error::ErrorEnum::create_myerror("recv NotFound"))
        }
        http::status::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        http::status::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        http::status::StatusCode::REQUEST_TIMEOUT => {
            Err(error::ErrorEnum::create_myerror("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_create_peer {
    use serde_json::json;

    use crate::helper::*;
    use crate::peer::*;

    /// A WebRTC Gateway returns 200 Created code and a PeerResponse struct, if it succeeds to create a Peer Object
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn create_peer() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |mut req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                    let mut full: Vec<u8> = Vec::new();
                    while let Some(item) = req.body_mut().next().await {
                        full.extend(&*item.unwrap());
                    }
                    let peer_options: PeerOptions =
                        serde_json::from_slice(&full).expect("PeerOptions parse error");
                    let json = json!({
                        "command_type": "PEERS_CREATE",
                        "params": {
                            "peer_id": peer_options.peer_id,
                            "token": token,
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::CREATED)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_peer(&addr, peer_id, false);
        if let CreatedResponse::Success(response) = task.await.expect("CreatedResponse parse error")
        {
            assert_eq!(response.command_type, "PEERS_CREATE".to_string());
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 403 Forbidden code and a PeerResponse struct.
    /// It happens when user inputs wrong api keys
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn create_peer_wrong_api_key() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "PEERS_CREATE",
                        "params": {
                            "errors": [
                               {
                                   "field": "",
                                   "message": "internal peer open error."
                               }
                           ]
                       }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::FORBIDDEN)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_peer(&addr, peer_id, false);
        if let CreatedResponse::Error(response) = task.await.expect("CreatedResponse parse error") {
            assert_eq!(response.command_type, "PEERS_CREATE".to_string());
            assert_eq!(response.params.errors.len(), 1);
        } else {
            unreachable!();
        }
    }

    /// If this program connects to an another web server,
    /// create_peer returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn create_peer_connected_to_another_webserver() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                    let json = json!("invalid-message");
                    http::Response::builder()
                        .status(hyper::StatusCode::CREATED)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await;
        assert!(result.is_err());
        if let Err(error::ErrorEnum::ReqwestError { error: _e }) = result {
        } else {
            unreachable!();
        }
    }

    /// If this program connects to an another web server,
    /// create_peer returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn create_peer_connected_to_another_version() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |mut req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                    let mut full: Vec<u8> = Vec::new();
                    while let Some(item) = req.body_mut().next().await {
                        full.extend(&*item.unwrap());
                    }
                    let peer_options: PeerOptions =
                        serde_json::from_slice(&full).expect("PeerOptions parse error");
                    let json = json!({
                        "command_type": "PEERS_CREATE_v2",
                        "params": {
                            "peer_id": peer_options.peer_id,
                            "token": token,
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::CREATED)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await;
        assert!(result.is_err());
        if let Err(error::ErrorEnum::MyError { error: _e }) = result {
        } else {
            unreachable!();
        }
    }

    /// If WebRTC Gateway itself is not found, create_peer function returns error
    #[tokio::test]
    async fn create_peer_without_server() {
        let peer_id = "hoge";

        let task = super::create_peer("http://localhost:0", peer_id, false);
        let result = task.await;
        assert!(result.is_err());
        if let Err(error::ErrorEnum::ReqwestError { error: _e }) = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_listen_event {
    use serde_json::json;

    use crate::helper::*;
    use crate::peer::data::{PeerEventEnum, PeerInfo};
    use crate::peer::*;

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_open() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "OPEN",
                        "params": {
                            "peer_id": peer_id,
                            "token": token
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::OPEN(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_connection() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "CONNECTION",
                        "data_params": {
                            "data_connection_id": "dc-test"
                        },
                        "params": {
                            "peer_id": peer_id,
                            "token": token
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::CONNECTION(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
            assert_eq!(response.data_params.data_connection_id, "dc-test");
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_call() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "CALL",
                        "call_params": {
                            "media_connection_id": "mc-test"
                        },
                        "params": {
                            "peer_id": peer_id,
                            "token": token
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::CALL(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
            assert_eq!(response.call_params.media_connection_id, "mc-test");
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_close() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "CLOSE",
                        "params": {
                            "peer_id": peer_id,
                            "token": token
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::CLOSE(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_error() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "ERROR",
                        "params": {
                            "peer_id": peer_id,
                            "token": token
                        },
                        "error_message": "error"
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::ERROR(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
            assert_eq!(response.error_message, "error");
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns invalid json, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_invalid_json() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "event": "OPEN",
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await;
        assert!(result.is_err());
    }

    /// A WebRTC Gateway returns 400 Invalid Input code and a PeerResponse struct, if any error happen
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_400() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "command_type": "PEERS_EVENTS",
                        "params": {
                            "errors": [
                                  {
                                    "field": "peer_id",
                                    "message": "peer_id field is not specified"
                                  }
                            ]
                        }
                    });
                    http::Response::builder()
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 403, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_403() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({});

                    http::Response::builder()
                        .status(hyper::StatusCode::FORBIDDEN)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 404, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_404() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({});

                    http::Response::builder()
                        .status(hyper::StatusCode::NOT_FOUND)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 405, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_405() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({});

                    http::Response::builder()
                        .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 406, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_406() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({});

                    http::Response::builder()
                        .status(hyper::StatusCode::NOT_ACCEPTABLE)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 408, listen_event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn test_listen_event_recv_408() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}/events?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({});

                    http::Response::builder()
                        .status(hyper::StatusCode::REQUEST_TIMEOUT)
                        .header("Content-type", "application/json")
                        .body(hyper::Body::from(json.to_string()))
                        .unwrap()
                } else {
                    unreachable!();
                }
            }
        });

        let addr = format!("http://{}", server.addr());
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
        };

        let task = super::listen_event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}
