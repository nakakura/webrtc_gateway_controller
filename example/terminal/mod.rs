use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

#[allow(dead_code)]
pub async fn read_stdin(
    tx: mpsc::Sender<String>,
) -> Result<(), Box<mpsc::error::SendError<String>>> {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        let message = std::str::from_utf8(&buf[0..n]).unwrap().trim().to_string();
        tx.send(message.clone()).await.map_err(|e| Box::new(e))?;
        if message == "exit" {
            break;
        }
    }
    Ok(())
}
