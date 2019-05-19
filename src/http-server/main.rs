use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) {
    println!("> {:?}", stream);
    //let mut buf = [0; 128];
    let mut buf = vec![0; 128];
    match stream.read(&mut buf) {
        Ok(c) => {
            println!("ok : {:?} {:?}", c, buf);
        }
        Err(e) => {
            println!("err: {:?}", e);
        }
    }
    stream.write(&buf).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
