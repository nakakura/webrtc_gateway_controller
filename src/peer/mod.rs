pub mod api;
pub mod formats;

use futures::*;

#[cfg(test)]
use crate::error;
use formats::*;

pub async fn listen_events(
    base_url: &str,
    peer_info: PeerInfo,
    mut on_open_tx: futures::channel::mpsc::Sender<PeerOpenEvent>,
    mut on_call_tx: futures::channel::mpsc::Sender<PeerCallEvent>,
    mut on_connect_tx: futures::channel::mpsc::Sender<PeerConnectionEvent>,
    mut on_close_tx: futures::channel::mpsc::Sender<PeerCloseEvent>,
    mut on_error_tx: futures::channel::mpsc::Sender<PeerErrorEvent>,
    #[cfg(test)] mut inject_api: Box<
        dyn FnMut(&str, &PeerInfo) -> Result<PeerEventEnum, error::ErrorEnum>,
    >,
) -> impl Future<Output = ()> {
    loop {
        #[cfg(test)]
        let result = inject_api(base_url, &peer_info);
        #[cfg(not(test))]
        let result = api::event(base_url, &peer_info).await;

        match result {
            Ok(PeerEventEnum::OPEN(event)) => {
                if on_open_tx.send(event).await.is_err() {
                    break;
                };
            }
            Ok(PeerEventEnum::CALL(event)) => {
                if on_call_tx.send(event).await.is_err() {
                    break;
                };
            }
            Ok(PeerEventEnum::CONNECTION(event)) => {
                if on_connect_tx.send(event).await.is_err() {
                    break;
                };
            }
            Ok(PeerEventEnum::CLOSE(event)) => {
                let _ = on_close_tx.send(event).await;
                break;
            }
            Ok(PeerEventEnum::TIMEOUT) => {}
            Ok(PeerEventEnum::ERROR(e)) => {
                let _ = on_error_tx.send(e).await;
                break;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    future::ready(())
}

#[cfg(test)]
mod test_listen_peer {
    use futures::*;

    use super::*;
    use crate::error;

    #[tokio::test]
    async fn ten_timeout_and_open() {
        //set up callbacks
        let (on_open_tx, on_open_rx) = futures::channel::mpsc::channel::<formats::PeerOpenEvent>(0);
        let on_open_future = on_open_rx.for_each(on_open);
        let (on_call_tx, on_call_rx) = futures::channel::mpsc::channel::<formats::PeerCallEvent>(0);
        let on_call_future = on_call_rx.for_each(on_call);
        let (on_connect_tx, on_connect_rx) =
            futures::channel::mpsc::channel::<formats::PeerConnectionEvent>(0);
        let on_connect_future = on_connect_rx.for_each(on_connect);
        let (on_close_tx, on_close_rx) =
            futures::channel::mpsc::channel::<formats::PeerCloseEvent>(0);
        let on_close_future = on_close_rx.for_each(on_close);
        let (on_error_tx, on_error_rx) =
            futures::channel::mpsc::channel::<formats::PeerErrorEvent>(0);
        let on_error_future = on_error_rx.for_each(on_error);
        tokio::spawn(on_open_future.map(|_| ()));
        tokio::spawn(on_call_future.map(|_| ()));
        tokio::spawn(on_connect_future.map(|_| ()));
        tokio::spawn(on_close_future.map(|_| ()));
        tokio::spawn(on_error_future.map(|_| ()));

        //set up parameters
        let base_url = "http://localhost:8000".to_string();
        let peer_info = PeerInfo {
            peer_id: "hoge".to_string(),
            token: "hoge".to_string(),
        };

        //set up an inject function
        //this function returns TIMEOUT 10 times, after that it returns CLOSE
        let inject_api = {
            || {
                let mut counter = 0u16;
                move |_base_url: &str,
                      _peer_info: &PeerInfo|
                      -> Result<PeerEventEnum, error::ErrorEnum> {
                    let peer_info = PeerInfo {
                        peer_id: "hoge".to_string(),
                        token: "hoge".to_string(),
                    };
                    counter += 1;
                    if counter < 10 {
                        Ok(PeerEventEnum::TIMEOUT)
                    } else if counter == 10 {
                        Ok(PeerEventEnum::CLOSE(PeerCloseEvent{ params: peer_info }))
                    } else {
                        unreachable!();
                    }
                }
            }
        }();
        let listen_event_future = listen_events(
            &base_url,
            peer_info,
            on_open_tx,
            on_call_tx,
            on_connect_tx,
            on_close_tx,
            on_error_tx,
            Box::new(inject_api),
        );

        let _ = listen_event_future.await;
    }

    // dummy functions
    fn on_open(_event: formats::PeerOpenEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    // dummy functions
    fn on_call(_event: formats::PeerCallEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    // dummy functions
    fn on_connect(_event: formats::PeerConnectionEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    // dummy functions
    fn on_close(_event: formats::PeerCloseEvent) -> impl Future<Output = ()> {
        future::ready(())
    }

    // dummy functions
    fn on_error(_event: formats::PeerErrorEvent) -> impl Future<Output = ()> {
        future::ready(())
    }
}
