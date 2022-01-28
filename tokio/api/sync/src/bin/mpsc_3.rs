use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:6379").await?;
    let (tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the
        // original handle.
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.
    drop(tx); //不drop多余的tx不会结束

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}
