/// Functions in this mod are responsible for calling raw APIs
use futures::*;
use reqwest;
use reqwest::Client;

use super::formats::*;
use crate::common::api;
use crate::error;
use crate::prelude::{PeerId, PeerInfo};

/// It access to the POST /peer endpoint, and return its response.
/// Server returns values with 201 Created and 403 Forbidden.
/// In this case, create_peer returns CreatedResponse
/// If server returns 400, 405, 406, 408,
/// or if server is not working,
/// this function returns error
/// Also, if server returns json which command_type is not "PEERS_CREATE", it returns error.
/// http://35.200.46.204/#/1.peers/peer
pub(crate) async fn create_peer(
    base_url: &str,
    api_key: impl Into<String>,
    domain: impl Into<String>,
    peer_id: PeerId,
    turn: bool,
) -> Result<CreatedResponse, error::Error> {
    let peer_options = CreatePeerQuery {
        key: api_key.into(),
        domain: domain.into(),
        peer_id: peer_id,
        turn: turn,
    };
    let api_url = format!("{}/peers", base_url);
    let api_call = || {
        Client::new()
            .post(&api_url)
            .json(&peer_options)
            .send()
            .map_err(Into::into)
    };
    let parser = |r: reqwest::Response| r.json::<CreatedResponse>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
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
pub(crate) async fn event(base_url: &str, peer_info: &PeerInfo) -> Result<EventEnum, error::Error> {
    let api_url = format!(
        "{}/peers/{}/events?token={}",
        base_url,
        peer_info.peer_id.as_str(),
        peer_info.token.as_str()
    );
    let api_call = || {
        {
            Client::new()
                .get(&api_url)
                .header(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .send()
        }
        .map_err(Into::into)
    };
    let parser = |r: reqwest::Response| r.json::<EventEnum>().map_err(Into::into);
    match api::api_access(reqwest::StatusCode::OK, true, api_call, parser).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            error::Error::LocalError(message) if message == "recv RequestTimeout" => {
                Ok(EventEnum::TIMEOUT)
            }
            e => Err(e),
        },
    }
}

