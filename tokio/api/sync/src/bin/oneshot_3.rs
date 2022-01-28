use tokio::sync::oneshot;

/*
一对一
tx drop后rx会收到Err
rx drop后tx发送会返回Err
*/

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<i32>();

    tokio::spawn(async move {
        std::mem::drop(tx);
    });
    if let Ok(v) = rx.await {
        println!("got = {:?}", v);
    } else {
        println!("the sender dropped");
    }
}

/*
the sender dropped
*/
