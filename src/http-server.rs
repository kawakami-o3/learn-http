mod http_request;

use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
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

    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                //println!("ok : {}\n{}", n, String::from_utf8(buf[0..n].to_vec()).unwrap());
                //stream.write(&buf[0..n]).unwrap();
                println!("> {}", n);
                request.parse(&mut buf[0..n].to_vec());
                if request.is_terminated() {
                    break;
                }
                /*
                if n < 2 {
                    break;
                } else if is_terminated(&requestBytes) {
                    break;
                }
                */
            }
            Err(e) => {
                println!("ERR: {:?}", e);
                return;
                //stream.write("error\r\n".as_bytes()).unwrap();
            }
        }
    }

    let response = create_response(request);
    println!("{:?}", String::from_utf8(response.clone()));
    stream.write(response.as_slice()).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(HOST)?;

    println!("starting...");
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
