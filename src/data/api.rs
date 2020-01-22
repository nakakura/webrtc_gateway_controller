//FIXME
#![allow(dead_code)]

use futures::*;
use reqwest;
use reqwest::Client;
use serde_json::json;

use super::formats::*;
use crate::common;
use crate::error;

/// It access to the POST /data endpoint, and return its response.
/// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
/// If server returns 400, 405, 406, 408, create_data returns error
/// http://35.200.46.204/#/2.data/data
pub async fn create_data(base_url: &str) -> Result<CreatedResponse, error::ErrorEnum> {
    let api_url = format!("{}/data", base_url);
    let json = json!({});
    let api_call = || Client::new().post(&api_url).json(&json).send();
    let parser = |r: reqwest::Response| r.json::<CreatedResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// This function access to the DELETE /data endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_delete
pub async fn delete_data(base_url: &str, data_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/data/{}", base_url, data_id);
    let api_call = || Client::new().delete(&api_url).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// This function access to the POST /data/connections endpoint.
/// The API returns 202 Accepted, when a WebRTC Gateway succeed to start calling
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connections_create
pub async fn create_data_connection(
    base_url: &str,
    params: &CreateDataConnectionQuery,
) -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
    let api_url = format!("{}/data/connections", base_url);
    let api_call = || Client::new().post(&api_url).json(params).send();
    let parser =
        |r: reqwest::Response| r.json::<CreateDataConnectionResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::ACCEPTED, false, api_call, parser).await
}

/// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connection_close
pub async fn delete_data_connection(
    base_url: &str,
    data_connection_id: &str,
) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/data/connections/{}", base_url, data_connection_id);
    let api_call = || Client::new().delete(&api_url).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// This function access to the PUT data/connections/{data_connection_id} endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to start redirecting data received from neighbours
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connection_put
pub async fn redirect_data_connection(
    base_url: &str,
    data_connection_id: &str,
    redirect_data_params: &RedirectDataParams,
) -> Result<RedirectDataResponse, error::ErrorEnum> {
    let api_url = format!("{}/data/connections/{}", base_url, data_connection_id);
    let api_call = || {
        Client::new()
            .put(&api_url)
            .json(redirect_data_params)
            .send()
    };
    let parser = |r: reqwest::Response| r.json::<RedirectDataResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

/// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/status
pub async fn status(
    base_url: &str,
    data_connection_id: &str,
) -> Result<DataConnectionStatus, error::ErrorEnum> {
    let api_url = format!(
        "{}/data/connections/{}/status",
        base_url, data_connection_id
    );
    let api_call = || Client::new().get(&api_url).send();
    let parser = |r: reqwest::Response| r.json::<DataConnectionStatus>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

/// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
/// Fn event returns DataConnectionEventEnum::Timeout to listen event again.
/// When it receives 400, 403, 404, 405, 406, show errors.
/// http://35.200.46.204/#/2.data/events
pub async fn event(
    base_url: &str,
    data_connection_id: &str,
) -> Result<DataConnectionEventEnum, error::ErrorEnum> {
    let api_url = format!(
        "{}/data/connections/{}/events",
        base_url, data_connection_id
    );
    let api_call = || Client::new().get(&api_url).send();
    let parser = |r: reqwest::Response| r.json::<DataConnectionEventEnum>().map_err(Into::into);
    match common::api_access(reqwest::StatusCode::OK, true, api_call, parser).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            error::ErrorEnum::MyError { error: message } if message == "recv RequestTimeout" => {
                Ok(DataConnectionEventEnum::TIMEOUT)
            }
            e => Err(e),
        },
    }
}

#[cfg(test)]
mod test_create_data {
    use serde_json::json;

