use std::io::prelude::*;
use std::net::TcpStream;

pub fn run() -> std::io::Result<()> {
    let addr: String = "127.0.0.1:5000".to_string();
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&[1])?;
    Ok(())
}