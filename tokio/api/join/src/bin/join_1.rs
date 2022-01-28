async fn do_stuff_async() {
    println!("a");
}

async fn more_async_work() {
    println!("b");
}

#[tokio::main]
async fn main() {
    let (_first, _second) = tokio::join!(do_stuff_async(), more_async_work());

    println!("c");
}
