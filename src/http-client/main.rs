use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "127.0.0.1:34254";
    let mut stream = TcpStream::connect(host)?;

    // Ex.) curl 127.0.0.1:34254 --http1.0
    //let cnt = format!("GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: curl/7.58.0\r\nAccept: */*\r\n\r\n", host);

    // Simple-Request
    let cnt = format!("GET http://{}/index.html\r\n", host);

    stream.write(cnt.as_bytes())?;
    let mut buf = vec![0; 128];
    match stream.read(&mut buf) {
        Ok(n) => {
            println!("> {} {}", n, String::from_utf8(buf[0..n].to_vec()).unwrap());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    Ok(())
}
