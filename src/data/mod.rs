mod api;
pub(crate) mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::data::formats::*;
use crate::error;

use crate::common::formats::{SerializableId, SocketInfo};

pub use formats::{
    ConnectQuery, ConnectionQueryOption, DataConnectionId, DataConnectionIdWrapper,
    DataConnectionStatus, DataId, DataIdWrapper, DcInit, RedirectDataParams,
};

/// Shows DataConnection events.
///
/// It's response from GET /data/connections/{data_connection_id}/events
///
/// [API](http://35.200.46.204/#/2.data/data_connection_events)
#[derive(Debug, PartialEq, PartialOrd)]
pub enum DataConnectionEventEnum {
    OPEN(DataConnectionId),
    CLOSE(DataConnectionId),
    ERROR((DataConnectionId, String)),
    TIMEOUT,
}

/// This function let a WebRTC Gateway open a socket to receive media which will be redirected to neighbour peer.
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::data::open_data_socket;
///
/// async fn example() {
///     let result = open_data_socket().await;
/// }
/// ```
pub async fn open_data_socket() -> Result<SocketInfo<DataId>, error::Error> {
    let base_url = super::base_url();
    api::create_data(base_url).await
}

/// This function let a WebRTC Gateway close a socket to receive media which will be redirected to neighbour peer.
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::data::close_data_socket;
/// use skyway_webrtc_gateway_api::prelude::{DataId, SerializableId};
///
/// async fn example() {
///     let data_id = DataId::try_create("da-50a32bab-b3d9-4913-8e20-f79c90a6a211").unwrap();
///     let result = close_data_socket(&data_id).await;
/// }
/// ```
pub async fn close_data_socket(data_id: &DataId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_data(base_url, data_id.as_str()).await
}

/// This function let a WebRTC Gateway establish a DataChannel to neighbour
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::data::ConnectQuery;
/// use skyway_webrtc_gateway_api::prelude::{PeerId, Token};
///
/// let query = ConnectQuery {
///     peer_id: PeerId::new("peer_id"),
///     token: Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap(),
///     options: None,
///     target_id: PeerId::new("target_id"),
///     params: None,
///     redirect_params: None,
/// };
/// ```
pub async fn connect(query: ConnectQuery) -> Result<DataConnectionId, error::Error> {
    let base_url = super::base_url();
    let result = api::create_data_connection(base_url, &query).await?;
    Ok(result.params.data_connection_id)
}

/// This function let a WebRTC Gateway close a DataChannel
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::data::disconnect;
/// use skyway_webrtc_gateway_api::prelude::DataConnectionId;
///
/// async fn example() {
///     let data_connection_id = DataConnectionId::new("dc-example");
///     let result = disconnect(&data_connection_id).await;
/// }
/// ```
pub async fn disconnect(data_connection_id: &DataConnectionId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_data_connection(base_url, data_connection_id.as_str()).await
}

/// DataConnection is automatically established when neighbour connect to this side.
/// In that case, the connection doesn't have source and destination port information.
/// This function set the information.
///
/// # Example
/// ```
/// use skyway_webrtc_gateway_api::prelude::{DataId, DataConnectionId, PhantomId, SocketInfo, SerializableSocket, SerializableId};
/// use skyway_webrtc_gateway_api::data::{DataIdWrapper, RedirectDataParams, redirect};
///
/// async fn example() {
///     let data_connection_id = DataConnectionId::new("dc-example");
///     let feed_params = Some(DataIdWrapper {
///         data_id: DataId::try_create("da-50a32bab-b3d9-4913-8e20-f79c90a6a211").unwrap()
///     });
///     let redirect_params = SocketInfo::<PhantomId>::try_create(None, "127.0.0.1", 8000).unwrap();
///     let redirect_params = RedirectDataParams {
///         feed_params: feed_params,
///         redirect_params: Some(redirect_params)
///     };
///     let result = redirect(&data_connection_id, &redirect_params).await;
/// }
/// ```
pub async fn redirect(
    data_connection_id: &DataConnectionId,
    redirect_data_params: &RedirectDataParams,
) -> Result<RedirectDataResponse, error::Error> {
    let base_url = super::base_url();
    api::redirect_data_connection(base_url, data_connection_id.as_str(), redirect_data_params).await
}

