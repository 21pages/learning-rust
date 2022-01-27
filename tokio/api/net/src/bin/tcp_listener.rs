use std::io::Result;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/*
read_to_string()需要EOF来结束, 所以用来读文件最好
*/

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234").await?;

    //addr
    assert_eq!(
        listener.local_addr()?,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234))
    );
    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            process(stream).await.unwrap();
        });
    }
}

async fn process(mut stream: TcpStream) -> Result<()> {
    println!("before read");
    let mut buf: [u8; 128] = [0; 128];
    let size = stream.read(&mut buf).await?;
    let mut v = vec![0; 128];
    v.copy_from_slice(&buf);
    v = v[..size].to_vec();
    println!("read {:?}", v);
    v.reverse();
    stream.write_all(&v[..]).await?;
    stream.flush().await?;
    println!("write finish");
    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(1));
    Ok(())
}
