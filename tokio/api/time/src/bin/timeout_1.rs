use tokio::sync::oneshot;
use tokio::time::timeout;

use std::time::Duration;

#[tokio::main]
async fn main() {
    let (_tx, rx) = oneshot::channel::<i32>();

    // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
    if let Err(_) = timeout(Duration::from_millis(10), rx).await {
        println!("did not receive value within 10 ms");
    }
}
