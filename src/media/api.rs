use futures::*;
use reqwest;
use reqwest::Client;

use super::formats::*;
use crate::common::api::api_access;
use crate::common::formats::{PhantomId, SocketInfo};
use crate::error;
use crate::common::api_refactor;
use crate::new_error;

/// Fn create_media access to the POST /media endpoint, and return its response.
/// If the API returns values with 201 Created, create_data returns the information as CreateMediaResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/2.data/data
pub(crate) async fn create_media(
    base_url: &str,
    is_video: bool,
) -> Result<SocketInfo<MediaId>, new_error::Error> {
    let api_url = format!("{}/media", base_url);
    let option = CreateMediaOptions { is_video: is_video };
    let api_call = || Client::new().post(&api_url).json(&option).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<SocketInfo<MediaId>>().map_err(Into::into);
    api_refactor::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// Fn delete_media access to the DELETE /media endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/streams_delete
pub(crate) async fn delete_media(base_url: &str, media_id: &str) -> Result<(), new_error::Error> {
    let api_url = format!("{}/media/{}", base_url, media_id);
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api_refactor::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
/// If the API returns values with 201 Created, it returns CreateRtcpResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_create
pub(crate) async fn create_rtcp(base_url: &str) -> Result<SocketInfo<RtcpId>, new_error::Error> {
    let api_url = format!("{}/media/rtcp", base_url);
    let api_call = || Client::new().post(&api_url).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<SocketInfo<RtcpId>>().map_err(Into::into);
    api_refactor::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_rtcp_delete
pub(crate) async fn delete_rtcp(base_url: &str, rtcp_id: &str) -> Result<(), new_error::Error> {
    let api_url = format!("{}/media/rtcp/{}", base_url, rtcp_id);
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api_refactor::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn create_call access to the POST /media/connections endpoint.
/// If the API returns values with 202 Accepted, it returns CallResponse
/// If server returns 400, 405, 406, 408, create_media returns error
/// http://35.200.46.204/#/3.media/media_connection_create
pub(crate) async fn create_call(
    base_url: &str,
    call_params: &CallQuery,
) -> Result<CallResponse, new_error::Error> {
    let api_url = format!("{}/media/connections", base_url);
    let api_call = || Client::new().post(&api_url).json(call_params).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<CallResponse>().map_err(Into::into);
    api_refactor::api_access(reqwest::StatusCode::ACCEPTED, false, api_call, parser).await
}

/// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
/// If the API returns values with 204 No Content
/// If server returns 400, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_close
pub(crate) async fn delete_call(
    base_url: &str,
    media_connection_id: &str,
) -> Result<(), new_error::Error> {
    let api_url = format!("{}/media/connections/{}", base_url, media_connection_id);
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api_refactor::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
/// If the API returns values with 202 Accepted, it returns AnswerResponse
/// If server returns 400, 403, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_answer
pub(crate) async fn answer(
    base_url: &str,
    media_connection_id: &str,
    params: &AnswerQuery,
) -> Result<AnswerResponse, new_error::Error> {
    let api_url = format!(
        "{}/media/connections/{}/answer",
        base_url, media_connection_id
    );
    let api_call = || Client::new().post(&api_url).json(params).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<AnswerResponse>().map_err(Into::into);
    api_refactor::api_access(reqwest::StatusCode::ACCEPTED, true, api_call, parser).await
}

/// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
/// If the API returns values with 201 Accepted, it returns ()
/// If server returns 400, 403, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_pli
pub(crate) async fn pli(
    base_url: &str,
    media_connection_id: &str,
    params: &SocketInfo<PhantomId>,
) -> Result<(), new_error::Error> {
    let api_url = format!("{}/media/connections/{}/pli", base_url, media_connection_id);
    let api_call = || Client::new().post(&api_url).json(params).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api_refactor::api_access(reqwest::StatusCode::CREATED, true, api_call, parser).await
}

/// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
/// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
/// If server returns 400, 403, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_event
pub(crate) async fn event(
    base_url: &str,
    media_connection_id: &str,
) -> Result<EventEnum, new_error::Error> {
    let api_url = format!(
        "{}/media/connections/{}/events",
        base_url, media_connection_id
    );
    let api_call = || Client::new().get(&api_url).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<EventEnum>().map_err(Into::into);
    match api_refactor::api_access(reqwest::StatusCode::OK, true, api_call, parser).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            new_error::Error::LocalError(message) if message == "recv RequestTimeout" => {
                Ok(EventEnum::TIMEOUT)
            }
            e => Err(e.into()),
        },
    }
}

/// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
/// If the API returns values with 200 Ok, it returns MediaConnectionStatus
/// If server returns 400, 403, 404, 405, 406, 408, it returns error
/// http://35.200.46.204/#/3.media/media_connection_status
pub(crate) async fn status(
    base_url: &str,
    media_connection_id: &str,
) -> Result<MediaConnectionStatus, new_error::Error> {
    let api_url = format!(
        "{}/media/connections/{}/status",
        base_url, media_connection_id
    );
    let api_call = || Client::new().get(&api_url).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<MediaConnectionStatus>().map_err(Into::into);
    api_refactor::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

#[cfg(test)]
mod test_create_media {
    use mockito::mock;

    use crate::common::formats::SerializableSocket;
    use crate::error;
    use crate::new_error;
    use crate::media::formats::*;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_video() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "media_id": "vi-4d053831-5dc2-461b-a358-d062d6115216",
                "port": 10001,
                "ip_v4": "127.0.0.1"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, true);
        let result = task.await.expect("event parse error");
        assert_eq!(
            result.get_id().unwrap(),
            MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216")
        );
        assert_eq!(result.port(), 10001);
        assert_eq!(result.ip().to_string(), String::from("127.0.0.1"));

        // server called
        httpserver.assert();
    }

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_201_audio() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": false
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "media_id": "au-4d053831-5dc2-461b-a358-d062d6115216",
                "port": 10001,
                "ip_v4": "127.0.0.1"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, false);
        let result = task.await.expect("event parse error");
        assert_eq!(
            result.get_id().unwrap(),
            MediaId::new("au-4d053831-5dc2-461b-a358-d062d6115216")
        );
        assert_eq!(result.port(), 10001);
        assert_eq!(result.ip().to_string(), String::from("127.0.0.1"));

        // server called
        httpserver.assert();
    }

    /// API returns 400
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_400() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CREATE",
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
        let task = super::create_media(&url, true);
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_403() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, true);
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_405() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, true);
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_406() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, true);
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/3.media/media
    #[tokio::test]
    async fn recv_408() {
        // set up server mock
        let httpserver = mock("POST", "/media")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                    "is_video": true
                }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_media(&url, true);
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_delete_media {
    use mockito::mock;

    use crate::error;
    use crate::new_error;
    use crate::media::formats::*;

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_204() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_400() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "MEDIA_DELETE",
                "params": {
                    "errors": [{
                        "field": "media_id",
                        "message": "media_id field is not specified"
                    }]
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_403() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 404, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_404() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_405() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_406() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_media access to the DELETE /media endpoint, and return its response.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/streams_delete
    #[tokio::test]
    async fn recv_408() {
        // set up parameters
        let media_id = MediaId::new("vi-4d053831-5dc2-461b-a358-d062d6115216");
        let path = format!("/media/{}", media_id.as_str());

        // set up server mock
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_media(&url, media_id.as_str());
        let result = task.await.err().expect("parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_create_rtcp {
    use mockito::mock;

    use crate::common::formats::SerializableSocket;
    use crate::error;
    use crate::new_error;

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If the API returns values with 201 Created, it returns CreateRtcpResponse
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_201() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "rtcp_id": "rc-test",
                    "port": 10003,
                    "ip_v4": "127.0.0.1"
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.expect("event parse error");
        assert_eq!(result.get_id().unwrap().as_str(), "rc-test");
        assert_eq!(result.port(), 10003);
        assert_eq!(result.ip().to_string(), String::from("127.0.0.1"));

        // server called
        httpserver.assert();
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_400() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_DELETE",
                    "params": {
                        "errors": [
                            {
                                "field": "media_id",
                                "message": "media_id field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_403() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_405() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_406() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_rtcp access to the POST /media/rtcp endpoint, and return its response.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_create
    #[tokio::test]
    async fn recv_408() {
        // set up server mock
        let httpserver = mock("POST", "/media/rtcp")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_rtcp(&url);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_delete_rtcp {
    use mockito::mock;

    use crate::error;
    use crate::new_error;

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/media_rtcp_delete
    #[tokio::test]
    async fn recv_204() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "RTCP_DELETE",
                "params": {
                    "errors": [{
                        "field": "media_id",
                        "message": "media_id field is not specified"
                    }]
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 404, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_rtcp access to the DELETE /media/rtcp/{rtcp_id} endpoint, and return its response.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/media_rtcp_deletee
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let rtcp_id = "rc-test";

        // set up server mock
        let path = format!("/media/rtcp/{}", rtcp_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_rtcp(&url, rtcp_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_create_call {
    use mockito::mock;

    use crate::error;
    use crate::new_error;
    use crate::media::formats::CallQuery;
    use crate::prelude::*;

    fn create_params() -> CallQuery {
        CallQuery {
            peer_id: PeerId::new("peer_id"),
            token: Token::new("pt-test"),
            target_id: PeerId::new("target_id"),
            constraints: None,
            redirect_params: None,
        }
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If the API returns values with 202 Accepted, it returns CallResponse
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_202() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::ACCEPTED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "PEERS_CALL",
                "params": {
                    "media_connection_id": "mc-102127d9-30de-413b-93f7-41a33e39d82b"
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::create_call(&url, &call_params);
        let result = task.await.expect("event parse error");
        assert_eq!(
            result.params.media_connection_id.as_str(),
            "mc-102127d9-30de-413b-93f7-41a33e39d82b"
        );

        // server called
        httpserver.assert();
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 400, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_CREATE",
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

        let task = super::create_call(&url, &call_params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 403, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::create_call(&url, &call_params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 405, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::create_call(&url, &call_params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 406, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::create_call(&url, &call_params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn create_call access to the POST /media/connections endpoint.
    /// If server returns 408, create_media returns error
    /// http://35.200.46.204/#/3.media/media_connection_create
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let call_params = create_params();

        // set up server mock
        let httpserver = mock("POST", "/media/connections")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::create_call(&url, &call_params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_delete_call {
    use mockito::mock;

    use crate::error;
    use crate::new_error;

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If the API returns values with 204 No Content
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_204() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "MEDIA_CONNECTION_DELETE",
                "params": {
                    "errors": [{
                        "field": "media_connection_id",
                        "message": "media_connection_id field is not specified"
                    }]
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn delete_call access to the DELETE /media/connections/{media_connection_id} endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_close
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}", media_connection_id);
        let httpserver = mock("DELETE", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_call(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_answer {
    use mockito::mock;

    use crate::error;
    use crate::new_error;
    use crate::media::formats::*;

    fn create_params() -> AnswerQuery {
        let video_params = MediaParams {
            band_width: 1500,
            codec: String::from("H264"),
            media_id: MediaId::new("test"),
            rtcp_id: None,
            payload_type: None,
            sampling_rate: None,
        };

        let constraints = Constraints {
            video: true,
            videoReceiveEnabled: Some(true),
            audio: false,
            audioReceiveEnabled: Some(false),
            video_params: Some(video_params),
            audio_params: None,
            metadata: None,
        };

        AnswerQuery {
            constraints: constraints,
            redirect_params: None,
        }
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If the API returns values with 202 Accepted, it returns AnswerResponse
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_202() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::ACCEPTED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_ANSWER",
                    "params": {
                        "video_port": 10011,
                        "video_id": "vi-test",
                        "audio_port": 10021,
                        "audio_id": "au-test"
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.expect("event parse error");
        assert_eq!(result.params.video_id, Some(MediaId::new("vi-test")));
        assert_eq!(result.params.audio_id, Some(MediaId::new("au-test")));

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_ANSWER",
                    "params": {
                        "errors": [
                            {
                                "field": "media_id",
                                "message": "media_id field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn answer access to the POST /media/connections/{media_connection_id}/answer endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_answer
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/answer", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::answer(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_pli {
    use mockito::mock;

    use crate::common::formats::SerializableSocket;
    use crate::error;
    use crate::new_error;
    use crate::prelude::*;

    fn create_params() -> SocketInfo<PhantomId> {
        SocketInfo::<PhantomId>::new(None, "127.0.0.1:10001".parse().unwrap())
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If the API returns values with 201 Accepted, it returns ()
    /// http://35.200.46.204/#/3.media/media_connection_pli
    #[tokio::test]
    async fn recv_202() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.expect("event parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_PLI",
                    "params": {
                        "errors": [
                            {
                                "field": "media_id",
                                "message": "media_id field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn pli access to the POST /media/connections/{media_connection_id}/pli endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_plir
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";
        let params = create_params();

        // set up server mock
        let path = format!("/media/connections/{}/pli", media_connection_id);
        let httpserver = mock("POST", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();

        let task = super::pli(&url, media_connection_id, &params);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_events {
    use mockito::mock;

    use crate::error;
    use crate::new_error;
    use crate::media::formats::*;

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_ready() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{"event": "READY"}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, EventEnum::READY);

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_stream() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{"event": "STREAM"}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, EventEnum::STREAM);

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_close() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{"event": "CLOSE"}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, EventEnum::CLOSE);

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionEventEnum
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_202_error() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{"event": "ERROR", "error_message": "hoge"}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(
            result,
            EventEnum::ERROR {
                error_message: "hoge".to_string()
            }
        );

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_EVENTS",
                    "params": {
                        "errors": [
                            {
                                "field": "media_id",
                                "message": "media_id field is not specified"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn events access to the GET /media/connections/{media_connection_id}/events endpoint.
    /// If server returns 408, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_event
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/events", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        assert_eq!(result, EventEnum::TIMEOUT);

        // server called
        httpserver.assert();
    }
}

#[cfg(test)]
mod test_status {
    use mockito::mock;

    use crate::error;
    use crate::new_error;

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If the API returns values with 200 Ok, it returns MediaConnectionStatus
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_200() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "metadata": "",
                "open": true,
                "remote_id": "media_caller",
                "ssrc": [{
                    "media_id": "au-test",
                    "ssrc": 2
                },
                {
                    "media_id": "vi-test",
                    "ssrc": 3
                }]
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.expect("event parse error");
        let ssrc = result.ssrc.clone().unwrap();
        assert_eq!(result.open, true);
        assert_eq!(ssrc.len(), 2);
        assert_eq!(ssrc[0].media_id.as_str(), "au-test");
        assert_eq!(ssrc[0].ssrc, 2);
        assert_eq!(ssrc[1].media_id.as_str(), "vi-test");
        assert_eq!(ssrc[1].ssrc, 3);

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "MEDIA_CONNECTION_STATUS",
                    "params": {
                        "errors": [
                            {
                                "field": "media_connection_id",
                                "message": "media_connection_id is not exists."
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 403, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 404, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 405, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 406, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// Fn status access to the GET /media/connections/{media_connection_id}/status endpoint.
    /// If server returns 400, it returns error
    /// http://35.200.46.204/#/3.media/media_connection_status
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let media_connection_id = "mc-102127d9-30de-413b-93f7-41a33e39d82b";

        // set up server mock
        let path = format!("/media/connections/{}/status", media_connection_id);
        let httpserver = mock("GET", path.as_str())
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, media_connection_id);
        let result = task.await.err().expect("event parse error");
        if let new_error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
}
