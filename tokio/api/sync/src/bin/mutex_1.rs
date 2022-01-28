/*
与std::sync::Mutex区别:
1.不会阻塞
2.MutexLockGuard 可以在await里使用

应该选用哪种锁:
一般使用std锁,在需要异步的情况下使用Arc<Mutex<T>>>, 如果不用Arc, 会死锁
只有在共享io资源如数据库连接的情况下需要使用tokio锁, 但是也应该使用消息传递, 而不是锁
*/

use std::sync::Arc;
// use std::sync::Mutex;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1 = Arc::new(Mutex::new(0));
    // let data2 = data1.clone();
    let data2 = Arc::clone(&data1); //直接clone和Arc::clone效果一样

    let join_handle = tokio::spawn(async move {
        let mut lock = data1.lock().await;
        *lock += 1;
        println!("in {}", *lock);
    });

    let mut lock = data2.lock().await;
    *lock += 1;
    println!("out {}", *lock);
    drop(lock);

    let _ = join_handle.await;
    Ok(())
}

/*
还是应该用std

std:
out 1
in 2

sync:
out 1

用sync需要把外面的lock drop掉里面的才能获得锁
*/
