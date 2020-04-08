use std::env;
use std::io::{Error, ErrorKind};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = env::args().skip(1).next().ok_or(ErrorKind::InvalidData)?;
    let mut stream = TcpStream::connect(format!("{}:5858", address)).await?;

    loop {
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).await?;

        println!("Received {}", u32::from_be_bytes(buffer));
    }
}
