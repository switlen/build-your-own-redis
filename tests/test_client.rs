use std::{io::{Result, Write}, net::TcpStream};



#[test]
fn connect_server() -> Result<()>{
    let mut stream: TcpStream = TcpStream::connect("localhost:6379")?;
    let buf = b"ping";
    stream.write_all(buf)?;
    Ok(())
}