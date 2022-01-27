use mini_redis::{Connection, Frame};
use tokio::{net::{TcpListener, TcpStream}};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    loop {
         let (stream, _addr) = listener.accept().await?;
        if let Err(_) = process(stream).await {
            break;
        }
    }

    Ok(())
}

async fn process(stream : TcpStream) -> mini_redis::Result<()>{
    let mut conn = Connection::new(stream);
    if let Some(frame) = conn.read_frame().await? {
        println!("read Frame:{:?}", frame);//read Frame:Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])
        let response = Frame::Error("unimplemented".into());
        conn.write_frame(&response).await?;
    }
    Ok(())
}