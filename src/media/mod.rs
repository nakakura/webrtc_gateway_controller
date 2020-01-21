mod api;
pub mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::error;
use formats::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MediaConnectionEventEnum {
    READY(MediaConnectionId),
    STREAM(MediaConnectionId),
    CLOSE(MediaConnectionId),
    ERROR((MediaConnectionId, String)),
}

pub async fn open_media_socket(is_video: bool) -> Result<CreateMediaResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::create_media(base_url, is_video).await
}

pub async fn delete_media(media_id: &MediaId) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::delete_media(base_url, media_id.as_str()).await
}

pub async fn open_rtcp_socket() -> Result<CreateRtcpResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::create_rtcp(base_url).await
}

pub async fn delete_rtcp(rtcp_id: &RtcpId) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::delete_rtcp(base_url, rtcp_id.as_str()).await
}

pub async fn call(call_params: &CallParameters) -> Result<CallResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::create_call(base_url, call_params).await
}

pub async fn answer(
    media_connection_id: &MediaConnectionId,
    params: &AnswerParameters,
) -> Result<AnswerResponse, error::ErrorEnum> {
    let base_url = super::base_url();
    api::answer(base_url, media_connection_id.as_str(), params).await
}

pub async fn disconnect(media_connection_id: &MediaConnectionId) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::delete_call(base_url, media_connection_id.as_str()).await
}

pub async fn send_pli(
    base_url: &str,
    media_connection_id: &MediaConnectionId,
    params: &RedirectParams,
) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();
    api::pli(base_url, media_connection_id.as_str(), params).await
}

pub async fn listen_events<'a>(
    media_connection_id: MediaConnectionId,
    mut event_notifier: mpsc::Sender<MediaConnectionEventEnum>,
) -> Result<(), error::ErrorEnum> {
    let base_url = super::base_url();

    loop {
        let result = api::event(base_url, media_connection_id.as_str()).await?;
        match result {
            formats::MediaConnectionEventEnum::READY => {
                if event_notifier
                    .send(MediaConnectionEventEnum::READY(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
            }
            formats::MediaConnectionEventEnum::CLOSE => {
                if event_notifier
                    .send(MediaConnectionEventEnum::CLOSE(media_connection_id.clone()))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
                break;
            }
            formats::MediaConnectionEventEnum::STREAM => {
                if event_notifier
                    .send(MediaConnectionEventEnum::STREAM(
                        media_connection_id.clone(),
                    ))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
                break;
            }
            formats::MediaConnectionEventEnum::ERROR {
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
                    return Err(error::ErrorEnum::create_myerror("fail to notify an event"));
                };
            }
            formats::MediaConnectionEventEnum::TIMEOUT => {}
        }
    }

    Ok(())
}

pub async fn status(
    media_connection_id: &MediaConnectionId,
) -> Result<MediaConnectionStatus, error::ErrorEnum> {
    let base_url = super::base_url();
    api::status(base_url, media_connection_id.as_str()).await
}
