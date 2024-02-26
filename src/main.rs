
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
use anyhow::Result;
use std::str;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("redis server begin running!");

    // bind a port
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("redis server listen on port 6379");
    
    // receive tcp connection
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // multi thread process connection
                thread::spawn(move || handle_connection(&mut stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

/**
 *  handle connection 
 *  stream tcp connection
 */
fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    // const pong
    const PONG: &[u8] = "+PONG\r\n".as_bytes();
    let mut buf = [0; 512];
    // read client msg and write pong
    while let Ok(bytes) = stream.read(&mut buf) {
        println!("Received {bytes} {:?}", str::from_utf8(&buf[0..bytes]).unwrap());
        stream.write_all(PONG)?;
    }
    Ok(())
}
