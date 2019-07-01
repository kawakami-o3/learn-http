mod http_request;

use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
//use crate::http_request;
//use self::http_request;


const HOST: & str = "127.0.0.1:34254";

//const CR: u8 = 13;
//const LF: u8 = 10;

fn create_response(request: http_request::Request) -> Vec<u8> {
    let res = request.bytes();
    //println!("{:?}", http_request::parse(request));
    //println!("{:?}", parse(request));
    return res;
}

//fn is_terminated(request: &Vec<u8>) -> bool {
//    let len = request.len();
//
//    // too short
//    if len < 2 {
//        return false;
//    }
//
//    // for simple request.
//    return request[len-2..len] == [CR, LF];
//}

fn handle_request(mut stream: TcpStream) {
    //let mut buf = [0; 128];
    let mut buf = vec![0; 1024];

    let mut request = http_request::new();

    match stream.read(&mut buf) {
        Ok(n) => {
            if n == 0 {
                println!("shutdown");
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }

            println!("> {} {:?}", n, buf[0..n].to_vec());
            match request.parse(&mut buf[0..n].to_vec()) {
                Ok(()) => {},
                Err(e) => {
                    println!("{}", e);
                    stream.shutdown(Shutdown::Both).unwrap();
                    return;
                }
            }
        }
        Err(e) => {
            println!("ERR: {:?}", e);
            return;
        }
    }

    let response = create_response(request);
    println!("{:?}", String::from_utf8(response.clone()));
    stream.write(response.as_slice()).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(HOST)?;

    println!("starting ... {}", HOST);
    /*
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_request(&mut stream);
            }
            Err(e) => {
                println!("ERR: {:?}", e);
            }
        }
    }
    Ok(())
    */

    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handle_request(stream);
        });
    }
}
