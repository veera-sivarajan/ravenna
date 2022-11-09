use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("example.org:80")?;
    let message = "GET /index.html HTTP/1.0\r\nHost: example.org\r\n\r\n";
    stream.write(message.as_bytes())?;
    let mut buffer = [0; 128];
    stream.read(&mut buffer)?;
    let output = std::str::from_utf8(&buffer).unwrap();
    println!("{:?}", output);
    Ok(())
}
