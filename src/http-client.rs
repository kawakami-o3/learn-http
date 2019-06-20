use std::io::prelude::*;
use std::net::TcpStream;

fn request_body() -> String {
    format!("GET /get HTTP/1.0\r\n\r\n")

    // Simple-Request
    //format!("GET http://{}/\r\n", host);
    //format!("GET http://www.google.com/\r\n", host);
    //format!("GET http://www.google.com/\r\n");
    //format!("GET /index.html\r\n");
    //format!("GET /index.html HTTP/1.0\r\n\r\n");

    // HTTP/1.0
    //format!("GET / HTTP/1.0\r\nPragma: no-cache\r\n\r\n");
    //format!("GET /ip HTTP/1.0\r\nPragma: no-cache\r\n\r\n");
    //format!("GET /get HTTP/1.0\r\n\r\n");

    // HTTP/1.0
    //format!("GET /index.html HTTP/1.1\r\n\r\n");
}

fn main() -> std::io::Result<()> {
    //let host = "127.0.0.1:34254";
    //let host = "www.google.com:80";
    let host = "httpbin.org:80";
    let mut stream = TcpStream::connect(host)?;

    // Ex.) curl 127.0.0.1:34254 --http1.0
    //let cnt = format!("GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: curl/7.58.0\r\nAccept: */*\r\n\r\n", host);


    stream.write(request_body().as_bytes())?;
    //let mut buf = vec![0; 1024*1024*1024];
    let mut buf = vec![0; 1024];
    let mut response = Vec::new();
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                //println!("> {} {}", n, String::from_utf8(buf[0..n].to_vec()).unwrap());
                //print!("{}", String::from_utf8(buf[0..n].to_vec()).unwrap());
                response.append(&mut buf[0..n].to_vec());
                if n == 0 {
                    break;
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    unsafe {
        println!("{}", String::from_utf8_unchecked(response));
    }
    //println!("{}", String::from_utf8(response).unwrap());
    Ok(())
}