/// It access to the DELETE /peers/{peer_id} endpoint, and return its response.
/// If a WebRTC Gateway succeed to delete a Peer Object, it returns 204.
/// If any error happens, it returns 400, 403, 404, 405, 406, 408.
/// When it returns 400, it also send a json message.
/// http://35.200.46.204/#/1.peers/peer_destroy
pub(crate) async fn delete_peer(base_url: &str, peer_info: &PeerInfo) -> Result<(), error::Error> {
    let api_url = format!(
        "{}/peers/{}?token={}",
        base_url,
        peer_info.peer_id.as_str(),
        peer_info.token.as_str()
    );
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Status function access to the GET /peers/{peer_id}/status endpoint to get status of WebRTC Gateway
/// The API returns JSON with 200 OK.
/// If any error happens, it returns 400, 403, 404, 405, 406, 408
/// http://35.200.46.204/#/1.peers/peer_status
pub(crate) async fn status(
    base_url: &str,
    peer_info: &PeerInfo,
) -> Result<PeerStatusMessage, error::Error> {
    let api_url = format!(
        "{}/peers/{}/status?token={}",
        base_url,
        peer_info.peer_id.as_str(),
        peer_info.token.as_str()
    );
    let api_call = || Client::new().get(&api_url).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<PeerStatusMessage>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

#[cfg(test)]
mod test_create_peer {
    use mockito::mock;

    use crate::error;
    use crate::prelude::*;

    fn create_params() -> (PeerId, Token) {
        let peer_id = PeerId::new("hoge");
        let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap();
        (peer_id, token)
    }

    /// A WebRTC Gateway returns 201 Created and a PeerResponse struct, if it succeeds to create a Peer Object
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_201() {
        // set up parameters
        let (peer_id, token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "PEERS_CREATE",
                "params": {
                    "peer_id": "hoge",
                    "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.expect("CreatedResponse parse error");
        assert_eq!(result.command_type, "PEERS_CREATE".to_string());
        assert_eq!(result.params.peer_id, peer_id);
        assert_eq!(result.params.token, token);

        // server called
        httpserver.assert();
    }

    /// If this program connects to an another web server,
    /// create_peer returns error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_201_but_from_another_webserver() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{ invalid: "message" }"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await;
        assert!(result.is_err());
        if let Err(error::Error::ReqwestError(_e)) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// When WebRTC Gateway returns 400, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_400() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "PEERS_CREATE",
                    "params": {
                        "errors": [
                            {
                                "field": "key",
                                "message": "key field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// A WebRTC Gateway returns 403 Forbidden code and a PeerResponse struct.
    /// It happens when user inputs wrong api keys
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_403() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// When WebRTC Gateway returns 405, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_405() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// When WebRTC Gateway returns 405, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_406() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// When WebRTC Gateway returns 408, parse error
    /// http://35.200.46.204/#/1.peers/peer
    #[tokio::test]
    async fn recv_408() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // set up server mock
        let httpserver = mock("POST", "/peers")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// If WebRTC Gateway itself is not found, create_peer function returns error
    #[tokio::test]
    async fn no_server() {
        // set up parameters
        let (peer_id, _token) = create_params();

        // call api
        let url = "http://localhost:0".to_string();
        let task = super::create_peer(&url, "api_key", "domain", peer_id.clone(), false);
        let result = task.await;
        assert!(result.is_err());
        if let Err(error::Error::ReqwestError(_e)) = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_event {
    use mockito::mock;

    use crate::error;
    use crate::peer::formats::*;

    fn create_params() -> PeerInfo {
        let peer_id = PeerId::new("hoge");
        let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap();
        PeerInfo {
            peer_id: peer_id.clone(),
            token: token.clone(),
        }
    }

    /// PeerEvent API returns 200 and events
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_recv_open() {
        // set up parameters
        let peer_info = create_params();

        // set up server mock
        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "OPEN",
                    "params": {
                        "peer_id": "hoge",
                        "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::event(&url, &peer_info);
        let result = task.await.expect("event parse error");
        if let EventEnum::OPEN(response) = result {
            assert_eq!(response.params.peer_id, peer_info.peer_id);
            assert_eq!(response.params.token, peer_info.token);
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// PeerEvent API returns 200 and events
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_recv_connection() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "CONNECTION",
                    "params": {
                        "peer_id": "hoge",
                        "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                    },
                    "data_params": {
                        "data_connection_id": "da-102127d9-30de-413b-93f7-41a33e39d82b"
                    }
                  }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::event(&url, &peer_info);
        let result = task.await.expect("event parse error");
        if let EventEnum::CONNECTION(response) = result {
            assert_eq!(response.params.peer_id, peer_info.peer_id);
            assert_eq!(response.params.token, peer_info.token);
            assert_eq!(
                response.data_params.data_connection_id.as_str(),
                "da-102127d9-30de-413b-93f7-41a33e39d82b"
            );
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// PeerEvent API returns 200 and events
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_recv_call() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "CALL",
                    "params": {
                        "peer_id": "hoge",
                        "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                    },
                    "call_params": {
                        "media_connection_id": "mc-102127d9-30de-413b-93f7-41a33e39d82b"
                    }
                  }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.expect("event parse error");
        if let EventEnum::CALL(response) = result {
            assert_eq!(response.params.peer_id, peer_info.peer_id);
            assert_eq!(response.params.token, peer_info.token);
            assert_eq!(
                response.call_params.media_connection_id.as_str(),
                "mc-102127d9-30de-413b-93f7-41a33e39d82b"
            );
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// PeerEvent API returns 200 and events
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_recv_close() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "CLOSE",
                    "params": {
                        "peer_id": "hoge",
                        "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.expect("event parse error");
        if let EventEnum::CLOSE(response) = result {
            assert_eq!(response.params.peer_id, peer_info.peer_id);
            assert_eq!(response.params.token, peer_info.token);
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// PeerEvent API returns 200 and events, even the event type is error
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_recv_error() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "ERROR",
                    "params": {
                        "peer_id": "hoge",
                        "token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"
                    },
                    "error_message": "error"
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.expect("event parse error");
        if let EventEnum::ERROR(response) = result {
            assert_eq!(response.params.peer_id, peer_info.peer_id);
            assert_eq!(response.params.token, peer_info.token);
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// care for the case PeerEvent API returns invalid json
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_200_but_recv_invalid_json() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "event": "EX",
                    "field": "invalid"
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await;
        assert!(result.is_err());
        // server called
        httpserver.assert();
    }

    /// API returns 400
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_400() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "PEERS_EVENTS",
                    "params": {
                        "errors": [
                              {
                                "field": "peer_id",
                                "message": "peer_id field is not specified"
                              }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_403() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 404
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_404() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_405() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_406() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/1.peers/peer_event
    #[tokio::test]
    async fn recv_408() {
        // set up parameters
        let peer_info = create_params();

        let path = format!("/peers/hoge/events?token={}", peer_info.token.as_str());
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, &peer_info);
        let result = task.await.expect("api does not return Ok(timeout)");
        assert_eq!(result, EventEnum::TIMEOUT);

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_delete_peer {
    use mockito::mock;

    use crate::peer::api::*;
    use crate::peer::formats::*;

    fn create_params() -> PeerInfo {
        let peer_id = PeerId::new("hoge");
        let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap();
        PeerInfo {
            peer_id: peer_id.clone(),
            token: token.clone(),
        }
    }

    /// A WebRTC Gateway returns 204, if it succeeds to delete a Peer Objec
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_204() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await;
        assert!(result.is_ok());

        // server called
        httpserver.assert();
    }

    /// API returns 400
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_400() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "PEERS_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "key",
                                "message": "key field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_403() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 404
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_404() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_405() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_406() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/1.peers/peer_destroy
    #[tokio::test]
    async fn recv_408() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_peer(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_status {
    use mockito::mock;

    use crate::error;
    use crate::peer::formats::*;

    fn create_params() -> PeerInfo {
        let peer_id = PeerId::new("hoge");
        let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap();
        PeerInfo {
            peer_id: peer_id.clone(),
            token: token.clone(),
        }
    }

    /// Status API returns json with 200 OK
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_200() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "peer_id": "hoge",
                "disconnected": false
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let status: PeerStatusMessage = task.await.expect("parse error");
        assert_eq!(status.peer_id, peer_info.peer_id);
        assert_eq!(status.disconnected, false);

        // server called
        httpserver.assert();
    }

    /// API returns 400
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_400() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "PEERS_STATUS",
                "params": {
                    "errors": [{
                        "field": "peer_id",
                        "message": "peer_id field is not specified"
                    }]
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_403() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 404
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_404() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_406() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/1.peers/peer_status
    #[tokio::test]
    async fn recv_408() {
        // set up parameters
        let peer_info = create_params();

        let path = format!(
            "/peers/{}/status?token={}",
            peer_info.peer_id.as_str(),
            peer_info.token.as_str()
        );
        // set up server mock
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, &peer_info);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}
