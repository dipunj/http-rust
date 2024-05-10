use crate::http::method::Method;
use crate::http::code::HttpCode;
use crate::http::version::Version;

pub struct RequestLine {
    pub(crate) method: Method,
    pub(crate) path: String,
    protocol: String,
}


impl RequestLine {
    pub fn from_string(raw_header: String) -> Option<Self> {
        let mut segments = raw_header.split(" ");

        let Some(method_str) = segments.next() else {
            println!("Invalid HTTP header format: Invalid Method string");
            return None;
        };

        let Some(method) = Method::from_string(method_str) else {
            println!("Invalid HTTP header format: Unsupported HTTP method {}", method_str);
            return None;
        };

        let Some(path) = segments.next() else {
            println!("Invalid HTTP header format: path");
            return None;
        };

        let Some(protocol) = segments.next() else {
            println!("Invalid HTTP header format: protocol");
            return None;
        };

        return Some(Self {
            method,
            path: path.to_string(),
            protocol: protocol.to_string()
        })
    }
}

pub struct ResponseHeader {
    protocol: Version,
    http_code: HttpCode
}

impl ResponseHeader {
    pub fn to_string(&self) -> String {
        String::from(self.protocol.as_str()) + " " + &self.http_code.to_string()
    }
    
    pub fn new(protocol: Version, http_code: HttpCode) -> Self {
        Self {
            protocol,
            http_code,
        }
    }
}