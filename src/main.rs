use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::{fmt, fmt::Display};

fn get_webpage(host: &str, path: &str, port: u16) -> std::io::Result<String> {
    let host_and_port = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(host_and_port)?;
    let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;
    let mut buffer = [0; 1024];
    let mut response = String::new();
    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size == 0 => break,
            Ok(size) => response.push_str(std::str::from_utf8(&buffer[0..size]).unwrap()),
            Err(_) => eprintln!("Error reading from server."),
        }
    }
    Ok(response)
}

#[derive(Default, Debug)]
struct Response {
    status: String,
    header: HashMap<String, String>,
    body: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        let mut in_angle = false;
        for c in self.body.chars() {
            if c == '<' {
                in_angle = true;
            } else if c == '>' {
                in_angle = false;
            } else if !in_angle {
                buffer.push(c);
            }
        }
        write!(f, "{}", buffer)
    }
}

fn parse_webpage(contents: &str) -> Result<Response, String> {
    let mut contents_split = contents.split("\r\n\r\n");
    let Some(meta) = contents_split.next() else {
        return Err(String::from("Header not found."));
    };
    let mut meta_split = meta.split("\r\n");
    let status = meta_split.next().unwrap();
    if status.contains("200 OK") {
        let mut header = HashMap::new();
        for line in meta_split {
            let mut key_value = line.split(':').map(|word| word.trim().to_ascii_lowercase());
            let key = key_value.next().unwrap();
            let value = key_value.next().unwrap();
            header.insert(key, value);
        }
        let Some(body) = contents_split.next() else {
            return Err(String::from("Body not found."));
        };
        Ok(Response {
            status: status.to_string(),
            header,
            body: body.to_string(),
        })
    } else {
        Err(String::from("Error fetching webpage."))
    }
}

fn main() -> std::io::Result<()> {
    let data = get_webpage("example.org", "/index.html", 80)?;
    // print!("{:?}", data);
    println!("{}", parse_webpage(&data).unwrap());
    Ok(())
}
