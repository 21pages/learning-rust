use tokio::time;

use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(1));

    let now = time::Instant::now();
    interval.tick().await;
    interval.tick().await; //这个上面的才执行
    interval.tick().await;
    let newnow = time::Instant::now();
    let diff = newnow.saturating_duration_since(now);
    println!("{:?}", diff); //2.0147102s

    // approximately 20ms have elapsed.
}
