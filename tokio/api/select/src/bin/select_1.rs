/*
等待多个并发分支，当第一个分支完成时返回，取消其余分支。
The select! macro must be used inside of async functions, closures, and blocks.
必须在异步函数, 异步闭包, 异步块中使用, 任何可以用于.await的东西

<pattern> = <async expression> (, if <precondition>)? => <handler>,
else => <expression>

所有表达式都在同一个线程上运行，如果一个分支阻塞了线程，所有其他表达式将无法继续。 如果需要并行性，请使用 tokio::spawn 生成每个异步表达式并将连接句柄传递给 select!

如果所有分支都不满足,且没有else会panic

*/

async fn do_stuff_async() {
    println!("do_stuff_async");
}

async fn more_async_work() {
    println!("more_async_work");
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = more_async_work() => {
            println!("more_async_work() completed first")
        }
    };
}
