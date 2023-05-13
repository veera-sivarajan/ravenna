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
    pub status: StatusCode,
    pub header: HashMap<String, String>, // TODO: Make this more strongly typed. Like: ContenType(String), Length(usize), Other
    pub body: String,
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

fn parse_status(status: Option<&str>) -> StatusCode {
    if let Some(status) = status {
        let mut status_split = status.split(' ');
        status_split.next();
        let status_number: Option<u16> = status_split.next().and_then(|s| s.parse().ok());
        StatusCode::from(status_number)
    } else {
        StatusCode::Unknown
    }
}

fn parse_header(header: &str) -> Result<HashMap<String, String>, RequestError> {
    let mut header = header.split("\r\n");
    let status = parse_status(header.next());
    if let StatusCode::Successful = status {
        let mut map = HashMap::new();
        for line in header {
            if let Some((key, value)) = line.split_once(':') {
                map.insert(key.to_string(), value.trim().to_string());
            }
        }
        Ok(map)
    } else {
        Err(RequestError::Status(status))
    }
}

fn parse_response(contents: &str) -> Result<Response, RequestError> {
    if let Some((header, body)) = contents.split_once("\r\n\r\n") {
        Ok(Response {
            status: StatusCode::Successful,
            header: parse_header(header)?,
            body: body.into(),
        })
    } else {
        Err(RequestError::Irregular("Body not found.".into()))
    }
}

pub struct DisplayableResponse(pub Result<Response, Box<dyn std::error::Error>>);

impl Display for DisplayableResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(response) => {
                let mut in_angle = false;
                for c in response.body.chars() {
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
            Err(error) => {
                write!(f, "{error}")?;
                Ok(())
            }
        }
    }
}

pub fn get(host: &str, path: &str, port: u16) -> DisplayableResponse {
    let response = send_request(host, path, port);
    DisplayableResponse(response)
}

fn send_request(host: &str, path: &str, port: u16) -> Result<Response, Box<dyn std::error::Error>> {
    let response = get_helper(host, path, port)?;
    parse_response(&response).map_err(|e| e.into())
}

fn get_helper(host: &str, path: &str, port: u16) -> Result<String, Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(format!("{host}:{port}"))?;
    let mut stream = TlsConnector::new()?.connect(host, stream)?;
    let request = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\n\r\n");
    stream.write_all(request.as_bytes())?;
    let mut buffer = [0; 1024];
    let mut response = String::new();
    loop {
        match stream.read(&mut buffer)? {
            0 => break,
            size => response.push_str(std::str::from_utf8(&buffer[0..size]).unwrap()),
        }
    }
    Ok(response)
}
