use tokio::time::{self, Duration};
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3]);
    let sleep = time::sleep(Duration::from_secs(1));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            maybe_v = stream.next() => {
                if let Some(v) = maybe_v {
                    println!("got = {}", v);
                } else {
                    break;
                }
            }
            _ = &mut sleep => {
                println!("timeout");
                break;
            }
        }
    }
}
/*
got = 1
got = 2
got = 3
*/
