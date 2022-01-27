use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:1234").await?;
    let kv = ("hello", "world");
    client.set(kv.0,  kv.1.into()).await?;
    let mut client = client::connect("127.0.0.1:1234").await?;
    let result = client.get(kv.0).await?.unwrap();
    println!("get val from {:?}", result);

    Ok(())
}


