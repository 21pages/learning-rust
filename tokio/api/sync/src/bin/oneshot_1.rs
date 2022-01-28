use tokio::sync::oneshot;

/*
一对一
tx drop后rx会收到Err
rx drop后tx发送会返回Err
rx不用调用recv, 直接await, 返回一个Result
*/

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}

/*
got = 3
*/
