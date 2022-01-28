use tokio::{
    sync::oneshot,
    time::{interval, sleep, sleep_until, timeout, Instant},
};

use std::{ops::Add, time::Duration};

//sleep自己能运行, 加上其它任务就不能运行了, 不要sleep
// async fn work() -> String {
//     // sleep(Duration::from_secs(1)).await;
//     // let mut interval = interval(Duration::from_secs(1));
//     // interval.tick().await;
//     // interval.tick().await;
//     // let (_tx, rx) = oneshot::channel::<i32>();
//     // let _ = timeout(Duration::from_secs(1), rx).await;
//     "good work".to_string()
// }

#[tokio::main]
async fn main() {
    let start_instant = Instant::now();

    //定时器
    let mut interval = interval(Duration::from_secs(1));

    //休息
    let xiuxi = sleep_until(Instant::now().add(Duration::from_secs(1)));
    tokio::pin!(xiuxi);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("{:?} tick", Instant::now().saturating_duration_since(start_instant));
            }
            _ = &mut xiuxi => {
                println!("{:?} xiuxi", Instant::now().saturating_duration_since(start_instant));
                xiuxi.as_mut().reset(Instant::now().add(Duration::from_secs(1)))
            }
            // s = work() => {
            //     println!("{}", s);
            // }
        }
    }
}
