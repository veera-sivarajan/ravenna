use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

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

fn parse_webpage(contents: &str) -> Result<Response, String> {
    let mut contents_split = contents.split("\r\n\r\n");
    let Some(meta) = contents_split.next() else {
        return Err(String::from("Header not found."));
    };
    let Some(body) = contents_split.next() else {
        return Err(String::from("Body not found."));
    };
    let mut meta_split = meta.split("\r\n");
    let status = meta_split.next().unwrap();
    let mut header = HashMap::new();
    for line in meta_split {
        let mut key_value = line.split(':');
        let key = key_value.next().unwrap().to_ascii_lowercase();
        let value = key_value.next().unwrap().trim().to_ascii_lowercase();
        header.insert(key, value);
    }
    Ok(Response {
        status: status.to_string(),
        header,
        body: body.to_string(),
    })
}

fn main() -> std::io::Result<()> {
    let data = get_webpage("example.org", "/index.html", 80)?;
    // print!("{:?}", data);
    println!("{:#?}", parse_webpage(&data));
    Ok(())
}
