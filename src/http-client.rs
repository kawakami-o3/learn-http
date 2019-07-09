use std::io::prelude::*;
use std::net::TcpStream;

const HOST: &str = "127.0.0.1:34254";
//const HOST: &str = "httpbin.org:80";

fn request_body() -> String {
    //format!("GET /get HTTP/1.0\r\n\r\n")

    // Simple-Request
    //format!("GET http://{}/\r\n", host);
    //format!("GET http://www.google.com/\r\n", host);
    //format!("GET http://www.google.com/\r\n");
    //format!("GET /index.html\r\n")
    //format!("GET /index.html HTTP/1.0\r\n\r\n");

    // HTTP/1.0
    //format!("GET / HTTP/1.0\r\nPragma: no-cache\r\n\r\n");
    //format!("GET /ip HTTP/1.0\r\nPragma: no-cache\r\n\r\n");
    //format!("GET /get HTTP/1.0\r\n\r\n");

    // HTTP/1.1
    //format!("GET /index.html HTTP/1.1\r\n\r\n");

    // httpbin.org
    let mut s = "GET /../Cargo.toml HTTP/1.0".to_string();
    s.push_str("\r\n");
//    s.push_str("Host: httpbin.org");
//    s.push_str("\r\n");
    s.push_str("Accept: application/json");
    s.push_str("\r\n");
//    s.push_str("User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:67.0) Gecko/20100101 Firefox/67.0");
//    s.push_str("\r\n");
//    //s.push_str("Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8");
//    //s.push_str("\r\n");
//    s.push_str("Accept-Language: ja,en-US;q=0.7,en;q=0.3");
//    s.push_str("\r\n");
//    s.push_str("Accept-Encoding: gzip, deflate");
//    s.push_str("\r\n");
//    s.push_str("DNT: 1");
//    s.push_str("\r\n");
//    s.push_str("Connection: keep-alive");
//    s.push_str("\r\n");
//    s.push_str("Upgrade-Insecure-Requests: 1");
//    s.push_str("\r\n");
//    s.push_str("Pragma: no-cache");
//    s.push_str("\r\n");
//    s.push_str("Cache-Control: no-cache");
//    s.push_str("\r\n");
    s.push_str("\r\n");
    return s;
}

fn main() -> std::io::Result<()> {
    //let host = "www.google.com:80";
    //let host = "httpbin.org:80";
    let mut stream = TcpStream::connect(HOST)?;

    // Ex.) curl 127.0.0.1:34254 --http1.0
    //let cnt = format!("GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: curl/7.58.0\r\nAccept: */*\r\n\r\n", host);


    print!("sending ... ");
    stream.write(request_body().as_bytes())?;
    println!("done.");

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
