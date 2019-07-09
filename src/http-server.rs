
mod conf;
mod method;
mod http_request;
mod http_response;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

use crate::http_request::*;
use crate::http_response::*;
//use crate::conf;

// TODO cli option
const CONF_PATH: & str = "server_conf.json";

fn handle(request: &Request, response: &mut Response) -> Result<(), String> {
    response.version = request.version.clone();
    response.set_host(format!("{}:{}", conf::ip(), conf::port()));
    response.set_server(conf::server());

    match request.uri.as_str() {
        "/debug" => {
            match String::from_utf8(request.bytes()) {
                Ok(s) => {
                    // echo response
                    response.entity_body.push_str(s.as_str());
                }
                Err(_) => {
                    response.status = status::INTERNAL_SERVER_ERROR;
                }
            }
        }
        request_uri => {
            // TODO fix directory traversal
            //
            let uri = match util::canonicalize(request_uri) {
                Some(s) => s,
                None => {
                    //println!("debug(403): {}", format!("{}{}", conf::root(), request_uri));
                    response.status = status::FORBIDDEN;
                    return Ok(());
                }
            };
            let access_path = format!("{}{}", conf::root(), uri);
            match File::open(access_path) {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    match file.read_to_string(&mut buffer) {
                        Ok(_) => { }
                        Err(_) => {
                            //println!("debug(403): {}", format!("{}{}", conf::root(), uri));
                            response.status = status::FORBIDDEN;
                            return Ok(());
                        }
                    }
                    response.entity_body.push_str(buffer.as_str());
                }
                Err(_) => {
                    //println!("debug(404): {}", format!("{}{}", conf::root(), uri));
                    response.status = status::NOT_FOUND;
                    return Ok(());
                }
            }

        }
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];

    let mut request = http_request::new();

    match stream.read(&mut buf) {
        Ok(n) => {
            if n == 0 {
                println!("shutdown");
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }

            match request.parse(&mut buf[0..n].to_vec()) {
                Ok(()) => {},
                Err(e) => {
                    println!("{}", e); // TODO error response
                    stream.shutdown(Shutdown::Both).unwrap();
                    return;
                }
            }
        }
        Err(e) => {
            println!("ERR: {:?}", e); // TODO error response
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
    }

    let response = &mut http_response::new();
    match handle(&request, response) {
        Ok(()) => {
            println!("{:?}", String::from_utf8(response.to_bytes()));
            stream.write(response.to_bytes().as_slice()).unwrap();
        }
        Err(e) => {
            println!("ERR: {:?}", e); // TODO error response
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
    }
}

fn main() -> std::io::Result<()> {

    // read a configuration file.
    let server_conf = conf::load(CONF_PATH);
    conf::set(server_conf.clone());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", conf::port()))?;

    match listener.local_addr() {
        Ok(addr) => {
            println!("starting ... {}:{}", addr.ip(), addr.port());
        }
        Err(e) => {
            panic!("Error(local_addr): {}", e);
        }
    }

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

        let proc = |cnf, stream| {
            return || {
                conf::set(cnf);
                handle_request(stream);
            };
        };
        thread::spawn(proc(server_conf.clone(), stream));
    }
}
