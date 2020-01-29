mod api;
pub(crate) mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::common::{PhantomId, SocketInfo};
use crate::error;

pub use formats::{
    AnswerParameters, AnswerResponse, AnswerResponseParams, CallParameters, CallResponse,
    Constraints, MediaConnectionEventEnum, MediaConnectionIdWrapper, MediaConnectionStatus,
    MediaParams, RedirectParameters, SsrcPair,
};
use formats::{MediaConnectionId, MediaId, RtcpId};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MediaConnectionEvents {
    READY(MediaConnectionId),
    STREAM(MediaConnectionId),
    CLOSE(MediaConnectionId),
    ERROR((MediaConnectionId, String)),
}

pub async fn open_media_socket(is_video: bool) -> Result<SocketInfo<MediaId>, error::Error> {
    let base_url = super::base_url();
    api::create_media(base_url, is_video).await
}

pub async fn delete_media(media_id: &MediaId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_media(base_url, media_id.as_str()).await
}

pub async fn open_rtcp_socket() -> Result<SocketInfo<RtcpId>, error::Error> {
    let base_url = super::base_url();
    api::create_rtcp(base_url).await
}

pub async fn delete_rtcp(rtcp_id: &RtcpId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_rtcp(base_url, rtcp_id.as_str()).await
}

pub async fn call(call_params: &CallParameters) -> Result<CallResponse, error::Error> {
    let base_url = super::base_url();
    api::create_call(base_url, call_params).await
}

pub async fn answer(
    media_connection_id: &MediaConnectionId,
    params: &AnswerParameters,
) -> Result<AnswerResponse, error::Error> {
    let base_url = super::base_url();
    api::answer(base_url, media_connection_id.as_str(), params).await
}

pub async fn disconnect(media_connection_id: &MediaConnectionId) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::delete_call(base_url, media_connection_id.as_str()).await
}

pub async fn send_pli(
    media_connection_id: &MediaConnectionId,
    params: &SocketInfo<PhantomId>,
) -> Result<(), error::Error> {
    let base_url = super::base_url();
    api::pli(base_url, media_connection_id.as_str(), params).await
}

pub async fn listen_events<'a>(
    media_connection_id: MediaConnectionId,
    mut event_notifier: mpsc::Sender<MediaConnectionEvents>,
) -> Result<(), error::Error> {
    let base_url = super::base_url();

    loop {
        let result = api::event(base_url, media_connection_id.as_str()).await?;
        match result {
            formats::MediaConnectionEventEnum::READY => {
                if event_notifier
                    .send(MediaConnectionEvents::READY(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::MediaConnectionEventEnum::CLOSE => {
                if event_notifier
                    .send(MediaConnectionEvents::CLOSE(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
                break;
            }
            formats::MediaConnectionEventEnum::STREAM => {
                if event_notifier
                    .send(MediaConnectionEvents::STREAM(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::MediaConnectionEventEnum::ERROR {
                error_message: message,
            } => {
                if event_notifier
                    .send(MediaConnectionEvents::ERROR((
                        media_connection_id.clone(),
                        message,
                    )))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("fail to notify an event"));
                };
            }
            formats::MediaConnectionEventEnum::TIMEOUT => {}
        }
    }

    Ok(())
}

pub async fn status(
    media_connection_id: &MediaConnectionId,
) -> Result<MediaConnectionStatus, error::Error> {
    let base_url = super::base_url();
    api::status(base_url, media_connection_id.as_str()).await
}
