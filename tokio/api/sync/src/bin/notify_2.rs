use std::{ops::Add, sync::Arc};
use tokio::{
    sync::Notify,
    time::{sleep_until, Duration, Instant},
};

/*
notify_waiters:
The purpose of this method is to notify all already registered waiters
*/

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified1 = notify.notified(); //这里是注册, 注册过的都会被通知到
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
