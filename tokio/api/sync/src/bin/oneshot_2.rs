use tokio::sync::oneshot;

/*
一对一
tx drop后rx会收到Err
rx drop后tx发送会返回Err
*/

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    std::mem::drop(rx);
    tokio::spawn(async move {
        if let Err(e) = tx.send(3) {
            println!("send err {:?}", e)
        } else {
            println!("send ok")
        }
    });

    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

/*
end err 3
*/
