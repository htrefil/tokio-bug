use std::io::Error;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut listener = TcpListener::bind("0.0.0.0:5858").await?;
    loop {
        let (mut stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            println!("{}: connected", addr);
            async move {
                let mut i = 0u32;
                loop {
                    if let Err(err) = stream.write_all(&i.to_be_bytes()).await {
                        println!("{}: error sending: {}", addr, err);
                        return;
                    }

                    time::delay_for(Duration::from_secs(5)).await;
                    i = i.wrapping_add(1);
                }
            }
            .await;
            println!("{}: disconnected", addr);
        });
    }
}
