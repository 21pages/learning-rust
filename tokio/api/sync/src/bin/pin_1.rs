use tokio::{pin, select};
use tokio_stream::{self as stream, StreamExt};

async fn my_async_fn() {
    println!("async fn");
}

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3, 4]);

    let future = my_async_fn();
    pin!(future); //固定不动

    loop {
        select! {
            _ = &mut future => {
                // Stop looping `future` will be polled after completion
                break;
            }
            Some(val) = stream.next() => {
                println!("got value = {}", val);
            }
        }
    }
}
