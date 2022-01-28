use std::error::Error;
use tokio::sync::broadcast;

/*
发送不用等到接受, 可以一直发
超过容量后, 只需要判断一次错误, 但是那次判断会将可能的非溢出数据丢弃
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let (tx, mut rx) = broadcast::channel(2);
    tx.send(1)?;
    tx.send(2)?;
    println!("before overflow");
    tx.send(3)?;
    println!("after overflow");
    tx.send(4)?;
    tx.send(5)?;

    let result = rx.recv().await;
    if result.is_ok() {
        println!("first value:{}", result?);
    } else {
        println!("Lagged");//Lagged
    }

    let v1 = rx.recv().await?;
    println!("v1:{}", v1);//4
    let v2 = rx.recv().await?;
    println!("v2:{}", v2);//5

    Ok(())

}