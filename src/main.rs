use std::net::TcpListener;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Waiting for a connection...");

    let (mut stream, peer_addr) = listener.accept()?;
    println!("Connection established with {peer_addr}");

    let mut buffer = [0u8; 1024];
    let mut buffer = [0u8; 5];
    stream.read_exact(&mut buffer)?;

    println!("Received bytes: {buffer:?}");

    Ok(())
}