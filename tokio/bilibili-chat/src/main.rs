use futures_util::{SinkExt, StreamExt};
use tokio::{self, net::TcpListener, sync::broadcast};
use tokio_tungstenite::accept_async;
use tungstenite::{error::Error, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_listener = TcpListener::bind("127.0.0.1:9007").await.unwrap();
    let (sender, receiver) = broadcast::channel::<ChatMessage>(10);

    while let Ok((stream, _addr)) = tcp_listener.accept().await {
        tokio::spawn(accetp_connection(stream, sender.clone()));
    }

    Ok(())
}

extern "C" {
    fn time(output: *mut i64) -> i64;
}

async fn accetp_connection(
    stream: tokio::net::TcpStream,
    sender: broadcast::Sender<ChatMessage>,
) -> tungstenite::Result<()> {
    //Accepts a new WebSocket connection with the provided stream.
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10)); //秒定时器
    let mut receiver = sender.subscribe();
    let mut last_pong_receive_timestamp: i64 = unsafe { time(std::ptr::null_mut()) };
    loop {
        //同时观察, websocket, channel, timer
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() {
                            if let Message::Text(msg) = msg {
                                let (cmd, arg) = msg.split_once(' ').unwrap();
                                match cmd {
                                    "send" => {
                                        let msg = ChatMessage {
                                            message:arg.to_string()
                                        };
                                        sender.send(msg).unwrap();

                                    },
                                    _ => {},
                                }
                                ws_sender.send(Message::Text(msg)).await?;
                            }

                        } else if msg.is_ping() {
                            ws_sender.send(Message::Pong(Vec::new())).await?;
                        } else if msg.is_pong() {
                            // last_pong_receive_timestamp = unsafe {

                            // }
                        }
                        else if msg.is_close() {
                            break Err(Error::ConnectionClosed)
                        }
                    }
                    None => {

                    }
                }
            }
            Ok(msg) = receiver.recv() => {
                ws_sender.send(Message::Text(msg.message)).await?;
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Ping(vec![])).await?;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ChatMessage {
    // username: String,
    message: String,
}
