use bytes::Bytes;
use mini_redis::client;
use std::{io::Result};
use tokio::{sync::{mpsc, oneshot}};

/*
tokio的channel:

mpsc: 多对一
oneshot: 一对一
broadcast: 多对多, 每个rx都能收到
watch: 一对多, 但是只能收到最新的一条消息

如果想要 多对多, 每个生产者的消息只能被消费一次, 那就使用async-channel这个库

std::sync::channel 是阻塞的, 不能异步

*/

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;//结果收发的channel, 一对一
enum Command {
    Get {
        key:String,
        resp:Responder<Option<Bytes>>,
    },

    Set {
        key:String,
        val:Bytes,
        resp:Responder<()>,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    const CHAN_SIZE : usize = 10;
    let (tx_set, mut rx) = mpsc::channel(CHAN_SIZE);
    let tx_get = tx_set.clone();

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            let mut client = client::connect("127.0.0.1:1234").await.unwrap(); //我只能每次建立一个连接
            match cmd {
                Command::Get{key, resp} => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                },
                Command::Set{key, val, resp} => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);

                }
            }
        }
    });

    let task_set = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key : "hello".to_string(),
            val : Bytes::from("world"),
            resp :resp_tx,
        };
        if tx_set.send(cmd).await.is_err() {
            eprintln!("set send failed");
            return;
        }
        let ret = resp_rx.await;
        println!("set ret:{:?}", ret);
    });

    let task_get = tokio::spawn(async move {
        let (resp_tx,resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key:"hello".to_string(),
            resp:resp_tx,
        };
        if tx_get.send(cmd).await.is_err() {
            eprint!("get send failed");
            return;
        }
        let ret = resp_rx.await;
        println!("get ret:{:?}", ret);
        
    });

    task_set.await.unwrap();
    task_get.await.unwrap();
    manager.await.unwrap();

    Ok(())
}

/*
output:
set ret:Ok(Ok(()))
get ret:Ok(Ok(Some(b"world")))
*/