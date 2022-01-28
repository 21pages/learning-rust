use tokio::sync::mpsc;

/*
一般不用无边界的
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        for i in 1..=10 {
            if let Err(e) = tx.send(i) {
                println!("{:#?}", e);
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("recv:{}", i);
    }

    Ok(())
}
