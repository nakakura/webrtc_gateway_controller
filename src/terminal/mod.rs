use std::time::Duration;

use futures::channel::mpsc;
use futures::*;
use tokio::time::timeout;

use crate::error;

// FIXME
// Keyboard events should be subscribed from many locations
// The struct should implement addEventListener and deleteEventListener.
pub async fn read(mut observer: mpsc::Sender<String>) -> Result<(), error::ErrorEnum> {
    let stdin = async_std::io::stdin();
    let mut line = String::new();
    while let _n = stdin.read_line(&mut line).await? {
        observer.send(line.trim().to_string()).await;
        line.clear()
    }
    Ok(())
}