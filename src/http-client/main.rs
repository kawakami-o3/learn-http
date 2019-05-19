use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    stream.write(&[10])?;
    let mut buf = vec![0; 128];
    stream.read(&mut buf)?;
    println!("> {:?}", buf);
    Ok(())
}
