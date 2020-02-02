use futures::channel::mpsc;
use futures::*;

use skyway_webrtc_gateway_api::error;

// FIXME
// Keyboard events should be subscribed from many locations
// The struct should implement addEventListener and deleteEventListener.
pub async fn read(mut observer: mpsc::Sender<String>) -> Result<(), error::Error> {
    let stdin = async_std::io::stdin();
    let mut line = String::new();
    loop {
        let _n = stdin.read_line(&mut line).await?;
        let message = line.trim().to_string();
        println!("{:?}", message);
        let _ = observer
            .send(message.clone())
            .await
            .expect("terminal error");
        if message == "exit" {
            break;
        }
        line.clear()
    }
    Ok(())
}
