use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use crate::http::header::RequestHeader;

pub struct Request {
    pub(crate) first_line: RequestHeader,
    pub(crate) fields: HashMap<String, String>,
}

impl Request {
    pub fn from_tcp_stream(raw_request: &TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(raw_request);

        let mut first_line = String::new();
        reader.read_line(&mut first_line).unwrap();

        let mut fields = HashMap::new();

        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();

            if line.trim().is_empty() {
                // body begins
                break;
            }

            if let Some((key, value)) = Self::parse_header_line(&line) {
                fields.insert(key, value);
            }
        }

        if let Some(content_length) = fields.get("Content-Length") {
            let content_length: usize = content_length.parse().unwrap();
            let mut body_bytes = vec![0; content_length];
            reader.read_exact(&mut body_bytes).unwrap();

            let body = String::from_utf8(body_bytes).unwrap();
            fields.insert("body".to_string(), body);
        }

        if let Some(header) = RequestHeader::from_string(first_line) {
            Ok(Self {
                first_line: header,
                fields,
            })
        } else {
            Err("Failed to parse request header".into())
        }
    }

    fn parse_header_line(line: &str) -> Option<(String, String)> {
        if let Some(index) = line.find(':') {
            let key = line[..index].trim().to_string();
            let value = line[index + 1..].trim().to_string();
            Some((key, value))
        } else {
            None
        }
    }
}
