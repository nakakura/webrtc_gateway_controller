pub mod api;
pub mod formats;

use futures::channel::mpsc::*;
use futures::*;

use crate::error;
use crate::data::formats::*;

pub async fn connect_flow<'a>(
    base_url: &str,
    peer_info: super::peer::formats::PeerInfo,
    on_open_tx: Option<Sender<String>>,
    on_close_tx: Option<Sender<String>>,
    on_error_tx: Option<Sender<(String, String)>>,
    #[cfg(test)] mut inject_api_create_data: Box<
        dyn FnMut(&str) -> Result<CreatedResponse, error::ErrorEnum> + 'a,
    >,
    #[cfg(test)] mut inject_api_create_data_connection: Box<
        dyn FnMut(
                &str,
                &CreateDataConnectionQuery,
            ) -> Result<CreateDataConnectionResponse, error::ErrorEnum>
            + 'a,
    >,
    #[cfg(test)] inject_api_events: Box<
        dyn FnMut(&str, &str) -> Result<DataConnectionEventEnum, error::ErrorEnum> + 'a,
    >,
) -> Result<(), error::ErrorEnum> {
    #[cfg(test)]
    let result = inject_api_create_data(base_url);
    #[cfg(not(test))]
    let result = api::create_data(base_url).await;
    if result.is_err() {
        return result.map(|_| ());
    }
    let result = result.unwrap();

    let data_id = formats::DataId {
        data_id: result.data_id,
    };
    let query = formats::CreateDataConnectionQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        options: None,                        //FIXME
        target_id: "data_callee".to_string(), //FIXME
        params: data_id,
        redirect_params: None, //FIXME
    };
    #[cfg(test)]
    let result = inject_api_create_data_connection(base_url, &query);
    #[cfg(not(test))]
    let result = api::create_data_connection(base_url, &query).await;
    if result.is_err() {
        return result.map(|_| ());
    }
    let result = result.unwrap();
    listen_events(
        base_url,
        &result.params.data_connection_id,
        on_open_tx,
        on_close_tx,
        on_error_tx,
        #[cfg(test)]
        inject_api_events,
    )
    .await
}

pub async fn redirect_flow<'a>(
    base_url: &str,
    data_connection_id: &str,
    on_open_tx: Option<Sender<String>>,
    on_close_tx: Option<Sender<String>>,
    on_error_tx: Option<Sender<(String, String)>>,
    #[cfg(test)] mut inject_api_create_data: Box<
        dyn FnMut(&str) -> Result<CreatedResponse, error::ErrorEnum> + 'a,
    >,
    #[cfg(test)] mut inject_api_redirect_data: Box<
        dyn FnMut(&str, &str, &RedirectDataParams) -> Result<RedirectDataResponse, error::ErrorEnum>
            + 'a,
    >,
    #[cfg(test)] inject_api_events: Box<
        dyn FnMut(&str, &str) -> Result<DataConnectionEventEnum, error::ErrorEnum> + 'a,
    >,
) -> Result<(), error::ErrorEnum> {
    #[cfg(test)]
    let result = inject_api_create_data(base_url);
    #[cfg(not(test))]
    let result = api::create_data(base_url).await;
    if result.is_err() {
        return result.map(|_| ());
    }
    let result = result.unwrap();
    let data_id_obj = formats::DataId {
        data_id: result.data_id,
    };
    let redirect_params = formats::RedirectParams {
        ip_v4: Some("127.0.0.1".to_string()), //FIXME
        ip_v6: None,
        port: 10000, //FIXME
    };
    let redirect_data_params = formats::RedirectDataParams {
        feed_params: data_id_obj,
        redirect_params: redirect_params,
    };

    #[cfg(test)]
    let result = inject_api_redirect_data(base_url, data_connection_id, &redirect_data_params);
    #[cfg(not(test))]
    let result =
        api::redirect_data_connection(base_url, data_connection_id, &redirect_data_params).await;
    if result.is_err() {
        return result.map(|_| ());
    }
    let _ = result.unwrap();
    listen_events(
        base_url,
        data_connection_id,
        on_open_tx,
        on_close_tx,
        on_error_tx,
        #[cfg(test)]
        inject_api_events,
    )
    .await
}

