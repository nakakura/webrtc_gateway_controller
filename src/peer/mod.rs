/// Functions in this module are responsible for concealing the raw APIs
pub mod api;
pub mod formats;

use futures::*;

#[cfg(test)]
use crate::common::PeerInfo;
use crate::error;
use formats::*;

/// FIXME
/// all the peer events should be handled in one method.
/// also, all the keyboard events should be handled in the same method.
/// the method will work in and_then block of join(keyboard_events, peer_events).forld(status, |status, event|{})
pub async fn peer_create_and_listen_events<'a>(
    base_url: &str,
    peer_id: &str,
    turn: bool,
    mut on_open_tx: Option<futures::channel::mpsc::Sender<PeerOpenEvent>>,
    mut on_call_tx: Option<futures::channel::mpsc::Sender<PeerCallEvent>>,
    mut on_connect_tx: Option<futures::channel::mpsc::Sender<PeerConnectionEvent>>,
    mut on_close_tx: Option<futures::channel::mpsc::Sender<PeerCloseEvent>>,
    mut on_error_tx: Option<futures::channel::mpsc::Sender<PeerErrorEvent>>,
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

    #[cfg(test)]
    let result = inject_api_events(base_url, &peer_info);
    #[cfg(not(test))]
    let result = api::event(base_url, &peer_info).await;

    if let Ok(PeerEventEnum::OPEN(event)) = result {
        if let Some(ref mut tx) = on_open_tx {
            if tx.send(event).await.is_err() {
                return Err(error::ErrorEnum::create_myerror("peer_create_and_listen_events send OPEN event, but observer doesn't receive i, but observer doesn't receive it."));
            };
        }
    } else {
        return Err(error::ErrorEnum::create_myerror(
            "create peer success, but WebRTC Gateway doesn't return OPEN event",
        ));
    }

    loop {
        #[cfg(test)]
        let result = inject_api_events(base_url, &peer_info);
        #[cfg(not(test))]
        let result = api::event(base_url, &peer_info).await;

        match result {
            Ok(PeerEventEnum::OPEN(_event)) => {
                unreachable!();
            }
            Ok(PeerEventEnum::CALL(event)) => {
                if let Some(ref mut tx) = on_call_tx {
                    if tx.send(event).await.is_err() {
                        break;
                    };
                }
            }
            Ok(PeerEventEnum::CONNECTION(event)) => {
                if let Some(ref mut tx) = on_connect_tx {
                    if tx.send(event).await.is_err() {
                        break;
                    };
                }
            }
            Ok(PeerEventEnum::CLOSE(event)) => {
                if let Some(ref mut tx) = on_close_tx {
                    let _ = tx.send(event).await;
                }
                break;
            }
            Ok(PeerEventEnum::TIMEOUT) => {}
            Ok(PeerEventEnum::ERROR(e)) => {
                if let Some(ref mut tx) = on_error_tx {
                    let _ = tx.send(e).await;
                }
                break;
            }
            Err(e) => return Err(e),
        }
    }
    println!("break");
    Ok(())
}

#[cfg(test)]
mod test_peer_create_and_listen_events {
    use futures::channel::mpsc::*;
    use futures::*;

    use super::*;
    use crate::common::{PeerId, PeerInfo, Token};
    use crate::error;

    #[tokio::test]
    async fn create_error() {
        // create_peer api mock, returns 404 error
        let inject_api_create_peer = move |_base_url: &str,
                                           _peer_id: &str,
                                           _turn: bool|
              -> Result<CreatedResponse, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     _peer_info: &PeerInfo|
              -> Result<PeerEventEnum, error::ErrorEnum> {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        };
        let result = super::peer_create_and_listen_events(
            "base_url",
            "peer_id",
            true,
            None,
            None,
            None,
            None,
            None,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_success_and_not_recv_on_open_but_error() {
        // create_peer api mock, returns 404 error
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
        // event api mock, returns 404 error
        let inject_api_event = move |_base_url: &str,
                                     peer_info: &PeerInfo|
              -> Result<PeerEventEnum, error::ErrorEnum> {
            assert_eq!(
                peer_info,
                &PeerInfo {
                    peer_id: PeerId("peer_id".to_string()),
                    token: Token("token".to_string()),
                }
            );
            Ok(PeerEventEnum::ERROR(PeerErrorEvent {
                params: peer_info.clone(),
                error_message: "error".to_string(),
            }))
        };
        let (on_error_tx, mut on_error_rx) = channel::<PeerErrorEvent>(0);
        tokio::spawn(async move {
            let _ = on_error_rx
                .next()
                .map(|result| {
                    assert_eq!(
                        result,
                        Some(PeerErrorEvent {
                            params: PeerInfo {
                                peer_id: PeerId("peer_id".to_string()),
                                token: Token("token".to_string()),
                            },
                            error_message: "error".to_string()
                        })
                    );
                })
                .await;
        });
        let result = super::peer_create_and_listen_events(
            "base_url",
            "peer_id",
            true,
            None,
            None,
            None,
            None,
            Some(on_error_tx),
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_success_and_not_recv_on_open_but_timeout() {
        // create_peer api mock, returns 404 error
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
        //this function returns TIMEOUT in 1st call, and CLOSE 2nd call
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    let peer_info = PeerInfo {
                        peer_id: PeerId("hoge".to_string()),
                        token: Token("token".to_string()),
                    };
                    counter += 1;
                    if counter == 1 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 2 {
                        Ok(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();
        let (on_close_tx, mut on_close_rx) = channel::<PeerCloseEvent>(0);
        tokio::spawn(async move {
            let _ = on_close_rx
                .next()
                .map(|result| {
                    assert_eq!(
                        result,
                        Some(PeerCloseEvent {
                            params: PeerInfo {
                                peer_id: PeerId("hoge".to_string()),
                                token: Token("token".to_string()),
                            }
                        })
                    );
                })
                .await;
        });
        let result = super::peer_create_and_listen_events(
            "base_url",
            "peer_id",
            true,
            None,
            None,
            None,
            Some(on_close_tx),
            None,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_success_and_recv_on_open_timeout_close() {
        // create_peer api mock, returns 404 error
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
        //this function returns TIMEOUT in 1st call, and CLOSE 2nd call
        let inject_api_event = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    let peer_info = PeerInfo {
                        peer_id: PeerId("hoge".to_string()),
                        token: Token("token".to_string()),
                    };
                    counter += 1;
                    if counter == 1 {
                        Ok(PeerEventEnum::OPEN(PeerOpenEvent { params: peer_info }))
                    } else if counter == 2 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 3 {
                        Ok(PeerEventEnum::CLOSE(PeerCloseEvent { params: peer_info }))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();
        let (on_close_tx, mut on_close_rx) = channel::<PeerCloseEvent>(0);
        tokio::spawn(async move {
            let _ = on_close_rx
                .next()
                .map(|result| {
                    assert_eq!(
                        result,
                        Some(PeerCloseEvent {
                            params: PeerInfo {
                                peer_id: PeerId("hoge".to_string()),
                                token: Token("token".to_string()),
                            }
                        })
                    );
                })
                .await;
        });
        let result = super::peer_create_and_listen_events(
            "base_url",
            "peer_id",
            true,
            None,
            None,
            None,
            Some(on_close_tx),
            None,
            Box::new(inject_api_create_peer),
            Box::new(inject_api_event),
        )
        .await;
        assert_eq!(result.ok(), Some(()));
    }
}
