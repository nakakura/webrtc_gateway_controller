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

#[cfg(test)]
mod test_peer {
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
