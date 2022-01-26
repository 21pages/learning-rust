use futures_util::{SinkExt, StreamExt};
use tokio::{self, io, net::TcpListener};
use tokio_tungstenite::accept_async;
use tungstenite::{error::Error, Message};

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:9007").await.unwrap();

    while let Ok((stream, _addr)) = tcp_listener.accept().await {
        tokio::spawn(accetp_connection(stream));
    }

    Ok(())
}

async fn accetp_connection(stream: tokio::net::TcpStream) -> tungstenite::Result<()> {
    //Accepts a new WebSocket connection with the provided stream.
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1)); //秒定时器
    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break Err(Error::ConnectionClosed)
                        }
                    }
                    None => {

                    }
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }
}
