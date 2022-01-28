use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    sleep(Duration::new(1, 0)).await;
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now)); //Some(1.0117464s)
    println!("{:?}", now.checked_duration_since(new_now)); // None
}
