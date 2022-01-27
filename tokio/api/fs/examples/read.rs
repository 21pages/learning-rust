use tokio::fs;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let path = "tmp.txt";
    let write_content = "127.0.0.1:1234";
    fs::write(path, write_content).await?;
    let read_content = fs::read_to_string(path).await?;
    if read_content == write_content {
        println!("equal");
    } else {
        eprintln!("not equal");
    }
    Ok(())
}