    use crate::data::formats::DataId;
    use crate::error;
    use helper::server;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| async move {
            if req.uri() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({
                    "data_id": "da-test",
                    "port": 50000,
                    "ip_v4": "127.0.0.1",
                });
                http::Response::builder()
                    .status(hyper::StatusCode::CREATED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.expect("event parse error");
        assert_eq!(result.data_id, DataId::new("da-test"));
        assert_eq!(result.port, 50000);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| async move {
            if req.uri().to_string() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "DATA_CREATE",
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 403, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_403() {
        let server = server::http(move |req| async move {
            if req.uri().to_string() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::FORBIDDEN)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 405, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_405() {
        let server = server::http(move |req| async move {
            if req.uri().to_string() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 406, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_406() {
        let server = server::http(move |req| async move {
            if req.uri().to_string() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 408, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_408() {
        let server = server::http(move |req| async move {
            if req.uri().to_string() == "/data" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_delete_data {
    use serde_json::json;

    use crate::data::formats::DataId;
    use crate::error;
    use helper::server;

    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_204() {
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NO_CONTENT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, ());
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_400() {
        let data_id = DataId::new("da-test");
        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 403, delete_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_403() {
        let data_id = DataId::new("da-test");
        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 405, delete_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_405() {
        let data_id = DataId::new("da-test");
        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
                            }
                        ]
                    }
                });
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 406, delete_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_406() {
        let data_id = DataId::new("da-test");
        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
                            }
                        ]
                    }
                });
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 408, delete_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_408() {
        let data_id = DataId::new("da-test");
        let server = server::http(move |req| async move {
            if req.uri() == "/data/da-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
                            }
                        ]
                    }
                });
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_create_data_connection {
    use futures::*;
    use serde_json::json;

    use crate::data::formats::*;
    use crate::error;
    use crate::{PeerId, Token};
    use helper::server;

    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_202() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |mut req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let mut full: Vec<u8> = Vec::new();
                while let Some(item) = req.body_mut().next().await {
                    full.extend(&*item.unwrap());
                }
                let _peer_options: CreateDataConnectionQuery =
                    serde_json::from_slice(&full).expect("PeerOptions parse error");
                let json = json!({
                    "command_type": "PEERS_CONNECT",
                    "params": {
                        "data_connection_id": "dc-test"
                    }
                });
                http::Response::builder()
                    .status(hyper::StatusCode::ACCEPTED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.expect("parse error");
        assert_eq!(result.params.data_connection_id.as_str(), "dc-test");
    }

    /// It returns 400 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_400() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "DATA_CONNECTION_CREATE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// It returns 403 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_403() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::FORBIDDEN)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// It returns 405 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_405() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// It returns 406 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_406() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// It returns 408 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_408() {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataIdWrapper { data_id: data_id };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id),
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_delete_data_connection {
    use serde_json::json;

    use crate::error;
    use crate::DataConnectionId;
    use helper::server;

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
    /// It returns 400, 403, 404, 405, 406, 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_202() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NO_CONTENT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, ());
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_400() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "DATA_CONNECTION_DELETE",
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_403() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::FORBIDDEN)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_404() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_405() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_406() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_408() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::DELETE {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_redirect_data_connection {
    use futures::*;
    use serde_json::json;

    use crate::data::formats::*;
    use crate::error;
    use crate::DataConnectionId;
    use helper::server;

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to start redirecting data received from neighbours
    /// http://35.200.46.204/#/2.data/data_connection_put
    #[tokio::test]
    async fn recv_202() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |mut req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let mut full: Vec<u8> = Vec::new();
                while let Some(item) = req.body_mut().next().await {
                    full.extend(&*item.unwrap());
                }
                let redirect_data_params: RedirectDataParams =
                    serde_json::from_slice(&full).expect("PeerOptions parse error");
                assert_eq!(
                    redirect_data_params
                        .feed_params
                        .clone()
                        .expect("no data params")
                        .data_id,
                    DataId::new("da-test")
                );
                assert_eq!(
                    redirect_data_params
                        .redirect_params
                        .clone()
                        .expect("no redirect params")
                        .ip_v4,
                    Some(ip_v4.to_string())
                );
                assert_eq!(
                    redirect_data_params.redirect_params.expect("no port").port,
                    port
                );

                let json = json!({
                    "command_type": "DATA_CONNECTION_PUT",
                    "data_id": "da-50a32bab-b3d9-4913-8e20-f79c90a6a211"
                });
                http::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.expect("parse error");
        assert_eq!(
            result.data_id,
            DataId::new("da-50a32bab-b3d9-4913-8e20-f79c90a6a211")
        );
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_400() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({
                    "command_type": "DATA_CONNECTION_PUT",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_403() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::FORBIDDEN)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_404() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_405() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_406() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_408() {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| async move {
            println!("req.uri {:?}", req.uri());
            if req.uri() == "/data/connections/dc-test" && req.method() == reqwest::Method::PUT {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let data_id_obj = DataIdWrapper { data_id: data_id };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: Some(data_id_obj),
            redirect_params: Some(redirect_params),
        };

        let addr = format!("http://{}", server.addr());
        let task = super::redirect_data_connection(
            &addr,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
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

    use crate::error;
    use crate::DataConnectionId;
    use helper::server;

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_200() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "buffersize": 0,
                    "label": "c_3q8ymsw7n9c4s0ibzx8jymygb9",
                    "metadata": "",
                    "open": true,
                    "reliable": true,
                    "remote_id": "data_caller",
                    "serialization": "BINARY",
                    "type": "DATA"
                });
                http::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result.open, true);
        assert_eq!(result.reliable, true);
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_400() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "command_type": "DATA_CONNECTION_STATUS",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_403() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "command_type": "DATA_CONNECTION_STATUS",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_404() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_405() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_406() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 408 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_408() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/status"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_event {
    use serde_json::json;

    use crate::data::formats::DataConnectionEventEnum;
    use crate::error;
    use crate::DataConnectionId;
    use helper::server;

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_open() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "event": "OPEN"
                });
                http::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, DataConnectionEventEnum::OPEN);
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_close() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "event": "CLOSE"
                });
                http::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, DataConnectionEventEnum::CLOSE);
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_error() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "event": "ERROR",
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(
            result,
            DataConnectionEventEnum::ERROR {
                error_message: "error".to_string()
            }
        );
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 400, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_400() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({
                    "command_type": "DATA_CONNECTION_EVENTS",
                    "params": {
                        "errors": [
                            {
                                "field": "string",
                                "message": "string"
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 403, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_403() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::FORBIDDEN)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 404, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_404() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 405, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_405() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 406, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_406() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::NOT_ACCEPTABLE)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// Fn event returns DataConnectionEventEnum::Timeout to listen event again.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_408() {
        let data_connection_id = DataConnectionId::new("dc-test");

        let server = server::http(move |req| async move {
            if req.uri() == "/data/connections/dc-test/events"
                && req.method() == reqwest::Method::GET
            {
                let json = json!({});
                http::Response::builder()
                    .status(hyper::StatusCode::REQUEST_TIMEOUT)
                    .header("Content-type", "application/json")
                    .body(hyper::Body::from(json.to_string()))
                    .unwrap()
            } else {
                unreachable!();
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::event(&addr, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, DataConnectionEventEnum::TIMEOUT);
    }
}
