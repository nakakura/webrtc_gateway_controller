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

#[cfg(test)]
mod test_create_data {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn test_create_data_200() {
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
    async fn status_400() {
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
    async fn status_403() {
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
    async fn status_405() {
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
    async fn status_406() {
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
    async fn status_408() {
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
