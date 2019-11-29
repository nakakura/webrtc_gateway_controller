mod data;

use reqwest;
use reqwest::Client;

use crate::error;
use crate::peer::data::PeerErrorResponse;
use data::*;

/// It access to the POST /data endpoint, and return its response.
/// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
/// If server returns 400, 405, 406, 408, create_data returns error
/// http://35.200.46.204/#/2.data/data
pub async fn create_data(base_url: &str) -> Result<CreatedResponse, error::ErrorEnum> {
    let api_url = format!("{}/data", base_url);
    let res = Client::new().post(&api_url).send().await?;
    match res.status() {
        http::status::StatusCode::CREATED => res
            .json::<data::CreatedResponse>()
            .await
            .map_err(Into::into),
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

/// This function access to the DELETE /data endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_delete
pub async fn delete_data(base_url: &str, data_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/data/{}", base_url, data_id);
    let res = Client::new().delete(&api_url).send().await?;

    match res.status() {
        http::status::StatusCode::NO_CONTENT => Ok(()),
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

/// This function access to the POST /data/connections endpoint.
/// The API returns 202 Accepted, when a WebRTC Gateway succeed to start calling
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connections_create
pub async fn create_data_connection(
    base_url: &str,
    params: &CreateDataConnectionQuery,
) -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
    let api_url = format!("{}/data/connections", base_url);
    let res = Client::new().post(&api_url).json(params).send().await?;

    match res.status() {
        http::status::StatusCode::ACCEPTED => res
            .json::<CreateDataConnectionResponse>()
            .await
            .map_err(Into::into),
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
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
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

/// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connection_close
pub async fn delete_data_connection(
    base_url: &str,
    data_connection_id: &str,
) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/data/connections/{}", base_url, data_connection_id);
    let res = Client::new().delete(&api_url).send().await?;

    match res.status() {
        http::status::StatusCode::NO_CONTENT => Ok(()),
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
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
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
    let res = Client::new()
        .put(&api_url)
        .json(redirect_data_params)
        .send()
        .await?;

    match res.status() {
        http::status::StatusCode::OK => {
            res.json::<RedirectDataResponse>().await.map_err(Into::into)
        }
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
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
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
mod test_create_data {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_data(&addr);
        let result = task.await.expect("event parse error");
        assert_eq!(result.data_id, "da-test".to_string());
        assert_eq!(result.port, 50000);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| {
            async move {
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
        let server = server::http(move |req| {
            async move {
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
        let server = server::http(move |req| {
            async move {
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
        let server = server::http(move |req| {
            async move {
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
        let server = server::http(move |req| {
            async move {
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

mod test_delete_data {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_204() {
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
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
        let task = super::delete_data(&addr, data_id);
        let result = task.await.expect("parse error");
        assert_eq!(result, ());
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_400() {
        let data_id = "da-test";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id);
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
        let data_id = "da-test";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id);
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
        let data_id = "da-test";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id);
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
        let data_id = "da-test";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id);
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
        let data_id = "da-test";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/{}", data_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data(&addr, data_id);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

mod test_create_data_connection {
    use serde_json::json;

    use crate::data::data::*;
    use crate::error;
    use crate::helper::*;

    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_202() {
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |mut req| {
            async move {
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
            }
        });

        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.expect("parse error");
        assert_eq!(result.params.data_connection_id, "dc-test");
    }

    /// It returns 400 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_400() {
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
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
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
            redirect_params: None,
        };
        let task = super::create_data_connection(&addr, &query);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// It returns 404 to show errors
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_404() {
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
                if req.uri() == "/data/connections" && req.method() == reqwest::Method::POST {
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
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
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
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
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
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
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
        let peer_id = "peer_id";
        let token = "test-token";
        let target_id = "target_id";
        let data_id = "da-test";

        let server = server::http(move |req| {
            async move {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let data_id = DataId {
            data_id: data_id.to_string(),
        };
        let query = CreateDataConnectionQuery {
            peer_id: peer_id.to_string(),
            token: token.to_string(),
            options: None,
            target_id: target_id.to_string(),
            params: data_id,
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

mod test_delete_data_connection {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
    /// It returns 400, 403, 404, 405, 406, 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_202() {
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
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
        let task = super::delete_data_connection(&addr, data_connection_id);
        let result = task.await.expect("parse error");
        assert_eq!(result, ());
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_400() {
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
            }
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_data_connection(&addr, data_connection_id);
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
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
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
        let task = super::delete_data_connection(&addr, data_connection_id);
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
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
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
        let task = super::delete_data_connection(&addr, data_connection_id);
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
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
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
        let task = super::delete_data_connection(&addr, data_connection_id);
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
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
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
        let task = super::delete_data_connection(&addr, data_connection_id);
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
        let data_connection_id = "dc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
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
        let task = super::delete_data_connection(&addr, data_connection_id);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

mod test_redirect_data_connection {
    use serde_json::json;

    use crate::data::data::*;
    use crate::error;
    use crate::helper::*;

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to start redirecting data received from neighbours
    /// http://35.200.46.204/#/2.data/data_connection_put
    #[tokio::test]
    async fn recv_202() {
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |mut req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
                    let mut full: Vec<u8> = Vec::new();
                    while let Some(item) = req.body_mut().next().await {
                        full.extend(&*item.unwrap());
                    }
                    let redirect_data_params: RedirectDataParams =
                        serde_json::from_slice(&full).expect("PeerOptions parse error");
                    assert_eq!(redirect_data_params.feed_params.data_id, data_id);
                    assert_eq!(
                        redirect_data_params.redirect_params.ip_v4,
                        Some(ip_v4.to_string())
                    );
                    assert_eq!(redirect_data_params.redirect_params.port, port);

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
            }
        });

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
        let result = task.await.expect("parse error");
        assert_eq!(
            result.data_id,
            "da-50a32bab-b3d9-4913-8e20-f79c90a6a211".to_string()
        );
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_400() {
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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
            }
        });

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
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
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
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
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
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
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
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
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
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
        let data_id = "da-test";
        let data_connection_id = "dc-test";
        let ip_v4 = "127.0.0.1";
        let port = 10001;

        let server = server::http(move |req| {
            async move {
                let uri = format!("/data/connections/{}", data_connection_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::PUT {
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

        let data_id_obj = DataId {
            data_id: data_id.to_string(),
        };
        let redirect_params = RedirectParams {
            ip_v4: Some(ip_v4.to_string()),
            ip_v6: None,
            port: port,
        };
        let redirect_data_params = RedirectDataParams {
            feed_params: data_id_obj,
            redirect_params: redirect_params,
        };

        let addr = format!("http://{}", server.addr());
        let task =
            super::redirect_data_connection(&addr, &data_connection_id, &redirect_data_params);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}
