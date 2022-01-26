async fn func_err() -> std::result::Result<(), String> {
    println!("func_err");
    Err("hello".into())
}

async fn func_sleep() -> std::result::Result<(), String>{
    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(1));
    println!("func_sleep");
    Ok(())
}

async fn async_main() {
    if let Err(s) = futures::try_join!(func_err(), func_sleep()) {
        println!("get error:{}", s);
    }
}
/*
func_err
get error:hello
*/

#[test]
fn test() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}