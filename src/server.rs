use std::net::TcpListener;
use valterm_chess::*;
use std::io::prelude::*;



pub fn run() -> std::io::Result<()>{
    let addr: String = "127.0.0.1:5000".to_string();
    let listener = TcpListener::bind(addr)?;

    let (mut stream, _addr) = listener.accept()?;

    let mut buf = [0u8; 1];
    stream.read(&mut buf)?;

    println!("Recieved: {}", buf[0]);
    Ok(())
}
