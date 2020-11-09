mod api;
pub(crate) mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::common::{PhantomId, SocketInfo};
use crate::error;

pub use formats::{
    AnswerQuery, AnswerResponse, AnswerResponseParams, CallQuery, CallResponse, Constraints,
    MediaConnectionId, MediaConnectionIdWrapper, MediaConnectionStatus, MediaId, MediaParams,
    RedirectParameters, RtcpId, SsrcPair,
};

/// Shows DataConnection events.
///
/// It's response from GET /media/connections/{media_connection_id}/events
///
/// [API](http://35.200.46.204/#/3.media/media_connection_event)
#[derive(Debug, PartialEq, PartialOrd)]
pub enum MediaConnectionEventEnum {
    READY(MediaConnectionId),
    STREAM(MediaConnectionId),
    CLOSE(MediaConnectionId),
    ERROR((MediaConnectionId, String)),
}

/// Have WebRTC Gateway open a socket for feeding media.
///
/// This API need to identify whether the media is video or audio.
/// If is_video is true, it's video. Otherwise, it's audio.
///
/// It's bindings for POST /media.
///
/// [API](http://35.200.46.204/#/3.media/media)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::open_media_socket;
///
/// async fn example() {
///     let result = open_media_socket(true).await; //video
///     let result = open_media_socket(false).await; //audio
/// }
/// ```
pub async fn open_media_socket(is_video: bool) -> Result<SocketInfo<MediaId>, error::Error> {
    let base_url = super::base_url();
    api::create_media(base_url, is_video).await
}

/// Have WebRTC Gateway close a media socket.
///
/// It's bindings for DELETE /media/{media_id}
///
/// [API](http://35.200.46.204/#/3.media/streams_delete)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::delete_media;
/// use skyway_webrtc_gateway_api::prelude::MediaId;
///
/// async fn example() {
///     let media_id = MediaId::new("vi-example");
///     let result = delete_media(&media_id).await;
/// }
/// ```
pub async fn delete_media(media_id: &MediaId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_media(base_url, media_id.as_str()).await
}

/// Have WebRTC Gateway open a socket for feeding rtcp.
///
/// It's bindings for POST /media/rtcp.
///
/// [API](http://35.200.46.204/#/3.media/media_rtcp_create)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::open_rtcp_socket;
///
/// async fn example() {
///     let result = open_rtcp_socket().await;
/// }
/// ```
pub async fn open_rtcp_socket() -> Result<SocketInfo<RtcpId>, error::Error> {
    let base_url = super::base_url();
    api::create_rtcp(base_url).await
}

/// Have WebRTC Gateway close a rtcp socket.
///
/// It's bindings for DELETE /media/rtcp/{rtcp_id}
///
/// [API](http://35.200.46.204/#/3.media/media_rtcp_delete)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::delete_rtcp;
/// use skyway_webrtc_gateway_api::prelude::RtcpId;
///
/// async fn example() {
///     let rtcp_id = RtcpId::new("rc-example");
///     let result = delete_rtcp(&rtcp_id).await;
/// }
/// ```
pub async fn delete_rtcp(rtcp_id: &RtcpId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_rtcp(base_url, rtcp_id.as_str()).await
}

