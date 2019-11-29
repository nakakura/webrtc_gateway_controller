mod data;

use reqwest;
use reqwest::Client;

use crate::error;
use crate::peer::data::PeerErrorResponse;
use data::*;

/// Fn create_media access to the POST /media endpoint, and return its response.
/// If the API returns values with 201 Created, create_data returns the information as CreateMediaResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/2.data/data
pub async fn create_media(
    base_url: &str,
    is_video: bool,
) -> Result<CreateMediaResponse, error::ErrorEnum> {
    let api_url = format!("{}/media", base_url);
    let option = CreateMediaOptions { is_video: is_video };
    let res = Client::new().post(&api_url).json(&option).send().await?;
    match res.status() {
        http::status::StatusCode::CREATED => res
            .json::<data::CreateMediaResponse>()
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

/// Fn delete_media access to the DELETE /media endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/streams_delete
pub async fn delete_media(base_url: &str, media_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/{}", base_url, media_id);
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

/// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
/// If the API returns values with 201 Created, it returns CreateRtcpResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_create
pub async fn create_rtcp(base_url: &str) -> Result<CreateRtcpResponse, error::ErrorEnum> {
    let api_url = format!("{}/media/rtcp", base_url);
    let res = Client::new().post(&api_url).send().await?;
    match res.status() {
        http::status::StatusCode::CREATED => {
            res.json::<CreateRtcpResponse>().await.map_err(Into::into)
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

/// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_delete
pub async fn delete_rtcp(base_url: &str, rtcp_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/rtcp/{}", base_url, rtcp_id);
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

/// Fn call access to the POST /media/connections endpoint.
/// If the API returns values with 202 Accepted, it returns CallResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_connection_create
pub async fn call(
    base_url: &str,
    call_params: &CallParameters,
) -> Result<CallResponse, error::ErrorEnum> {
    let api_url = format!("{}/media/connections", base_url);
    let res = Client::new()
        .post(&api_url)
        .json(call_params)
        .send()
        .await?;
    match res.status() {
        http::status::StatusCode::ACCEPTED => res.json::<CallResponse>().await.map_err(Into::into),
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
mod test_create_media {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;
    use crate::media::data::CreateMediaOptions;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_video() {
        let server = server::http(move |mut req| {
            async move {
                if req.uri() == "/media" && req.method() == reqwest::Method::POST {
                    let mut full: Vec<u8> = Vec::new();
                    while let Some(item) = req.body_mut().next().await {
                        full.extend(&*item.unwrap());
                    }
                    let media_options: CreateMediaOptions =
                        serde_json::from_slice(&full).expect("PeerOptions parse error");

                    let media_id = if media_options.is_video {
                        "vi-test"
                    } else {
                        "au-test"
                    };
                    let json = json!({
                        "media_id": media_id,
                        "port": 10001,
                        "ip_v4": "127.0.0.1"
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
        let task = super::create_media(&addr, true);
        let result = task.await.expect("event parse error");
        assert_eq!(result.media_id, "vi-test".to_string());
        assert_eq!(result.port, 10001);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_audio() {
        let server = server::http(move |mut req| {
            async move {
                if req.uri() == "/media" && req.method() == reqwest::Method::POST {
                    let mut full: Vec<u8> = Vec::new();
                    while let Some(item) = req.body_mut().next().await {
                        full.extend(&*item.unwrap());
                    }
                    let media_options: CreateMediaOptions =
                        serde_json::from_slice(&full).expect("PeerOptions parse error");

                    let media_id = if media_options.is_video {
                        "vi-test"
                    } else {
                        "au-test"
                    };
                    let json = json!({
                        "media_id": media_id,
                        "port": 10001,
                        "ip_v4": "127.0.0.1"
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
        let task = super::create_media(&addr, false);
        let result = task.await.expect("event parse error");
        assert_eq!(result.media_id, "au-test".to_string());
        assert_eq!(result.port, 10001);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| {
            async move {
                if req.uri().to_string() == "/media" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "MEDIA_CREATE",
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
        let task = super::create_media(&addr, true);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 403, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_403() {
        let server = server::http(move |req| {
            async move {
                if req.uri().to_string() == "/media" && req.method() == reqwest::Method::POST {
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
        let task = super::create_media(&addr, true);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 405, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_405() {
        let server = server::http(move |req| {
            async move {
                if req.uri().to_string() == "/media" && req.method() == reqwest::Method::POST {
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
        let task = super::create_media(&addr, true);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 406, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_406() {
        let server = server::http(move |req| {
            async move {
                if req.uri().to_string() == "/media" && req.method() == reqwest::Method::POST {
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
        let task = super::create_media(&addr, true);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// If server returns 408, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_408() {
        let server = server::http(move |req| {
            async move {
                if req.uri().to_string() == "/media" && req.method() == reqwest::Method::POST {
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
        let task = super::create_media(&addr, true);
        let result = task.await.err().expect("parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_delete_media {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_204() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_400() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
                    let json = json!({
                        "command_type": "MEDIA_DELETE",
                        "params": {
                            "errors": [
                                {
                                    "field": "media_id",
                                    "message": "media_id field is not specified"
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_403() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 404, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_404() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_405() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_406() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_408() {
        let media_id = "test-media_id";
        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/{}", media_id);
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
        let task = super::delete_media(&addr, media_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_create_rtcp {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If the API returns values with 201 Created, it returns CreateRtcpResponse
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "rtcp_id": "rc-test",
                        "port": 10003,
                        "ip_v4": "127.0.0.1"
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
        let task = super::create_rtcp(&addr);
        let result = task.await.expect("event parse error");
        assert_eq!(result.rtcp_id, "rc-test");
        assert_eq!(result.port, 10003);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "MEDIA_DELETE",
                        "params": {
                            "errors": [
                                {
                                    "field": "media_id",
                                    "message": "media_id field is not specified"
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
        let task = super::create_rtcp(&addr);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_403() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
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
        let task = super::create_rtcp(&addr);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_405() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
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
        let task = super::create_rtcp(&addr);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_406() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
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
        let task = super::create_rtcp(&addr);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_408() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/rtcp" && req.method() == reqwest::Method::POST {
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
        let task = super::create_rtcp(&addr);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_delete_rtcp {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/media_rtcp_delete
    #[tokio::test]
    async fn recv_201() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_400() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
                if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
                    let json = json!({
                        "command_type": "MEDIA_DELETE",
                        "params": {
                            "errors": [
                                {
                                    "field": "media_id",
                                    "message": "media_id field is not specified"
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_403() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 404, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_404() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_405() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_406() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_408() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| {
            async move {
                let uri = format!("/media/rtcp/{}", rtcp_id);
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
        let task = super::delete_rtcp(&addr, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_call {
    use serde_json::json;

    use crate::error;
    use crate::helper::*;
    use crate::media::data::CallParameters;

    /// Fn call access to the POST /media/connections endpoint.
    /// If the API returns values with 202 Accepted, it returns CallResponse
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "PEERS_CALL",
                        "params": {
                            "media_connection_id": "mc-test"
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.expect("event parse error");
        assert_eq!(result.params.media_connection_id, "mc-test".to_string());
    }

    /// Fn call access to the POST /media/connections endpoint.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
                    let json = json!({
                        "command_type": "MEDIA_CONNECTION_CREATE",
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn call access to the POST /media/connections endpoint.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_403() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn call access to the POST /media/connections endpoint.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_405() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn call access to the POST /media/connections endpoint.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_406() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn call access to the POST /media/connections endpoint.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_408() {
        let server = server::http(move |req| {
            async move {
                if req.uri() == "/media/connections" && req.method() == reqwest::Method::POST {
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

        let call_params = CallParameters {
            peer_id: "peer_id".to_string(),
            token: "pt-test".to_string(),
            target_id: "target_id".to_string(),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}
