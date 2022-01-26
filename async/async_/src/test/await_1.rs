use futures::{self, executor::block_on};
// use std::thread;
// use std::time::Duration;

async fn dance() {
    println!("dance");
}
async fn learn_song() -> &'static str {
    println!("learn song");
    "melody"
}
async fn sing_song(song: &str) {
    println!("sing {}", song);
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    // thread::sleep(Duration::from_secs(1)); //异步编程不能sleep
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.

    futures::join!(f1, f2);
    // f1.await;
    // f2.await;
}

#[test]
fn test() {
    block_on(async_main());
}
