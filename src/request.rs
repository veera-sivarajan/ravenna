use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{fmt, fmt::Display};

#[derive(Default, Debug)]
pub enum StatusCode {
    Informational,
    Successful,
    Redirection,
    ClientError,
    ServerError,
    #[default]
    Unknown,
}

#[derive(Default, Debug)]
pub struct Response {
    status: StatusCode,
    header: HashMap<String, String>, // TODO: Make this more strongly typed. Like: ContenType(String), Length(usize), Other
    body: String,
}

#[derive(Debug)]
pub enum RequestError {
    Status(StatusCode),
    Irregular(String),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::Status(code) => {
                write!(f, "Error code: {code:?}")
            }
            RequestError::Irregular(msg) => {
                write!(f, "Malformed HTTPS Response: `{msg}`")
            }
        }
    }
}

impl std::error::Error for RequestError {}

impl From<Option<u16>> for StatusCode {
    fn from(status: Option<u16>) -> Self {
        match status {
            Some(100..=199) => StatusCode::Informational,
            Some(200..=299) => StatusCode::Successful,
            Some(300..=399) => StatusCode::Redirection,
            Some(400..=499) => StatusCode::ClientError,
            Some(500..=599) => StatusCode::ServerError,
            _ => StatusCode::Unknown,
        }
    }
}

impl Response {
    fn parse_status(status: Option<&str>) -> StatusCode {
        if let Some(status) = status {
            let mut status_split = status.split(" ");
            status_split.next();
            let status_number: Option<u16> =
                status_split.next().and_then(|s| s.parse().ok());
            StatusCode::from(status_number)
        } else {
            StatusCode::Unknown
        }
    }

    fn parse_header(
        header: Option<&str>,
    ) -> Result<HashMap<String, String>, RequestError> {
        if let Some(header) = header {
            let mut header = header.split("\r\n");
            let status = Response::parse_status(header.next());
            if let StatusCode::Successful = status {
                let mut map = HashMap::new();
                for line in header {
                    let mut key_value = line
                        .split(':')
                        .map(|word| word.trim().to_ascii_lowercase());
                    let key = key_value.next().expect("Key not found.");
                    let value =
                        key_value.next().expect("Value not found.");
                    map.insert(key, value);
                }
                Ok(map)
            } else {
                Err(RequestError::Status(status))
            }
        } else {
            Err(RequestError::Irregular(
                "Header not found.".into(),
            ))
        }
    }

    fn parse_response(
        contents: &str,
    ) -> Result<Response, RequestError> {
        let mut contents = contents.split("\r\n\r\n");
        let header = contents.next();
        let body = contents.last();

        let header = Response::parse_header(header)?;

        if let Some(body) = body {
            Ok(Response {
                status: StatusCode::Successful,
                header,
                body: body.into(),
            })
        } else {
            Err(RequestError::Irregular("Body not found.".into()))
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
    let response = get_helper(host, path, port)?;
    let parsed_response= Response::parse_response(&response)?;
    Ok(parsed_response)
}

fn get_helper(host: &'static str, path: &str, port: u16) -> Result<String, Box<dyn std::error::Error> {
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
    Ok(response)
}

