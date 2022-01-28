use tokio::{
    sync::Notify, 
    time::{sleep_until, Instant, Duration}
};
use std::{sync::Arc, ops::Add};

/*
notify_waiters:
The purpose of this method is to notify all already registered waiters
*/

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified1 = notify.notified();
    let notified2 = notify.notified();

    let _handle = tokio::spawn(async move {
        let _ = sleep_until(Instant::now().add(Duration::from_secs(1))).await;
        println!("sending notifications");
        notify2.notify_waiters();
    });

    println!("wait 1");
    notified1.await; //延迟调用
    let _ = sleep_until(Instant::now().add(Duration::from_secs(1))).await;
    println!("wait 2");
    notified2.await;
    println!("received notifications");
}