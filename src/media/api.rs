//FIXME
#![allow(dead_code)]

use futures::*;
use reqwest;
use reqwest::Client;

use super::formats::*;
use crate::common::{self, *};
use crate::error;

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
    let api_call = || Client::new().post(&api_url).json(&option).send();
    let parser = |r: reqwest::Response| r.json::<CreateMediaResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// Fn delete_media access to the DELETE /media endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/streams_delete
pub async fn delete_media(base_url: &str, media_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/{}", base_url, media_id);
    let api_call = || Client::new().delete(&api_url).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
/// If the API returns values with 201 Created, it returns CreateRtcpResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_create
pub async fn create_rtcp(base_url: &str) -> Result<CreateRtcpResponse, error::ErrorEnum> {
    let api_url = format!("{}/media/rtcp", base_url);
    let api_call = || Client::new().post(&api_url).send();
    let parser = |r: reqwest::Response| r.json::<CreateRtcpResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_delete
pub async fn delete_rtcp(base_url: &str, rtcp_id: &str) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/rtcp/{}", base_url, rtcp_id);
    let api_call = || Client::new().delete(&api_url).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn create_call access to the POST /media/connections endpoint.
/// If the API returns values with 202 Accepted, it returns CallResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_connection_create
pub async fn create_call(
    base_url: &str,
    call_params: &CallParameters,
) -> Result<CallResponse, error::ErrorEnum> {
    let api_url = format!("{}/media/connections", base_url);
    let api_call = || Client::new().post(&api_url).json(call_params).send();
    let parser = |r: reqwest::Response| r.json::<CallResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::ACCEPTED, false, api_call, parser).await
}

/// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_close
pub async fn delete_call(
    base_url: &str,
    media_connection_id: &str,
) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/connections/{}", base_url, media_connection_id);
    let api_call = || Client::new().delete(&api_url).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
/// If the API returns values with 202 Accepted, it returns AnswerResponse
/// If server returns 400, 403, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_answer
pub async fn answer(
    base_url: &str,
    media_connection_id: &str,
    params: &AnswerParameters,
) -> Result<AnswerResponse, error::ErrorEnum> {
    let api_url = format!(
        "{}/media/connections/{}/answer",
        base_url, media_connection_id
    );
    let api_call = || Client::new().post(&api_url).json(params).send();
    let parser = |r: reqwest::Response| r.json::<AnswerResponse>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::ACCEPTED, true, api_call, parser).await
}

/// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
/// If the API returns values with 201 Accepted, it returns ()
/// If server returns 400, 403, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_pli
pub async fn pli(
    base_url: &str,
    media_connection_id: &str,
    params: &RedirectParams,
) -> Result<(), error::ErrorEnum> {
    let api_url = format!("{}/media/connections/{}/pli", base_url, media_connection_id);
    let api_call = || Client::new().post(&api_url).json(params).send();
    let parser = |_| future::ok(());
    common::api_access(reqwest::StatusCode::CREATED, true, api_call, parser).await
}

/// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
/// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
/// If server returns 400, 403, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_event
pub async fn events(
    base_url: &str,
    media_connection_id: &str,
) -> Result<MediaConnectionEventEnum, error::ErrorEnum> {
    let api_url = format!(
        "{}/media/connections/{}/events",
        base_url, media_connection_id
    );
    let api_call = || Client::new().get(&api_url).send();
    let parser = |r: reqwest::Response| r.json::<MediaConnectionEventEnum>().map_err(Into::into);
    match common::api_access(reqwest::StatusCode::OK, true, api_call, parser).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            error::ErrorEnum::MyError { error: message } if message == "recv RequestTimeout" => {
                Ok(MediaConnectionEventEnum::TIMEOUT)
            }
            e => Err(e),
        },
    }
}

/// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
/// If the API returns values with 200 Ok, it returns MediaConnectionStatus
/// If server returns 400, 403, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_status
pub async fn status(
    base_url: &str,
    media_connection_id: &str,
) -> Result<MediaConnectionStatus, error::ErrorEnum> {
    let api_url = format!(
        "{}/media/connections/{}/status",
        base_url, media_connection_id
    );
    let api_call = || Client::new().get(&api_url).send();
    let parser = |r: reqwest::Response| r.json::<MediaConnectionStatus>().map_err(Into::into);
    common::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

#[cfg(test)]
mod test_create_media {
    use futures::*;
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use crate::media::formats::*;
    use helper::server;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_video() {
        let server = server::http(move |mut req| async move {
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_media(&addr, true);
        let result = task.await.expect("event parse error");
        assert_eq!(result.media_id, MediaId::new("vi-test"));
        assert_eq!(result.port, 10001);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_audio() {
        let server = server::http(move |mut req| async move {
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::create_media(&addr, false);
        let result = task.await.expect("event parse error");
        assert_eq!(result.media_id, MediaId::new("au-test"));
        assert_eq!(result.port, 10001);
        assert_eq!(result.ip_v4, Some("127.0.0.1".to_string()));
    }

    /// If server returns 400, create_data returns error
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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

    use crate::common::*;
    use crate::error;
    use helper::server;

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_204() {
        let media_id = "test-media_id";
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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

    use crate::common::{self, *};
    use crate::error;
    use helper::server;

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If the API returns values with 201 Created, it returns CreateRtcpResponse
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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
        let server = server::http(move |req| async move {
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

    use crate::common::*;
    use crate::common::{self, *};
    use crate::error;
    use helper::server;

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/media_rtcp_delete
    #[tokio::test]
    async fn recv_201() {
        let rtcp_id = "rc-test";

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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

        let server = server::http(move |req| async move {
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
mod test_create_call {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use crate::media::formats::CallParameters;
    use helper::server;

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If the API returns values with 202 Accepted, it returns CallResponse
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_201() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.expect("event parse error");
        assert_eq!(result.params.media_connection_id, "mc-test".to_string());
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_400() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_403() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_405() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_406() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_408() {
        let server = server::http(move |req| async move {
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
        });

        let call_params = CallParameters {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        };

        let addr = format!("http://{}", server.addr());
        let task = super::create_call(&addr, &call_params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_delete_call {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use helper::server;

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_204() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_400() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::DELETE {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "media_connection_id",
                                "message": "media_connection_id field is not specified"
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
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_403() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_404() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_405() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_406() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_clos
    #[tokio::test]
    async fn recv_408() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::delete_call(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_answer {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use crate::media::formats::*;
    use helper::server;

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If the API returns values with 202 Accepted, it returns AnswerResponse
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_202() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_ANSWER",
                    "params": {
                        "video_port": 10011,
                        "video_id": "vi-test",
                        "audio_port": 10021,
                        "audio_id": "au-test"
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

        let addr = format!("http://{}", server.addr());
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.expect("event parse error");
        assert_eq!(result.params.video_port, Some(10011));
        assert_eq!(result.params.video_id, Some(MediaId::new("vi-test")));
        assert_eq!(result.params.audio_port, Some(10021));
        assert_eq!(result.params.audio_id, Some(MediaId::new("au-test")));
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_400() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_ANSWER",
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
        });

        let addr = format!("http://{}", server.addr());
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_403() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_404() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_405() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_406() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_408() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/answer", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = AnswerParameters {
            constraints: None,
            redirect_params: None,
        };
        let task = super::answer(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_pli {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use crate::media::formats::*;
    use helper::server;

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If the API returns values with 201 Accepted, it returns ()
    /// http://35.200.46.204/#/3.media/media_connection_pli
    #[tokio::test]
    async fn recv_202() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
                let json = json!({});
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_400() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_PLI",
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
        });

        let addr = format!("http://{}", server.addr());
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_403() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_404() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_405() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_406() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_408() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/pli", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::POST {
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
        let params = RedirectParams {
            port: 10001,
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: None,
        };

        let task = super::pli(&addr, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test_events {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use crate::media::formats::*;
    use helper::server;

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_ready() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({"event": "READY"});
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
        let task = super::events(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, MediaConnectionEventEnum::READY);
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_stream() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({"event": "STREAM"});
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
        let task = super::events(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, MediaConnectionEventEnum::STREAM);
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_close() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({"event": "CLOSE"});
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
        let task = super::events(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, MediaConnectionEventEnum::CLOSE);
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_error() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({"event": "ERROR", "error_message": "hoge"});
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
        let task = super::events(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(
            result,
            MediaConnectionEventEnum::ERROR {
                error_message: "hoge".to_string()
            }
        );
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_400() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_EVENTS",
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_403() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_404() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_405() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_406() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_408() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/events", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::events(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, MediaConnectionEventEnum::TIMEOUT);
    }
}

#[cfg(test)]
mod test_status {
    use serde_json::json;

    use crate::common::*;
    use crate::error;
    use helper::server;

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionStatus
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_200() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({
                    "metadata": "",
                    "open": true,
                    "remote_id": "media_caller",
                    "ssrc": [
                        {
                            "media_id": "au-test",
                            "ssrc": 2
                        },
                        {
                            "media_id": "vi-test",
                            "ssrc": 3
                        }
                    ]
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
        let task = super::status(&addr, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result.open, true);
        assert_eq!(result.ssrc.len(), 2);
        assert_eq!(result.ssrc[0].media_id.as_str(), "au-test");
        assert_eq!(result.ssrc[0].ssrc, 2);
        assert_eq!(result.ssrc[1].media_id.as_str(), "vi-test");
        assert_eq!(result.ssrc[1].ssrc, 3);
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_400() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
            if req.uri().to_string() == uri && req.method() == reqwest::Method::GET {
                let json = json!({
                    "command_type": "MEDIA_CONNECTION_STATUS",
                    "params": {
                        "errors": [
                            {
                                "field": "media_connection_id",
                                "message": "media_connection_id is not exists."
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
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_403() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_404() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_405() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_406() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_408() {
        let media_connection_id = "mc-test";

        let server = server::http(move |req| async move {
            let uri = format!("/media/connections/{}/status", media_connection_id);
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
        });

        let addr = format!("http://{}", server.addr());
        let task = super::status(&addr, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let error::ErrorEnum::MyError { error: _e } = result {
        } else {
            unreachable!();
        }
    }
}
