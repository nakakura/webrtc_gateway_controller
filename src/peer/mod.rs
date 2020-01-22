/// Functions in this module are responsible for concealing the raw APIs
mod api;
pub mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::error;
use formats::*;

/// Send Creating a Peer Object request to WebRTC Gateway
/// Notice: This api call does not guarantee that WebRTC Gateway creates a Peer Object successfully.
/// You need to wait OPEN event
/// This function returns PeerInfo just for starting receiving events
pub async fn create<'a>(
    api_key: impl Into<String>,
    domain: impl Into<String>,
    peer_id: PeerId,
    turn: bool,
) -> Result<PeerInfo, error::ErrorEnum> {
    let base_url = crate::base_url();
    let result = api::create_peer(base_url, api_key, domain, peer_id, turn).await?;
    Ok(result.params)
}

/// Listen events of a Peer Object
/// This function need to repeat long-polling to WebRTC Gateway's peer event API.
/// When the API returns TIMEOUT events, this function ignore them and keep listening events.
/// It keep listening events till receiving CLOSE event or HTTP Error Codes.
pub async fn listen_events<'a>(
    peer_info: &PeerInfo,
    mut event_sender: mpsc::Sender<PeerEventEnum>,
) -> Result<(), error::ErrorEnum> {
    let base_url = crate::base_url();
    loop {
        let result = api::event(base_url, peer_info).await?;

        match result {
            PeerEventEnum::TIMEOUT => {}
            PeerEventEnum::CLOSE(event) => {
                if event_sender
                    .send(PeerEventEnum::CLOSE(event))
                    .await
                    .is_err()
                {
                    return Err(error::ErrorEnum::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
                event_sender.close_channel();
                break;
            }
            event => {
                if event_sender.send(event).await.is_err() {
                    return Err(error::ErrorEnum::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
            }
        }
    }
    Ok(())
}

pub async fn delete(peer_info: &PeerInfo) -> Result<(), error::ErrorEnum> {
    let base_url = crate::base_url();
    api::delete_peer(base_url, peer_info).await
}

pub async fn status(peer_info: &PeerInfo) -> Result<formats::PeerStatusMessage, error::ErrorEnum> {
    let base_url = crate::base_url();
    api::status(base_url, peer_info).await
}
