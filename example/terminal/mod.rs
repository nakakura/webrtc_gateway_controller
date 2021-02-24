use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn read_stdin(mut tx: mpsc::Sender<String>) -> Result<(), Box<mpsc::error::SendError<String>>> {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        let str = std::str::from_utf8(&buf[0..n]);
        tx.send(str.unwrap().into()).await.map_err(|e| Box::new(e))?;
    }
    Ok(())
}
