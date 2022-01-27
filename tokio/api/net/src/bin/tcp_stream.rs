use std::io::Result;
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    let mut buf:[u8; 4] = [1,2,3,4];
    stream.write(&buf[..3]).await?;
    stream.flush().await?;
    println!("write finish");
    let size = stream.read(&mut buf).await?;
    for i in 0..size {
        println!("read byte {}", buf[i]);
    }
    Ok(())
}