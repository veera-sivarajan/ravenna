use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{fmt, fmt::Display};

#[derive(Default, Debug)]
pub struct Response {
    status: String,
    header: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
pub enum RequestError {
    E404,
    E401,
    E400,
    Irregular(String),
    Unknown,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::E404 => write!(f, "404"),
            RequestError::E401 => write!(f, "401"),
            RequestError::E400 => write!(f, "400"),
            RequestError::Irregular(msg) => {
                write!(f, "Malformed HTTPS Response: `{msg}`")
            }
            _ => write!(f, "Unknown Error"),
        }
    }
}

impl std::error::Error for RequestError {}

impl From<Option<u16>> for RequestError {
    fn from(number: Option<u16>) -> Self {
        match number {
            Some(404) => RequestError::E404,
            Some(400) => RequestError::E400,
            Some(401) => RequestError::E401,
            Some(_) | None => RequestError::Unknown,
        }
    }
}

impl Response {
    fn parse_status(status: &str) -> Option<u16> {
        let mut status_split = status.split(" ");
        status_split.next();
        status_split.next().and_then(|s| s.parse().ok())
    }
    
    fn parse_response(
        contents: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let mut contents_split = contents.split("\r\n\r\n");
        let Some(meta) = contents_split.next() else {
            return Err(Box::new(RequestError::Irregular("Header not found.".into())));
        };
        let mut meta_split = meta.split("\r\n");
        let status = meta_split.next().unwrap();
        if status.contains("200 OK") {
            let mut header = HashMap::new();
            for line in meta_split {
                let mut key_value = line
                    .split(':')
                    .map(|word| word.trim().to_ascii_lowercase());
                let key = key_value.next().expect("Key not found.");
                let value = key_value.next().expect("Value not found.");
                header.insert(key, value);
            }
            let Some(body) = contents_split.next() else {
                return Err(Box::new(RequestError::Irregular("Body not found.".into())));
            };
            Ok(Response {
                status: status.to_string(),
                header,
                body: body.to_string(),
            })
        } else {
            let response_code = Response::parse_status(status);
            Err(Box::new(RequestError::from(response_code)))
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_angle = false;
        for c in self.body.chars() {
            if c == '<' {
                in_angle = true;
            } else if c == '>' {
                in_angle = false;
            } else if !in_angle {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

pub fn get(
    host: &'static str,
    path: &str,
    port: u16,
) -> Result<Response, Box<dyn std::error::Error>> {
    let connector = TlsConnector::new()?;
    let stream = TcpStream::connect(format!("{host}:{port}"))?;
    let mut stream = connector.connect(host, stream)?;
    let request = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\n\r\n");
    stream.write_all(request.as_bytes())?;
    let mut buffer = [0; 1024];
    let mut response = String::new();
    loop {
        match stream.read(&mut buffer)? {
            0 => break,
            size => response
                .push_str(std::str::from_utf8(&buffer[0..size]).unwrap()),
        }
    }
    Response::parse_response(&response)
}
