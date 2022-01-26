// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheudling
// multiple futures onto the same thread.
use futures::executor::block_on;

//async 异步函数
async fn hello_world() {
    println!("hello, world!");
}

#[test]
fn test() {
    let future = hello_world(); // Nothing is printed
    println!("before block on");
    //直到block_on future才执行, 并阻塞当前线程
    block_on(future); // `future` is run and "hello, world!" is printed
    println!("after block on");
}