/// Have WebRTC Gateway start establishing MediaConnection to neighbour.
///
/// It's bindings for POST /media/connections.
///
/// [API](http://35.200.46.204/#/3.media/media_connection_create)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::{call, CallQuery, Constraints};
/// use skyway_webrtc_gateway_api::prelude::{PeerId, MediaConnectionId, Token};
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let query = CallQuery {
///         peer_id: PeerId::new("peer_id"),
///         token: Token::new("token"),
///         target_id: PeerId::new("target_id"),
///         constraints: Some(Constraints {
///             video: true,
///             videoReceiveEnabled: Some(false),
///             audio: false,
///             audioReceiveEnabled: Some(false),
///             video_params: None,
///             audio_params: None,
///         }),
///         redirect_params: None,
///     };
///     let result = call(&query).await;
/// }
/// ```
pub async fn call(call_params: &CallQuery) -> Result<CallResponse, error::Error> {
    let base_url = super::base_url();
    api::create_call(base_url, call_params).await
}

/// Have WebRTC Gateway accept to a request of establishing MediaConnection from neighbours.
///
/// It's bindings for POST /media/connections/{media_connection_id}/answer
///
/// [API](http://35.200.46.204/#/3.media/media_connection_answer)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::{answer, AnswerQuery, Constraints};
/// use skyway_webrtc_gateway_api::prelude::MediaConnectionId;
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let query = AnswerQuery {
///         constraints: Constraints {
///             video: true,
///             videoReceiveEnabled: Some(false),
///             audio: false,
///             audioReceiveEnabled: Some(false),
///             video_params: None,
///             audio_params: None,
///         },
///         redirect_params: None,
///     };
///     let result = answer(&media_connection_id, &query).await;
/// }
/// ```
pub async fn answer(
    media_connection_id: &MediaConnectionId,
    params: &AnswerQuery,
) -> Result<AnswerResponse, error::Error> {
    let base_url = super::base_url();
    api::answer(base_url, media_connection_id.as_str(), params).await
}

/// Have WebRTC Gateway close a MediaConnection
///
/// It's bindings for DELETE /media/connections/{media_connection_id}.
///
/// [API](http://35.200.46.204/#/3.media/media_connection_close)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::disconnect;
/// use skyway_webrtc_gateway_api::prelude::MediaConnectionId;
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let result = disconnect(&media_connection_id).await;
/// }
/// ```
pub async fn disconnect(media_connection_id: &MediaConnectionId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_call(base_url, media_connection_id.as_str()).await
}

/// Have WebRTC Gateway send a PLI(Picture Less Indication) packet
///
/// A PLI packets informs the encoder about the loss of an undefined amount of coded video data
/// belonging to one or more pictures([RFC](https://tools.ietf.org/html/rfc4585#section-6.3.1)).
///
/// It's bindings for POST /media/connections/{media_connection_id}/pli
///
/// [API](http://35.200.46.204/#/3.media/media_connection_pli)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::send_pli;
/// use skyway_webrtc_gateway_api::prelude::{MediaConnectionId, PhantomId, SerializableSocket, SocketInfo};
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let socket = SocketInfo::<PhantomId>::new(None, "127.0.0.1:8000".parse().unwrap());
///     let result = send_pli(&media_connection_id, &socket).await;
/// }
/// ```
pub async fn send_pli(
    media_connection_id: &MediaConnectionId,
    params: &SocketInfo<PhantomId>,
) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::pli(base_url, media_connection_id.as_str(), params).await
}

/// Request status of MediaConnection
///
/// This function keep listening events with GET /media/connections/{media_connection_id}/events
/// until it receives a CLOSE event or an Error event.
/// If it receives timeout, it ignores the event and listen events again.
///
/// [API](http://35.200.46.204/#/3.media/media_connection_event)
///
/// # Examples
/// ```
/// use futures::channel::mpsc;
/// use futures::future::{self, *};
/// use futures::stream::*;
/// use futures::*;
///
/// use skyway_webrtc_gateway_api::media::{MediaConnectionEventEnum, listen_events};
/// use skyway_webrtc_gateway_api::prelude::MediaConnectionId;
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let (mc_event_notifier, mc_event_observer) = mpsc::channel::<MediaConnectionEventEnum>(0);
///     let mc_event_observer = mc_event_observer.for_each(|event| async move {
///     // Do something
///     });
///     let events_fut = listen_events(media_connection_id, mc_event_notifier);
///     let _ = join!(mc_event_observer, events_fut);
/// }
/// ```
pub async fn listen_events<'a>(
    media_connection_id: MediaConnectionId,
    mut event_notifier: mpsc::Sender<MediaConnectionEventEnum>,
) -> Result<(), error::Error> {
    let base_url = super::base_url();

    loop {
        let result = api::event(base_url, media_connection_id.as_str()).await?;
        match result {
            formats::EventEnum::READY => {
                if event_notifier
                    .send(MediaConnectionEventEnum::READY(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::EventEnum::CLOSE => {
                if event_notifier
                    .send(MediaConnectionEventEnum::CLOSE(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
                break;
            }
            formats::EventEnum::STREAM => {
                if event_notifier
                    .send(MediaConnectionEventEnum::STREAM(
                        media_connection_id.clone(),
                    ))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::EventEnum::ERROR {
                error_message: message,
            } => {
                if event_notifier
                    .send(MediaConnectionEventEnum::ERROR((
                        media_connection_id.clone(),
                        message,
                    )))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::EventEnum::TIMEOUT => {}
        }
    }

    Ok(())
}

/// Request status of MediaConnection
///
/// It's bindings for GET /media/connections/{media_connection_id}/events.
///
/// [API](http://35.200.46.204/#/3.media/media_connection_status)
///
/// # Examples
/// ```
/// use skyway_webrtc_gateway_api::media::status;
/// use skyway_webrtc_gateway_api::prelude::MediaConnectionId;
///
/// async fn example() {
///     let media_connection_id = MediaConnectionId::new("mc-example");
///     let result = status(&media_connection_id).await;
/// }
/// ```
pub async fn status(
    media_connection_id: &MediaConnectionId,
) -> Result<MediaConnectionStatus, error::Error> {
    let base_url = super::base_url();
    api::status(base_url, media_connection_id.as_str()).await
}
