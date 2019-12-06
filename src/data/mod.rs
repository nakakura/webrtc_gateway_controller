pub mod api;
pub mod formats;

use futures::channel::mpsc::*;
use futures::*;

pub async fn listen_events(
    base_url: &str,
    data_connection_id: &str,
    mut on_open_tx: Sender<String>,
    mut on_close_tx: Sender<String>,
    mut on_error_tx: Sender<(String, String)>,
) {
    loop {
        match api::event(base_url, data_connection_id).await {
            Ok(formats::DataConnectionEventEnum::OPEN) => {
                if on_open_tx
                    .send(data_connection_id.to_string())
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok(formats::DataConnectionEventEnum::CLOSE) => {
                if on_close_tx
                    .send(data_connection_id.to_string())
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok(formats::DataConnectionEventEnum::ERROR {
                error_message: message,
            }) => {
                if on_error_tx
                    .send((data_connection_id.to_string(), message))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok(formats::DataConnectionEventEnum::TIMEOUT) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    ()
}
