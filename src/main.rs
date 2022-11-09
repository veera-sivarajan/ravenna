use std::io::prelude::*;
use std::net::TcpStream;

fn get_webpage(host: &str, path: &str, port: u16) -> std::io::Result<String> {
    let host_and_port = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(host_and_port)?;
    let message = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, host);
    stream.write_all(message.as_bytes())?;
    let mut buffer = [0; 1024];
    let mut output = String::new();
    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size == 0 => break,
            Ok(size) => output.push_str(std::str::from_utf8(&buffer[0..size]).unwrap()),
            Err(_) => eprintln!("Error reading from server."),
        }
    }
    Ok(output)
}

fn main() -> std::io::Result<()> {
    let data = get_webpage("example.org", "/index.html", 80)?;
    print!("{}", data);
    Ok(())
}
