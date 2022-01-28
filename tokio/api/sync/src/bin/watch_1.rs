/*
A single-producer, multi-consumer channel that only retains the last sent value.

This channel is useful for watching for changes to a value from multiple points in the code base, for example, changes to configuration values.

单发多收, 并且只能收到最新的一条
*/
use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = watch::channel::<i32>(0);
    (1..5).for_each(|i| {
        let _ = tx.send(i);
    });
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received: {}", *rx.borrow());
        }
    });
    let _ = tx.send(5);

    Ok(())
}

/*
received: 5
*/
