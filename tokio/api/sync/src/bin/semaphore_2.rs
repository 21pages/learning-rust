use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // perform task...
            // explicitly own `permit` in the task
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}