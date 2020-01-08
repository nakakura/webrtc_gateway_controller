/// Functions in this module are responsible for concealing the raw APIs
mod api;
pub mod formats;

use futures::channel::mpsc;
use futures::*;

use crate::common::PeerInfo;
use crate::error;
use formats::*;

pub async fn create<'a>(
    base_url: &str,
    peer_id: &str,
    turn: bool,
    mut event_sender: mpsc::Sender<PeerEventEnum>,
    #[cfg(test)] mut inject_api_create_peer: Box<
        dyn FnMut(&str, &str, bool) -> Result<CreatedResponse, error::ErrorEnum> + 'a,
    >,
    #[cfg(test)] mut inject_api_events: Box<
        dyn FnMut(&str, &PeerInfo) -> Result<PeerEventEnum, error::ErrorEnum> + 'a,
    >,
) -> Result<(), error::ErrorEnum> {
    #[cfg(test)]
    let result = inject_api_create_peer(base_url, peer_id, turn)?;
    #[cfg(not(test))]
    let result = api::create_peer(base_url, peer_id, turn).await?;
    let peer_info = result.params;

    loop {
        #[cfg(test)]
        let result = inject_api_events(base_url, &peer_info)?;
        #[cfg(not(test))]
        let result = api::event(base_url, &peer_info).await?;

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

pub async fn delete(base_url: &str, peer_info: &PeerInfo) -> Result<(), error::ErrorEnum> {
    api::delete_peer(base_url, peer_info).await
}

pub async fn status(base_url: &str, peer_info: &PeerInfo) -> Result<formats::PeerStatusMessage, error::ErrorEnum> {
    api::status(base_url, peer_info).await
}

/// create_peer start listening events if WebRTC Gateway succeed to create peer object.
/// This test check whether create_peer is success, and recv events properly
#[cfg(test)]
mod test_create {
    use futures::channel::mpsc::*;
    use futures::*;

    use super::*;
    use crate::common::{PeerId, PeerInfo, Token};
    use crate::error;

    #[tokio::test]
    async fn recv_open_event_after_long_time() {
        // it mocks that peer object is successfully created.
        let inject_api_create_peer = move |_base_url: &str,
                                           peer_id: &str,
                                           _turn: bool|
              -> Result<CreatedResponse, error::ErrorEnum> {
            Ok(CreatedResponse {
                command_type: "PEERS_CREATE".to_string(),
                params: PeerInfo {
                    peer_id: PeerId(peer_id.to_string()),
                    token: Token("token".to_string()),
                },
            })
        };
        // WebRTC Gateway may return TIMEOUT before another events.
        // It returns TIMEOUT in 1st call, and OPEN event in 2nd call
        // after CLOSE event receives, it never listen event again.
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    counter += 1;
                    if counter == 1 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        let peer_info = PeerInfo {
                            peer_id: PeerId("hoge".to_string()),
                            token: Token("token".to_string()),
                        };
                        Ok(PeerEventEnum::OPEN(PeerOpenEvent { params: peer_info }))
                    } else if counter == 3 {
                        let peer_info = PeerInfo {
                            peer_id: PeerId("hoge".to_string()),
                            token: Token("token".to_string()),
                        };
                        Ok(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();

        let (tx, mut event_listener) = mpsc::channel::<PeerEventEnum>(0);
        let fut = super::create(
            "base_url",
            "peer_id",
            true,
            tx,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        );

        let events_future = async {
            let event = event_listener.next().await;
            let peer_info = PeerInfo {
                peer_id: PeerId("hoge".to_string()),
                token: Token("token".to_string()),
            };
            assert_eq!(
                event,
                Some(PeerEventEnum::OPEN(PeerOpenEvent { params: peer_info }))
            );

            let event = event_listener.next().await;
            let peer_info = PeerInfo {
                peer_id: PeerId("hoge".to_string()),
                token: Token("token".to_string()),
            };
            assert_eq!(
                event,
                Some(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
            );

            let event = event_listener.next().await;
            assert!(event.is_none());
        };

        let (_, result) = join!(events_future, fut);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn recv_error_event_after_connection_event() {
        // it mocks that peer object is successfully created.
        let inject_api_create_peer = move |_base_url: &str,
                                           peer_id: &str,
                                           _turn: bool|
              -> Result<CreatedResponse, error::ErrorEnum> {
            Ok(CreatedResponse {
                command_type: "PEERS_CREATE".to_string(),
                params: PeerInfo {
                    peer_id: PeerId(peer_id.to_string()),
                    token: Token("token".to_string()),
                },
            })
        };
        // WebRTC Gateway may return TIMEOUT before another events.
        // It returns TIMEOUT in 1st call, and ERROR event in 2nd call
        // returning error event is normal operation of listen_event, so it keep listening events before recv close event
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    counter += 1;
                    if counter == 1 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        let peer_info = PeerInfo {
                            peer_id: PeerId("peer_id".to_string()),
                            token: Token("token".to_string()),
                        };
                        Ok(PeerEventEnum::ERROR(PeerErrorEvent {
                            params: peer_info,
                            error_message: "peer_id field is not specified".to_string(),
                        }))
                    } else if counter == 3 {
                        // user would call dalete /peer after receiving error message.
                        let peer_info = PeerInfo {
                            peer_id: PeerId("hoge".to_string()),
                            token: Token("token".to_string()),
                        };
                        Ok(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();

        let (tx, mut event_listener) = mpsc::channel::<PeerEventEnum>(0);
        let fut = super::create(
            "base_url",
            "peer_id",
            true,
            tx,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        );

        let events_future = async {
            let event = event_listener.next().await;
            let peer_info = PeerInfo {
                peer_id: PeerId("peer_id".to_string()),
                token: Token("token".to_string()),
            };
            let error_event = Some(PeerEventEnum::ERROR(PeerErrorEvent {
                params: peer_info,
                error_message: "peer_id field is not specified".to_string(),
            }));
            assert_eq!(event, error_event);

            let event = event_listener.next().await;
            let peer_info = PeerInfo {
                peer_id: PeerId("hoge".to_string()),
                token: Token("token".to_string()),
            };
            assert_eq!(
                event,
                Some(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
            );

            let event = event_listener.next().await;
            assert!(event.is_none());
        };

        let (_, result) = join!(events_future, fut);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn recv_404_after_connection_event() {
        // it mocks that peer object is successfully created.
        let inject_api_create_peer = move |_base_url: &str,
                                           peer_id: &str,
                                           _turn: bool|
              -> Result<CreatedResponse, error::ErrorEnum> {
            Ok(CreatedResponse {
                command_type: "PEERS_CREATE".to_string(),
                params: PeerInfo {
                    peer_id: PeerId(peer_id.to_string()),
                    token: Token("token".to_string()),
                },
            })
        };
        // WebRTC Gateway may return TIMEOUT before another events.
        // It returns TIMEOUT in 1st call, and 2nd call fail
        // 3rd call never fires
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    counter += 1;
                    if counter == 1 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        Err(error::ErrorEnum::create_myerror("error"))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();

        let (tx, mut event_listener) = mpsc::channel::<PeerEventEnum>(0);
        let fut = super::create(
            "base_url",
            "peer_id",
            true,
            tx,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        );

        let events_future = async {
            let event = event_listener.next().await;
            assert!(event.is_none());
        };

        let (_, result) = join!(events_future, fut);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_error() {
        // it mocks that webrtc gateway failed to create peer object
        let inject_api_create_peer = move |_base_url: &str,
                                           peer_id: &str,
                                           _turn: bool|
              -> Result<CreatedResponse, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        // If create_peer failed, event is never listened
        let inject_api_event = {
            || {
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    unreachable!();
                }
            }
        }();

        let (tx, mut event_listener) = mpsc::channel::<PeerEventEnum>(0);
        let fut = super::create(
            "base_url",
            "peer_id",
            true,
            tx,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        );

        let events_future = async {
            let event = event_listener.next().await;
            assert!(event.is_none());
        };

        let (_, result) = join!(events_future, fut);
        assert!(result.is_err());
    }
}
