use tokio::sync::mpsc;

/*
多发单收, 如果创建了发送者, 不发送, 接收不会结束
接收结束会收到一个None
默认有缓冲大小, 到了顶会阻塞等待消费
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx1, mut rx) = mpsc::channel::<i32>(5);
    // let _tx2 = tx1.clone();

    tokio::spawn(async move {
        for i in 1..=10 {
            let _ = tx1.send(i).await;
            println!("1 send {}", i);
        }

        //如果tx2不发送, rx将一直等待
        // for i in 1..=10 {
        //     if let Err(e) = tx2.send(i).await {
        //         println!("{:#?}", e);
        //     }
        // }
    });

    let _ = tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // while let Some(i) = rx.recv().await {
    //     println!("recv:{}", i);
    // }
    for _ in 0..11 {
        match rx.recv().await {
            Some(i) => println!("recv:{}", i),
            None => println!("recv None"),
        }
    }

    Ok(())
}
/*
1 send 1
1 send 2
1 send 3
1 send 4
1 send 5
recv:1
recv:2
recv:3
1 send 6
recv:4
recv:5
recv:6
recv:7
1 send 7
1 send 8
1 send 9
1 send 10
recv:8
recv:9
recv:10
recv None
*/
