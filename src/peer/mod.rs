pub mod api;
pub mod formats;

use futures::*;

use formats::*;

pub async fn listen_events(
    base_url: &str,
    peer_info: PeerInfo,
    mut on_open_tx: futures::channel::mpsc::Sender<PeerOpenEvent>,
    mut on_call_tx: futures::channel::mpsc::Sender<PeerCallEvent>,
    mut on_connect_tx: futures::channel::mpsc::Sender<PeerConnectionEvent>,
    mut on_close_tx: futures::channel::mpsc::Sender<PeerCloseEvent>,
    mut on_error_tx: futures::channel::mpsc::Sender<PeerErrorEvent>,
) -> impl Future<Output = ()> {
    loop {
        match api::event(base_url, &peer_info).await {
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
