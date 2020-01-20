pub mod api;
pub mod formats;

use futures::channel::mpsc::{self, *};
use futures::*;

use crate::common::{DataConnectionId, PeerId, PeerInfo};
use crate::data::formats::*;
use crate::error;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum DataConnectionEventEnum {
    OPEN(DataConnectionId),
    CLOSE(DataConnectionId),
    ERROR((DataConnectionId, String)),
}

/// This function let a WebRTC Gateway open a socket to receive media which will be redirected to neighbour peer.
pub async fn open_source_socket() -> Result<CreatedResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::create_data(base_url).await
}

/// This function let a WebRTC Gateway close a socket to receive media which will be redirected to neighbour peer.
pub async fn close_source_socket(data_id: DataId) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::delete_data(base_url, data_id.as_str()).await
}

/// This function let a WebRTC Gateway establish a DataChannel to neighbour
pub async fn connect(
    query: CreateDataConnectionQuery,
) -> Result<DataConnectionId, error::ErrorEnum> {
    let base_url = super::base_url();
    let result = api::create_data_connection(base_url, &query).await?;
    Ok(result.params.data_connection_id)
}

/// This function let a WebRTC Gateway close a DataChannel
pub async fn disconnect(data_connection_id: DataConnectionId) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::delete_data_connection(base_url, data_connection_id.as_str()).await
}

pub async fn redirect(
    data_connection_id: &DataConnectionId,
    redirect_data_params: &RedirectDataParams,
) -> Result<RedirectDataResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::redirect_data_connection(base_url, data_connection_id.as_str(), redirect_data_params).await
}

/// This function to get status of DataChannel
pub async fn status(
    data_connection_id: DataConnectionId,
) -> Result<DataConnectionStatus, error::ErrorEnum> {
    let base_url = super::base_url();
    api::status(base_url, data_connection_id.as_str()).await
}

/// This function keep listening events from a WebRTC Gateway.
/// It keep accessing event API endpoint until receiving a CLOSE event or HTTP Error Code.
pub async fn listen_events<'a>(
    data_connection_id: DataConnectionId,
    mut event_notifier: mpsc::Sender<DataConnectionEventEnum>,
    #[cfg(test)] mut inject_api_events: Box<
        dyn FnMut(&str, &str) -> Result<formats::DataConnectionEventEnum, error::ErrorEnum> + 'a,
    >,
) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();

    loop {
        #[cfg(test)]
        let result = inject_api_events(base_url, data_connection_id.as_str())?;
        #[cfg(not(test))]
        let result = api::event(base_url, data_connection_id.as_str()).await?;
        match result {
            formats::DataConnectionEventEnum::OPEN => {
                if event_notifier
                    .send(DataConnectionEventEnum::OPEN(data_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
            }
            formats::DataConnectionEventEnum::CLOSE => {
                if event_notifier
                    .send(DataConnectionEventEnum::CLOSE(data_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
                break;
            }
            formats::DataConnectionEventEnum::ERROR {
                error_message: message,
            } => {
                if event_notifier
                    .send(DataConnectionEventEnum::ERROR((
                        data_connection_id.clone(),
                        message,
                    )))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
            }
            formats::DataConnectionEventEnum::TIMEOUT => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_listen_events {
    use futures::channel::mpsc::*;
    use futures::*;

    use super::*;
    use crate::common::{PeerId, PeerInfo, Token};
    use crate::error;

    #[tokio::test]
    async fn listen_event() {
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _data_connection_id: &str|
                      -> Result<formats::DataConnectionEventEnum, error::ErrorEnum> {
                    counter += 1;
                    if counter == 1 {
                        Ok(formats::DataConnectionEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        Ok(formats::DataConnectionEventEnum::OPEN)
                    } else if counter == 3 {
                        Ok(formats::DataConnectionEventEnum::ERROR {
                            error_message: "error happen".to_string(),
                        })
                    } else if counter == 4 {
                        Ok(formats::DataConnectionEventEnum::CLOSE)
                    } else {
                        unreachable!();
                    }
                }
            }
        }();

        let (tx, mut rx) = mpsc::channel::<super::DataConnectionEventEnum>(0);
        let fut = listen_events(
            DataConnectionId::new("hoge"),
            tx,
            Box::new(inject_api_event),
        );
        let events_future = async {
            let event = rx.next().await;
            assert_eq!(
                event,
                Some(super::DataConnectionEventEnum::OPEN(DataConnectionId(
                    "hoge".to_string()
                )))
            );

            let event = rx.next().await;
            let error = super::DataConnectionEventEnum::ERROR((
                DataConnectionId::new("hoge"),
                "error happen".to_string(),
            ));
            assert_eq!(event, Some(error));

            let event = rx.next().await;
            assert_eq!(
                event,
                Some(super::DataConnectionEventEnum::CLOSE(DataConnectionId(
                    "hoge".to_string()
                )))
            );

            let event = rx.next().await;
            assert_eq!(event, None);
        };
        let (_, result) = join!(events_future, fut);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn listen_event_http_error() {
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _data_connection_id: &str|
                      -> Result<formats::DataConnectionEventEnum, error::ErrorEnum> {
                    counter += 1;
                    if counter == 1 {
                        Ok(formats::DataConnectionEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        Err(error::ErrorEnum::create_myerror("http error"))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();

        let (tx, mut rx) = mpsc::channel::<super::DataConnectionEventEnum>(0);
        let fut = listen_events(
            DataConnectionId::new("hoge"),
            tx,
            Box::new(inject_api_event),
        );
        let events_future = async {
            let event = rx.next().await;
            assert_eq!(event, None);
        };
        let (_, result) = join!(events_future, fut);
        assert!(result.is_err());
    }
}
