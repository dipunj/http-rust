use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::http::header::RequestHeader;

pub struct Request {
    pub(crate) header: RequestHeader,
    host: String,
    user_agent: String,
}

impl Request {
    pub fn from_tcp_stream(raw_request: &TcpStream) -> Result<Self,  Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(raw_request);
        let mut data = String::new();

        reader.read_line(&mut data).unwrap();

        let parts: Vec<&str> = data.trim().split_whitespace().collect();
        let (host, user_agent) = Self::extract_host_and_user_agent(parts.get(1).unwrap_or(&""));

        if let Some(header) = RequestHeader::from_string(data) {
            return Ok(Self {
                header,
                host,
                user_agent
            })
        } else {
            Err("Failed to parse request header".into())
        }
    }

    fn extract_host_and_user_agent(path: &str) -> (String, String) {
        // Assuming the path contains host and user_agent separated by "/"
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 3 {
            (parts[1].to_string(), parts[2].to_string())
        } else {
            (String::new(), String::new())
        }
    }
}
