use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("example.org:80")?;
    let message = "GET /index.html HTTP/1.0\r\nHost: example.org\r\n\r\n";
    stream.write(message.as_bytes())?;
    let mut buffer = [0; 1024];
    loop {
        if let Ok(size) = stream.read(&mut buffer) {
            if size == 0 {
                break;
            } else {
                let output = std::str::from_utf8(&buffer[0..size]).unwrap();
                print!("{}", output);
            }
        } else {
            eprintln!("Error reading from server.");
        }
    }
    Ok(())
}