/// This function to get status of DataChannel
///
/// # Example
/// ```
/// use skyway_webrtc_gateway_api::prelude::DataConnectionId;
/// use skyway_webrtc_gateway_api::data::status;
///
/// async fn example() {
///     let data_connection_id = DataConnectionId::new("dc-example");
///     let result = status(&data_connection_id).await;
/// }
/// ```
pub async fn status(
    data_connection_id: &DataConnectionId,
) -> Result<DataConnectionStatus, error::Error> {
    let base_url = super::base_url();
    api::status(base_url, data_connection_id.as_str()).await
}

/// This function get a single event from a WebRTC Gateway.
///
/// # Example
/// ```
/// use futures::future::{self, *};
/// use futures::stream::*;
/// use futures::*;
///
/// use skyway_webrtc_gateway_api::data::{DataConnectionEventEnum, listen_events, event};
/// use skyway_webrtc_gateway_api::prelude::DataConnectionId;
///
/// async fn example() {
///     let data_connection_id = DataConnectionId::new("dc-example");
///     let event_result = event(&data_connection_id).await;
/// }
/// ```
pub async fn event<'a>(
    data_connection_id: &DataConnectionId,
) -> Result<DataConnectionEventEnum, error::Error> {
    let base_url = super::base_url();
    let event = api::event(base_url, data_connection_id.as_str()).await?;
    let event = match event {
        formats::EventEnum::OPEN => DataConnectionEventEnum::OPEN(data_connection_id.clone()),
        formats::EventEnum::CLOSE => DataConnectionEventEnum::CLOSE(data_connection_id.clone()),
        formats::EventEnum::ERROR {
            error_message: message,
        } => DataConnectionEventEnum::ERROR((data_connection_id.clone(), message)),
        formats::EventEnum::TIMEOUT => DataConnectionEventEnum::TIMEOUT,
    };
    Ok(event)
}

/// This function keep listening events from a WebRTC Gateway.
/// It keep accessing event API endpoint until receiving a CLOSE event or HTTP Error Code.
///
/// # Example
/// ```
/// use futures::channel::mpsc;
/// use futures::future::{self, *};
/// use futures::stream::*;
/// use futures::*;
///
/// use skyway_webrtc_gateway_api::data::{DataConnectionEventEnum, listen_events};
/// use skyway_webrtc_gateway_api::prelude::DataConnectionId;
///
/// async fn example() {
///     let data_connection_id = DataConnectionId::new("dc-example");
///     let (dc_event_notifier, dc_event_observer) = mpsc::channel::<DataConnectionEventEnum>(0);
///     let dc_event_observer = dc_event_observer.for_each(|event| async move {
///     // Do something
///     });
///     let events_fut = listen_events(data_connection_id, dc_event_notifier);
///     let _ = join!(dc_event_observer, events_fut);
/// }
/// ```
pub async fn listen_events<'a>(
    data_connection_id: DataConnectionId,
    mut event_notifier: mpsc::Sender<DataConnectionEventEnum>,
) -> Result<(), error::Error> {
    let base_url = super::base_url();

    loop {
        let result = api::event(base_url, data_connection_id.as_str()).await?;
        match result {
            formats::EventEnum::OPEN => {
                if event_notifier
                    .send(DataConnectionEventEnum::OPEN(data_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_local_error("fail to notify an event"));
                };
            }
            formats::EventEnum::CLOSE => {
                if event_notifier
                    .send(DataConnectionEventEnum::CLOSE(data_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_local_error("fail to notify an event"));
                };
                break;
            }
            formats::EventEnum::ERROR {
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
                    return Err(error::Error::create_local_error("fail to notify an event"));
                };
            }
            formats::EventEnum::TIMEOUT => {}
        }
    }

    Ok(())
}
