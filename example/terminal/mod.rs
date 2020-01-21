use futures::channel::mpsc;
use futures::*;

use webrtc_gateway_controller::error;

// FIXME
// Keyboard events should be subscribed from many locations
// The struct should implement addEventListener and deleteEventListener.
pub async fn read(mut observer: mpsc::Sender<String>) -> Result<(), error::ErrorEnum> {
    let stdin = async_std::io::stdin();
    let mut line = String::new();
    while let _n = stdin.read_line(&mut line).await? {
        let message = line.trim().to_string();
        observer.send(message.clone()).await;
        if message == "exit" {
            break;
        }
        line.clear()
    }
    Ok(())
}