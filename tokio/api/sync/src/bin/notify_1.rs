/*
Notifies a single task to wake up.

Notify provides a basic mechanism to notify a single task of an event. Notify itself does not carry any data. Instead, it is to be used to signal another task to perform an operation.

Notify can be thought of as a Semaphore starting with 0 permits. notified().await waits for a permit to become available, and notify_one() sets a permit if there currently are no available permits.

The synchronization details of Notify are similar to thread::park and Thread::unpark from std. A Notify value contains a single permit. notified().await waits for the permit to be made available, consumes the permit, and resumes. notify_one() sets the permit, waking a pending task if there is one.

If notify_one() is called before notified().await, then the next call to notified().await will complete immediately, consuming the permit. Any subsequent calls to notified().await will wait for a new permit.

If notify_one() is called multiple times before notified().await, only a single permit is stored. The next call to notified().await will complete immediately, but the one after will wait for a new permit.
*/

use std::{error::Error, sync::Arc};
use tokio::{sync::Notify, time::{sleep, Duration}};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let notify1 = Arc::new(Notify::new());
    let notify2 = notify1.clone();
    tokio::spawn(async move {
        println!("wait to be notified");
        notify1.notified().await;
        println!("notified");
    });
    println!("before sleep");
    let _ = sleep(Duration::from_secs(1)).await; //别忘了await
    println!("after sleep");
    // notify2.notify_one(); //one
    println!("notify");
    notify2.notify_waiters(); //all

    Ok(())
}

/*
before sleep
wait to be notified
after sleep
notify
notified
*/