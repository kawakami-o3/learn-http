use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const BUF_SIZE: usize = 1024;

const HOST: & str = "127.0.0.1:34254";

fn handle_client(stream: &mut TcpStream) {
    //let mut buf = [0; 128];
    let mut buf = vec![0; BUF_SIZE];
    match stream.read(&mut buf) {
        Ok(n) => {
            println!("ok : {}\n{}", n, String::from_utf8(buf[0..n].to_vec()).unwrap());
            stream.write(&buf[0..n]).unwrap();
        }
        Err(e) => {
            println!("err: {:?}", e);
            stream.write("error\r\n".as_bytes()).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(HOST)?;

    println!("starting...");
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
