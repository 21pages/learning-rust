use mini_redis::{Connection, Frame, Command};
use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};
use std::{io, collections::HashMap};
use std::sync::{Arc};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
/*
锁如果在 .await 的过程中持有，应该使用 Tokio 提供的锁，原因是 .await的过程中锁可能在线程间转移，若使用标准库的同步锁存在死锁的可能性，例如某个任务刚获取完锁，还没使用完就因为 .await 让出了当前线程的所有权，结果下个任务又去获取了锁，造成死锁
锁竞争不多的情况下，使用 std::sync::Mutex
锁竞争多，可以考虑使用三方库提供的性能更高的锁，例如 parking_lot::Mutex
*/


#[tokio::main]
async fn main() -> io::Result<()>{
    let db = Arc::new(Mutex::new(HashMap::<String, Bytes>::new()));
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    loop {
         let (stream, _addr) = listener.accept().await?;
         let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = process(db, stream).await {
                eprintln!("addr:{:?}, err:{:}", _addr, e);
            }
        });
        println!("after spawn");
    }
}

async fn process(db:Db, stream : TcpStream) -> mini_redis::Result<()>{
    let mut conn = Connection::new(stream);
    if let Some(frame) = conn.read_frame().await? {
        println!("read Frame:{:?}", frame);//read Frame:Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])
        let response = match Command::from_frame(frame)? {
            Command::Get(cmd) => {
                let db = db.try_lock()?;
                if let Some(value) = db.get(cmd.key()) {
                    println!("get some");
                    Frame::Bulk(value.clone())
                } else {
                    println!("get none");
                    Frame::Null
                }
            },
            Command::Publish(_) => todo!(),
            Command::Set(cmd) => {
                let mut db = db.try_lock()?;
                db.insert(cmd.key().to_string(), cmd.value().clone());
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