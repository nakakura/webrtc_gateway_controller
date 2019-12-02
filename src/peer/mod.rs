pub mod formats;

use reqwest;
use reqwest::Client;

use crate::error;
use formats::*;

/// It access to the POST /peer endpoint, and return its response.
/// Server returns values with 201 Created and 403 Forbidden.
/// In this case, create_peer returns CreatedResponse
/// If server returns 400, 405, 406, 408,
/// or if server is not working,
/// this function returns error
/// Also, if server returns json which command_type is not "PEERS_CREATE", it returns error.
/// http://35.200.46.204/#/1.peers/peer
pub async fn create_peer(
    base_url: &str,
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

    let api_url = format!("{}/peers", base_url);
    let res = Client::new()
        .post(&api_url)
        .json(&peer_options)
        .send()
        .await?;
    match res.status() {
        reqwest::StatusCode::CREATED => res
            .json::<formats::CreatedResponse>()
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
            }),
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => {
            Err(error::ErrorEnum::create_myerror("recv Forbidden"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
            Err(error::ErrorEnum::create_myerror("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}

/// It access to the GET /peer/{peer_id}/event?token={token} endpoint, and return its response.
/// This function is used for long-polling, so it should be re-called after receiving events or 408 Request Timeout
/// When a server returns values with 200 Ok,
/// event returns PeerEventEnum::{OPEN, CONNECTION, CALL, STREAM, CLOSE, ERROR}
/// When a server returns 408 Request Timeout,
/// event returns PeerEventEnum::Timeout
/// When a server returns 403, 404, 405, 406
/// this function returns error
/// Also, if server returns json which command_type is not "PEERS_EVENTS", it returns error.
/// http://35.200.46.204/#/1.peers/peer_event
pub async fn event(
    base_url: &str,
    peer_info: &PeerInfo,
) -> Result<PeerEventEnum, error::ErrorEnum> {
    let api_url = format!(
        "{}/peers/{}/events?token={}",
        base_url, peer_info.peer_id, peer_info.token
    );

    let res = Client::new()
        .get(&api_url)
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .send()
        .await?;
    match res.status() {
        reqwest::StatusCode::OK => res.json::<PeerEventEnum>().await.map_err(Into::into),
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => {
            Err(error::ErrorEnum::create_myerror("recv Forbidden"))
        }
        reqwest::StatusCode::NOT_FOUND => {
            Err(error::ErrorEnum::create_myerror("recv NotFound"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => Ok(PeerEventEnum::Timeout),
        _ => {
            unreachable!();
        }
    }
}

/// It access to the DELETE /peers/{peer_id} endpoint, and return its response.
/// If a WebRTC Gateway succeed to delete a Peer Object, it returns 204.
/// If any error happens, it returns 400, 403, 404, 405, 406, 408.
/// When it returns 400, it also send a json message.
/// http://35.200.46.204/#/1.peers/peer_destroy
pub async fn delete_peer(base_url: &str, peer_info: &PeerInfo) -> Result<(), error::ErrorEnum> {
    let api_url = format!(
        "{}/peers/{}?token={}",
        base_url, peer_info.peer_id, peer_info.token
    );
    let res = Client::new().delete(&api_url).send().await?;
    match res.status() {
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => {
            Err(error::ErrorEnum::create_myerror("recv Forbidden"))
        }
        reqwest::StatusCode::NOT_FOUND => {
            Err(error::ErrorEnum::create_myerror("recv NotFound"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
            Err(error::ErrorEnum::create_myerror("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}

/// Status function access to the GET /peers/{peer_id}/status endpoint to get status of WebRTC Gateway
/// The API returns JSON with 200 OK.
/// If any error happens, it returns 400, 403, 404, 405, 406, 408
/// http://35.200.46.204/#/1.peers/peer_status
pub async fn status(
    base_url: &str,
    peer_info: &PeerInfo,
) -> Result<PeerStatusMessage, error::ErrorEnum> {
    let api_url = format!(
        "{}/peers/{}/status?token={}",
        base_url, peer_info.peer_id, peer_info.token
    );
    let res = Client::new().get(&api_url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => res.json::<PeerStatusMessage>().await.map_err(Into::into),
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => {
            Err(error::ErrorEnum::create_myerror("recv Forbidden"))
        }
        reqwest::StatusCode::NOT_FOUND => {
            Err(error::ErrorEnum::create_myerror("recv NotFound"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
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

    use crate::test_helper::*;
    use crate::peer::*;

    /// A WebRTC Gateway returns 201 Created and a PeerResponse struct, if it succeeds to create a Peer Object
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_201() {
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

    /// If this program connects to an another web server,
    /// create_peer returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_201_but_from_another_webserver() {
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
    async fn recv_201_from_another_version() {
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

    /// When WebRTC Gateway returns 400, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_400() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "PEERS_CREATE",
                        "params": {
                            "errors": [
                                {
                                    "field": "key",
                                    "message": "key field is not specified"
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
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// A WebRTC Gateway returns 403 Forbidden code and a PeerResponse struct.
    /// It happens when user inputs wrong api keys
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_403() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
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
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await.err().unwrap();
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When WebRTC Gateway returns 405, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_405() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
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
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When WebRTC Gateway returns 405, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_406() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
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
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When WebRTC Gateway returns 408, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_408() {
        let peer_id = "hoge";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/peers" && req.method() == reqwest::Method::POST {
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
        let task = super::create_peer(&addr, peer_id, false);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If WebRTC Gateway itself is not found, create_peer function returns error
    #[tokio::test]
    async fn no_server() {
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
mod test_event {
    use serde_json::json;

    use crate::test_helper::*;
    use crate::peer::formats::{PeerEventEnum, PeerInfo};
    use crate::peer::*;

    /// A WebRTC Gateway returns 200 OK code and a PeerResponse struct, if it recv correct peer_id and peer_token
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_200_recv_open() {
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

        let task = super::event(&addr, &peer_info);
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
    async fn recv_200_recv_connection() {
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

        let task = super::event(&addr, &peer_info);
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
    async fn recv_200_recv_call() {
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

        let task = super::event(&addr, &peer_info);
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
    async fn recv_200_recv_close() {
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

        let task = super::event(&addr, &peer_info);
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
    async fn recv_200_recv_error() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.expect("event parse error");
        if let PeerEventEnum::ERROR(response) = result {
            assert_eq!(response.params.peer_id, peer_id);
            assert_eq!(response.params.token, token);
            assert_eq!(response.error_message, "error");
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns invalid json, event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_200_but_recv_invalid_json() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await;
        assert!(result.is_err());
    }

    /// A WebRTC Gateway returns 400 Invalid Input code and a PeerResponse struct, if any error happen
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_400() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 403, event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_403() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 404, event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_404() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 405, event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_405() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 406, event returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_406() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If a WebRTC Gateway returns 408, event returns PeerEventEnum::Timeout
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_408() {
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

        let task = super::event(&addr, &peer_info);
        let result = task.await.expect("parse error");
        assert_eq!(result, PeerEventEnum::Timeout);
    }
}

#[cfg(test)]
mod test_delete_peer {
    use serde_json::json;

    use crate::test_helper::*;
    use crate::peer::*;

    /// A WebRTC Gateway returns 204, if it succeeds to delete a Peer Objec
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_204() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            let uri = format!("/peers/{}?token={}", peer_id, token);
            async move {
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
                    let json = json!({});
                    http::Response::builder()
                        .status(hyper::StatusCode::NO_CONTENT)
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await;
        assert!(result.is_ok());
    }

    /// When any error happens, WebRTC Gateway returns 400.
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_400() {
        let peer_id = "hoge";
        let token = "test_token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
                    let json = json!({
                        "command_type": "PEERS_DELETE",
                        "params": {
                            "errors": [
                                {
                                    "field": "key",
                                    "message": "key field is not specified"
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, WebRTC Gateway returns 403.
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_403() {
        let peer_id = "hoge";
        let token = "test_token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, WebRTC Gateway returns 404.
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_404() {
        let peer_id = "hoge";
        let token = "test_token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, WebRTC Gateway returns 405.
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_405() {
        let peer_id = "hoge";
        let token = "test_token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Request Timeout
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_408() {
        let peer_id = "hoge";
        let token = "test_token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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

        let task = super::delete_peer(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_status {
    use serde_json::json;

    use crate::test_helper::*;
    use crate::peer::formats::PeerStatusMessage;
    use crate::peer::*;

    /// Status API returns json with 200 OK
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_200() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "peer_id": peer_id,
                        "disconnected": false
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
        let task = super::status(&addr, &peer_info);
        let status: PeerStatusMessage = task.await.expect("parse error");
        assert_eq!(status.peer_id, peer_id);
        assert_eq!(status.disconnected, false);
    }

    /// When any error happens, status API returns json with 400 BAD_REQUEST
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_400() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                    let json = json!({
                        "command_type": "PEERS_STATUS",
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, status API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_403() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, status API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_404() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, status API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_405() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, status API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_406() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// When any error happens, status API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_408() {
        let peer_id = "hoge";
        let token = "test-token";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/peers/{}/status?token={}", peer_id, token);
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
        let task = super::status(&addr, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}
