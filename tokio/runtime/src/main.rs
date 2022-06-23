#[tokio::main]
async fn bar() -> i32 {
    1
}

#[tokio::main]
async fn foo() -> i32 {
    // std::thread::spawn(|| bar()).join().expect("hello")
    // std::thread::spawn(|| bar()).join().map_or(2, |i| i)
    tokio::task::spawn_blocking(|| bar()).await.map_or(2, |i| i)
}

fn foo2() -> i32 {
    // std::thread::spawn(|| bar()).join().expect("hello")
    std::thread::spawn(|| bar()).join().map_or(2, |i| i)
}

fn main() {
    let result = foo();
    println!("result:{}", result);

    let result = foo2();
    println!("result:{}", result);
}