async fn listen_events<'a>(
    base_url: &str,
    data_connection_id: &str,
    mut on_open_tx: Option<Sender<String>>,
    mut on_close_tx: Option<Sender<String>>,
    mut on_error_tx: Option<Sender<(String, String)>>,
    #[cfg(test)] mut inject_api_events: Box<
        dyn FnMut(&str, &str) -> Result<DataConnectionEventEnum, error::ErrorEnum> + 'a,
    >,
) -> Result<(), error::ErrorEnum> {
    loop {
        #[cfg(test)]
        let result = inject_api_events(base_url, data_connection_id);
        #[cfg(not(test))]
        let result = api::event(base_url, data_connection_id).await;
        match result {
            Ok(formats::DataConnectionEventEnum::OPEN) => {
                if let Some(ref mut tx) = on_open_tx {
                    if tx.send(data_connection_id.to_string()).await.is_err() {
                        break;
                    }
                }
            }
            Ok(formats::DataConnectionEventEnum::CLOSE) => {
                if let Some(ref mut tx) = on_close_tx {
                    let _ = tx.send(data_connection_id.to_string()).await;
                }
                break;
            }
            Ok(formats::DataConnectionEventEnum::ERROR {
                error_message: message,
            }) => {
                if let Some(ref mut tx) = on_error_tx {
                    let _ = tx.send((data_connection_id.to_string(), message)).await;
                }
                break;
            }
            Ok(formats::DataConnectionEventEnum::TIMEOUT) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_connect_flow {
    use futures::channel::mpsc::*;
    use futures::*;

    use super::formats::*;
    use super::*;
    use crate::error;
    use crate::peer::formats::PeerInfo;

    #[tokio::test]
    async fn create_data_error() {
        // create_data api mock, returns 404 error
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // create_data_connection api mock, returns 404 error
        let inject_api_create_data_connection =
            move |_base_url: &str,
                  _query: &CreateDataConnectionQuery|
                  -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let peer_info = PeerInfo {
            peer_id: "peer_id".to_string(),
            token: "token".to_string(),
        };

        let result = super::connect_flow(
            "base_url",
            peer_info,
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_create_data_connection),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_data_connection_error() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns 404 error
        let inject_api_create_data_connection =
            move |_base_url: &str,
                  _query: &CreateDataConnectionQuery|
                  -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let peer_info = PeerInfo {
            peer_id: "peer_id".to_string(),
            token: "token".to_string(),
        };

        let result = super::connect_flow(
            "base_url",
            peer_info,
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_create_data_connection),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_data_connection_success_event_error() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns success
        let inject_api_create_data_connection =
            move |_base_url: &str,
                  _query: &CreateDataConnectionQuery|
                  -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
                Ok(CreateDataConnectionResponse {
                    command_type: "PEERS_CONNECT".to_string(),
                    params: DataConnectionId {
                        data_connection_id: "data_connection_id".to_string(),
                    },
                })
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let peer_info = PeerInfo {
            peer_id: "peer_id".to_string(),
            token: "token".to_string(),
        };

        let result = super::connect_flow(
            "base_url",
            peer_info,
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_create_data_connection),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_data_connection_success_and_close_event() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns success
        let inject_api_create_data_connection =
            move |_base_url: &str,
                  _query: &CreateDataConnectionQuery|
                  -> Result<CreateDataConnectionResponse, error::ErrorEnum> {
                Ok(CreateDataConnectionResponse {
                    command_type: "PEERS_CONNECT".to_string(),
                    params: DataConnectionId {
                        data_connection_id: "data_connection_id".to_string(),
                    },
                })
            };
        // event api mock, returns success
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Ok(DataConnectionEventEnum::CLOSE)
        };
        let peer_info = PeerInfo {
            peer_id: "peer_id".to_string(),
            token: "token".to_string(),
        };
        let (on_close_tx, mut on_close_rx) = channel::<String>(0);
        tokio::spawn(async move {
            let _ = on_close_rx
                .next()
                .map(|result| {
                    assert_eq!(result, Some("data_connection_id".to_string()));
                })
                .await;
        });
        let result = super::connect_flow(
            "base_url",
            peer_info,
            None,
            Some(on_close_tx),
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_create_data_connection),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_redirect_flow {
    use super::formats::*;
    use super::*;
    use crate::error;

    #[tokio::test]
    async fn create_data_error() {
        // create_data api mock, returns 404 error
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // create_data_connection api mock, returns 404 error
        let inject_api_redirect_data =
            move |_base_url: &str,
                  _data_connection_di: &str,
                  _redirect_data_params: &RedirectDataParams|
                  -> Result<RedirectDataResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let result = super::redirect_flow(
            "base_url",
            "data_connection_id",
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_redirect_data),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_and_redirect_error() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns 404 error
        let inject_api_redirect_data =
            move |_base_url: &str,
                  _data_connection_di: &str,
                  _redirect_data_params: &RedirectDataParams|
                  -> Result<RedirectDataResponse, error::ErrorEnum> {
                Err(error::ErrorEnum::create_myerror("recv Not Found"))
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let result = super::redirect_flow(
            "base_url",
            "data_connection_id",
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_redirect_data),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_and_redirect_success_and_event_error() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns success
        let inject_api_redirect_data =
            move |_base_url: &str,
                  _data_connection_di: &str,
                  _redirect_data_params: &RedirectDataParams|
                  -> Result<RedirectDataResponse, error::ErrorEnum> {
                Ok(RedirectDataResponse {
                    command_type: "DATA_CONNECTION_PUT".to_string(),
                    data_id: "data_id".to_string(),
                })
            };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let result = super::redirect_flow(
            "base_url",
            "data_connection_id",
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_redirect_data),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_data_success_and_redirect_success_and_close_event() {
        // create_data api mock, returns success
        let inject_api_create_data =
            move |_base_url: &str| -> Result<CreatedResponse, error::ErrorEnum> {
                Ok(CreatedResponse {
                    data_id: "data_id".to_string(),
                    port: 10000,
                    ip_v4: Some("127.0.0.1".to_string()),
                    ip_v6: None,
                })
            };
        // create_data_connection api mock, returns success
        let inject_api_redirect_data =
            move |_base_url: &str,
                  _data_connection_di: &str,
                  _redirect_data_params: &RedirectDataParams|
                  -> Result<RedirectDataResponse, error::ErrorEnum> {
                Ok(RedirectDataResponse {
                    command_type: "DATA_CONNECTION_PUT".to_string(),
                    data_id: "data_id".to_string(),
                })
            };
        // event api mock, returns success
        let inject_api_event = move |_base_url: &str,
                                     _data_conenction_id: &str|
              -> Result<DataConnectionEventEnum, error::ErrorEnum> {
            Ok(DataConnectionEventEnum::CLOSE)
        };
        let result = super::redirect_flow(
            "base_url",
            "data_connection_id",
            None,
            None,
            None,
            Box::new(inject_api_create_data),
            Box::new(inject_api_redirect_data),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_ok());
    }
}
