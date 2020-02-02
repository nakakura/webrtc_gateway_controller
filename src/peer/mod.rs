/// Functions in this module are responsible for concealing the raw APIs
mod api;
pub(crate) mod formats;

use futures::channel::mpsc;
use futures::*;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::peer::formats::EventEnum;
use formats::PeerId;
pub use formats::{
    CreatePeerQuery, CreatedResponse, PeerCallEvent, PeerCallEventMediaParams, PeerCloseEvent,
    PeerConnectionEvent, PeerErrorEvent, PeerInfo, PeerOpenEvent, PeerStatusMessage,
};

/// Request to create Peer.
///
/// It's bindings for POST /peers
///
/// See [API](http://35.200.46.204/#/1.peers/peer)
///
/// Notice: This api call does not guarantee that WebRTC Gateway creates a Peer Object successfully.
/// You need to wait OPEN event
/// This function returns PeerInfo just for starting receiving events
pub async fn create<'a>(
    api_key: impl Into<String>,
    domain: impl Into<String>,
    peer_id: PeerId,
    turn: bool,
) -> Result<PeerInfo, error::Error> {
    let base_url = crate::base_url();
    let result = api::create_peer(base_url, api_key, domain, peer_id, turn).await?;
    Ok(result.params)
}

/// Listen events of a Peer Object.
///
/// It's bindings for GET /peers/{peer_id}/events
///
/// See [API](http://35.200.46.204/#/1.peers/peer_event)
///
/// This function need to repeat long-polling to WebRTC Gateway's peer event API.
/// When the API returns TIMEOUT events, this function ignore them and keep listening events.
/// It keep listening events till receiving CLOSE event or HTTP Error Codes.
pub async fn listen_events<'a>(
    peer_info: &PeerInfo,
    mut event_sender: mpsc::Sender<PeerEventEnum>,
) -> Result<(), error::Error> {
    let base_url = crate::base_url();
    loop {
        let result = api::event(base_url, peer_info).await?;

        match result {
            EventEnum::TIMEOUT => {}
            EventEnum::CLOSE(event) => {
                if event_sender
                    .send(PeerEventEnum::CLOSE(event))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
                event_sender.close_channel();
                break;
            }
            EventEnum::OPEN(event) => {
                if event_sender.send(PeerEventEnum::OPEN(event)).await.is_err() {
                    return Err(error::Error::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
            }
            EventEnum::CONNECTION(event) => {
                if event_sender
                    .send(PeerEventEnum::CONNECTION(event))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
            }
            EventEnum::CALL(event) => {
                if event_sender.send(PeerEventEnum::CALL(event)).await.is_err() {
                    return Err(error::Error::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
            }
            EventEnum::ERROR(event) => {
                if event_sender
                    .send(PeerEventEnum::ERROR(event))
                    .await
                    .is_err()
                {
                    return Err(error::Error::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
                };
            }
        }
    }
    Ok(())
}

/// Release PeerObject
///
/// It's bindings for DELETE /peers/{peer_id}
///
/// See [API](http://35.200.46.204/#/1.peers/peer_destroy)
pub async fn delete(peer_info: &PeerInfo) -> Result<(), error::Error> {
    let base_url = crate::base_url();
    api::delete_peer(base_url, peer_info).await
}

/// Get status of PeerObject
///
/// It's bindings for GET /peers/{peer_id}/status
///
/// See [API](http://35.200.46.204/#/1.peers/peer_status)
pub async fn status(peer_info: &PeerInfo) -> Result<formats::PeerStatusMessage, error::Error> {
    let base_url = crate::base_url();
    api::status(base_url, peer_info).await
}

/// Response from GET /peers/{peer_id}/events
///
/// See [API](http://35.200.46.204/#/1.peers/peer_event)
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub enum PeerEventEnum {
    OPEN(PeerOpenEvent),
    CLOSE(PeerCloseEvent),
    CONNECTION(PeerConnectionEvent),
    CALL(PeerCallEvent),
    ERROR(PeerErrorEvent),
}
