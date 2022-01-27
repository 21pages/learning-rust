use mini_redis::{Connection, Frame, Command};
use tokio::{net::{TcpListener, TcpStream}};
use std::{io, collections::HashMap};

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    loop {
         let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                eprintln!("addr:{:?}, err:{:}", _addr, e);
            }
        });
        println!("after spawn");
    }
}

async fn process(stream : TcpStream) -> mini_redis::Result<()>{
    let mut db = HashMap::<String, Vec<u8>>::new();
    let mut conn = Connection::new(stream);
    if let Some(frame) = conn.read_frame().await? {
        println!("read Frame:{:?}", frame);//read Frame:Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])
        let response = match Command::from_frame(frame)? {
            Command::Get(cmd) => {
                // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                if let Some(value) = db.get(cmd.key()) {
                    println!("get some");
                    Frame::Bulk(value.clone().into())
                } else {
                    println!("get none");
                    Frame::Null
                }
            },
            Command::Publish(_) => todo!(),
            Command::Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Command::Subscribe(_) => todo!(),
            Command::Unsubscribe(_) => todo!(),
            Command::Unknown(_) => todo!(),
        };
        conn.write_frame(&response).await?;
    }
    Ok(())
}