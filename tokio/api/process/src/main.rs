use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("notepad").spawn().expect("failed to spawn");

    let status = child.wait().await?;
    println!("the command exited with: {}", status);
    Ok(())
}
