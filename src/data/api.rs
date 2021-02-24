use futures::*;
use reqwest;
use reqwest::Client;
use serde_json::json;

use super::formats::*;
use crate::common::api;
use crate::common::formats::SocketInfo;
use crate::error;

/// It access to the POST /data endpoint, and return its response.
/// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
/// If server returns 400, 405, 406, 408, create_data returns error
/// http://35.200.46.204/#/2.data/data
pub(crate) async fn create_data(base_url: &str) -> Result<SocketInfo<DataId>, error::Error> {
    let api_url = format!("{}/data", base_url);
    let json = json!({});
    let api_call = || {
        Client::new()
            .post(&api_url)
            .json(&json)
            .send()
            .map_err(Into::into)
    };
    let parser = |r: reqwest::Response| r.json::<SocketInfo<DataId>>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::CREATED, false, api_call, parser).await
}

/// This function access to the DELETE /data endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_delete
pub(crate) async fn delete_data(base_url: &str, data_id: &str) -> Result<(), error::Error> {
    let api_url = format!("{}/data/{}", base_url, data_id);
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// This function access to the POST /data/connections endpoint.
/// The API returns 202 Accepted, when a WebRTC Gateway succeed to start calling
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connections_create
pub(crate) async fn create_data_connection(
    base_url: &str,
    params: &ConnectQuery,
) -> Result<ConnectionResponse, error::Error> {
    let api_url = format!("{}/data/connections", base_url);
    let api_call = || {
        Client::new()
            .post(&api_url)
            .json(params)
            .send()
            .map_err(Into::into)
    };
    let parser = |r: reqwest::Response| r.json::<ConnectionResponse>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::ACCEPTED, false, api_call, parser).await
}

/// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
/// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connection_close
pub(crate) async fn delete_data_connection(
    base_url: &str,
    data_connection_id: &str,
) -> Result<(), error::Error> {
    let api_url = format!("{}/data/connections/{}", base_url, data_connection_id);
    let api_call = || Client::new().delete(&api_url).send().map_err(Into::into);
    let parser = |_| future::ok(());
    api::api_access(reqwest::StatusCode::NO_CONTENT, true, api_call, parser).await
}

