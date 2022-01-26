use futures::{FutureExt, pin_mut, Stream, stream::FusedStream, StreamExt};

#[allow(dead_code)]
async fn async_select_one() {
    let f1 = async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("f1");
    }.fuse();
    
    let f2 = async {
        println!("f2");
    }.fuse();

    pin_mut!(f1, f2);
    
    // 只执行一个
    futures::select! {
        _ = f1 => println!("f1 finished first"),
        _ = f2 => println!("f2 finished first"),
    }
}

#[allow(dead_code)]
async fn async_select_all_1() {
    let mut ready_a = futures::future::ready(1);
    let mut ready_b  = futures::future::ready(2);
    loop {
        futures::select! {
            val_a = ready_a =>  println!("val_a:{:?}", val_a),
            val_b = ready_b => println!("val_b:{:?}", val_b),
            complete => {println!("complete"); break;},
            default => unreachable!(),
        };
    }
    /*
    val_a:1
    val_b:2
    complete
    */
}

#[allow(dead_code)]
async fn async_select_all_2() {
    let mut ready_a = async {
        println!("async a");
        1
    }.fuse();
    let mut ready_b  = async {
        println!("async b");
        "hello"
    }.fuse();
    pin_mut!(ready_a, ready_b);
    loop {
        futures::select! {
            val_a = ready_a =>  println!("val_a:{:?}", val_a),
            val_b = ready_b => println!("val_b:{:?}", val_b),
            complete => {println!("complete");break;},
            default => unreachable!(),
        };
    }
    /*
    async a
    val_a:1
    async b
    val_b:"hello"
    complete
    */
}

#[allow(dead_code)]
async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = futures::select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

#[test]
fn test() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_select_all_2());
}