use std::io::Result;
use tokio::{fs::File, io::{BufReader, BufWriter, AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() -> Result<()>{
    let path = "tmp.txt";
    let f = File::create(path).await?;
    let content = "今天天气不错";
    //写
    let mut writer = BufWriter::new(f);
    writer.write(content.as_bytes()).await?;
    writer.flush().await?; //必须flush

    //读
    let f2 = File::open(path).await?;
    let mut reader = BufReader::new(f2);
    let mut read_string = String::new();
    reader.read_to_string(&mut read_string).await?;
    println!("equal?:{:?}", content.eq(read_string.as_str()));

    Ok(())
}