/// This function access to the PUT data/connections/{data_connection_id} endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to start redirecting data received from neighbours
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/data_connection_put
pub(crate) async fn redirect_data_connection(
    base_url: &str,
    data_connection_id: &str,
    redirect_data_params: &RedirectDataParams,
) -> Result<RedirectDataResponse, error::Error> {
    let api_url = format!("{}/data/connections/{}", base_url, data_connection_id);
    let api_call = || {
        {
            Client::new()
                .put(&api_url)
                .json(redirect_data_params)
                .send()
        }
        .map_err(Into::into)
    };
    let parser = |r: reqwest::Response| r.json::<RedirectDataResponse>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

/// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
/// It returns 400, 403, 404, 405, 406, 408 to show errors.
/// http://35.200.46.204/#/2.data/status
pub(crate) async fn status(
    base_url: &str,
    data_connection_id: &str,
) -> Result<DataConnectionStatus, error::Error> {
    let api_url = format!(
        "{}/data/connections/{}/status",
        base_url, data_connection_id
    );
    let api_call = || Client::new().get(&api_url).send().map_err(Into::into);
    let parser = |r: reqwest::Response| r.json::<DataConnectionStatus>().map_err(Into::into);
    api::api_access(reqwest::StatusCode::OK, true, api_call, parser).await
}

/// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
/// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
/// Fn event returns DataConnectionEventEnum::Timeout to listen event again.
/// When it receives 400, 403, 404, 405, 406, show errors.
/// http://35.200.46.204/#/2.data/events
pub(crate) async fn event(
    base_url: &str,
    data_connection_id: &str,
) -> Result<EventEnum, error::Error> {
    let api_url = format!(
        "{}/data/connections/{}/events",
        base_url, data_connection_id
    );
    let api_call = || Client::new().get(&api_url).send().map_err(Into::into);
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

#[cfg(test)]
mod test_create_data {
    use mockito::mock;

    use crate::common::formats::SerializableSocket;
    use crate::data::formats::DataId;
    use crate::error;

    /// If the API returns values with 201 Created, create_data returns the information as CreateDataResponse
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_201() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::CREATED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "data_id": "da-test",
                    "port": 50000,
                    "ip_v4": "127.0.0.1"
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data(&url);
        let result = task.await.expect("event parse error");
        assert_eq!(result.get_id(), Some(DataId::new("da-test")));
        assert_eq!(result.port(), 50000);
        assert_eq!(result.ip().to_string(), String::from("127.0.0.1"));

        // server called
        httpserver.assert();
    }

    /// API returns 400 error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_400() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "DATA_CREATE",
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
        let task = super::create_data(&url);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403 error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_403() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data(&url);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405 error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_405() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data(&url);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406 error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_406() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data(&url);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406 error
    /// http://35.200.46.204/#/2.data/data
    #[tokio::test]
    async fn recv_408() {
        // set up server mock
        let httpserver = mock("POST", "/data")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data(&url);
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
mod test_delete_data {
    use mockito::mock;

    use crate::data::formats::DataId;
    use crate::error;

    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Data Object.
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_204() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_403() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 404
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_404() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_405() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }
        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_406() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/2.data/data_delete
    #[tokio::test]
    async fn recv_408() {
        let data_id = DataId::new("da-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/da-test")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data(&url, data_id.as_str());
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
mod test_create_data_connection {
    use mockito::mock;

    use crate::data::formats::*;
    use crate::error;
    use crate::prelude::*;

    fn create_options() -> ConnectQuery {
        let peer_id = PeerId::new("peer_id");
        let token = Token::new("test-token");
        let target_id = PeerId::new("target_id");
        let data_id = DataId::new("da-test");
        let data_id = DataIdWrapper { data_id: data_id };
        let query = ConnectQuery {
            peer_id: peer_id,
            token: token,
            options: None,
            target_id: target_id,
            params: Some(data_id.clone()),
            redirect_params: None,
        };
        query
    }

    /// The API returns 202 Accepted, when a WebRTC Gateway succeed to create a DataConnection
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_202() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::ACCEPTED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "PEERS_CONNECT",
                    "params": {
                        "data_connection_id": "dc-test"
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
        let result = task.await.expect("parse error");
        assert_eq!(result.params.data_connection_id.as_str(), "dc-test");

        // server called
        httpserver.assert();
    }

    /// API returns 400
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_400() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "DATA_CONNECTION_CREATE",
                    "params": {
                        "errors": [
                            {
                                "field": "field",
                                "message": "something happened"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 403
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_403() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 405
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_405() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 406
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_406() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// API returns 408
    /// http://35.200.46.204/#/2.data/data_connections_create
    #[tokio::test]
    async fn recv_408() {
        // create api parameters
        let query = create_options();

        // set up server mock
        let httpserver = mock("POST", "/data/connections")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::create_data_connection(&url, &query);
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
mod test_delete_data_connection {
    use mockito::mock;

    use crate::error;
    use crate::prelude::*;

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// The API returns 204 No Content, when a WebRTC Gateway succeed to delete a Peer Object
    /// It returns 400, 403, 404, 405, 406, 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_204() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::NO_CONTENT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, ());

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 400 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_400() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "DATA_CONNECTION_DELETE",
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
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_403() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_404() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_405() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_406() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the DELETE /data/connections/{data_connection_id} endpoint.
    /// It returns 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_close
    #[tokio::test]
    async fn recv_408() {
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("DELETE", "/data/connections/dc-test")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::delete_data_connection(&url, data_connection_id.as_str());
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
mod test_redirect_data_connection {
    use std::net::{IpAddr, SocketAddr};

    use mockito::mock;

    use crate::common::formats::SerializableSocket;
    use crate::data::formats::*;
    use crate::error;
    use crate::prelude::*;

    fn create_param() -> (RedirectDataParams, DataConnectionId) {
        let data_id = DataId::new("da-test");
        let data_connection_id = DataConnectionId::new("dc-test");
        let ip_v4 = "127.0.0.1";
        let port = 10001u16;
        let data_id_obj = DataIdWrapper { data_id: data_id };
        let addr: IpAddr = ip_v4.parse().unwrap();
        let params = SocketInfo::<PhantomId>::new(None, SocketAddr::new(addr, port));

        (
            RedirectDataParams {
                feed_params: Some(data_id_obj),
                redirect_params: Some(params),
            },
            data_connection_id,
        )
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to start redirecting data received from neighbours
    /// http://35.200.46.204/#/2.data/data_connection_put
    #[tokio::test]
    async fn recv_200() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "DATA_CONNECTION_PUT",
                "data_id": "da-50a32bab-b3d9-4913-8e20-f79c90a6a211"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.expect("parse error");
        assert_eq!(
            result.data_id,
            DataId::new("da-50a32bab-b3d9-4913-8e20-f79c90a6a211")
        );

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 400
    /// http://35.200.46.204/#/2.data/data_connection_put
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "DATA_CONNECTION_DELETE",
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
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_put
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the PUT data/connections/{data_connection_id} endpoint.
    /// It returns 408 to show errors.
    /// http://35.200.46.204/#/2.data/data_connection_pute
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let (redirect_data_params, data_connection_id) = create_param();

        // set up server mock
        let httpserver = mock("PUT", "/data/connections/dc-test")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "feed_params": {
                    "data_id": "da-test"
                },
                "redirect_params": {
                    "ip_v4": "127.0.0.1",
                    "port": 10001
                }
            }"#
                .into(),
            ))
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::redirect_data_connection(
            &url,
            data_connection_id.as_str(),
            &redirect_data_params,
        );
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
    use crate::prelude::*;

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_200() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "buffersize": 0,
                "label": "c_3q8ymsw7n9c4s0ibzx8jymygb9",
                "metadata": "",
                "open": true,
                "reliable": true,
                "remote_id": "data_caller",
                "serialization": "BINARY",
                "type": "DATA"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result.open, true);
        assert_eq!(result.reliable, true);

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// API returns 400
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "command_type": "DATA_CONNECTION_STATUS",
                "params": {
                    "errors": [{
                        "field": "string",
                        "message": "string"
                    }]
                }
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 403 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }
    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 404 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 405 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 406 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/status endpoint.
    /// It returns 408 to show errors.
    /// http://35.200.46.204/#/2.data/status
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/status")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::status(&url, data_connection_id.as_str());
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
mod test_event {
    use mockito::mock;

    use crate::data::formats::EventEnum;
    use crate::error;
    use crate::prelude::*;

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_open() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "event": "OPEN"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, EventEnum::OPEN);

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_close() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "event": "CLOSE"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(result, EventEnum::CLOSE);

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 200 Ok, when a WebRTC Gateway succeed to display dataconnection's status.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_200_error() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::OK.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "event": "ERROR",
                "error_message": "error"
            }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.expect("parse error");
        assert_eq!(
            result,
            EventEnum::ERROR {
                error_message: "error".to_string()
            }
        );

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// The API returns 400.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_400() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::BAD_REQUEST.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "command_type": "DATA_CONNECTION_EVENTS",
                    "params": {
                        "errors": [
                            {
                                "field": "string",
                                "message": "string"
                            }
                        ]
                    }
                }"#,
            )
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 403, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_403() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::FORBIDDEN.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 404, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_404() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::NOT_FOUND.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 405, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_405() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::METHOD_NOT_ALLOWED.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// When it receives 406, show errors.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_406() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::NOT_ACCEPTABLE.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.err().expect("parse error");
        if let error::Error::LocalError(_e) = result {
        } else {
            unreachable!();
        }

        // server called
        httpserver.assert();
    }

    /// This function access to the GET /data/connections/{data_connection_id}/events endpoint.
    /// Fn event returns DataConnectionEventEnum::Timeout to listen event again.
    /// http://35.200.46.204/#/2.data/events
    #[tokio::test]
    async fn recv_408() {
        // set up params
        let data_connection_id = DataConnectionId::new("dc-test");

        // set up server mock
        let httpserver = mock("GET", "/data/connections/dc-test/events")
            .with_status(reqwest::StatusCode::REQUEST_TIMEOUT.as_u16() as usize)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // call api
        let url = mockito::server_url();
        let task = super::event(&url, data_connection_id.as_str());
        let result = task.await.unwrap();
        assert_eq!(result, EventEnum::TIMEOUT);

        // server called
        httpserver.assert();
    }
}
