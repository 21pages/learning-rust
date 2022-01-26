use futures;
use tokio::runtime::Runtime;
use log::{trace};

async fn func1() {
    trace!("func1 start");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    trace!("func1 end");
}

async fn func2() {
    trace!("func2");
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();
    futures::join!(f1, f2);
    trace!("after join finish");
}

#[test]
fn main() {
    let logenv = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_WRITE_SYTLE", "always");
    env_logger::init_from_env(logenv);
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
}