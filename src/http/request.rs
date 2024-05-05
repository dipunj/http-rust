use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::http::header::RequestHeader;

pub struct Request {
    pub(crate) header: RequestHeader,
    host: String,
    pub(crate) user_agent: String,
}

impl Request {
    pub fn from_tcp_stream(raw_request: &TcpStream) -> Result<Self,  Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(raw_request);

        let mut header_line = String::new();
        reader.read_line(&mut header_line).unwrap();

        let mut host_line = String::new();
        reader.read_line(&mut host_line).unwrap();
        let host = Self::extract_value(header_line.trim());

        let mut ua_line= String::new();
        reader.read_line(&mut ua_line).unwrap();
        let user_agent = Self::extract_value(ua_line.trim());


        println!("THe host and user agent are {}, {}", host, user_agent);
        if let Some(header) = RequestHeader::from_string(header_line) {
            return Ok(Self {
                header,
                host,
                user_agent
            })
        } else {
            Err("Failed to parse request header".into())
        }
    }

    fn extract_value(line: &str) -> String {
        if let Some(index) = line.find(':') {
            line[index + 1..].trim().to_string()
        } else {
            String::new()
        }

    }

}


