use crate::http::method::Method;
use crate::http::code::HttpCode;

pub struct RequestHeader {
    method: Method,
    path: String,
    protocol: String,
}


impl RequestHeader {
    fn from_string(raw_header: String) -> Option<Self> {
        /**
        * Converts raw_header to a object of type Header
        **/

        let mut header_data = raw_header.split(" ");

        let Some(method_str) = header_data.next() else {
            println!("Invalid HTTP header format: method_str");
            return None;
        };

        let Some(method) = Method::from_string(method_str) else {
            println!("Invalid HTTP header format: method");
            return None;
        };

        let Some(path) = header_data.next() else {
            println!("Invalid HTTP header format: path");
            return None;
        };

        let Some(protocol) = header_data.next() else {
            println!("Invalid HTTP header format: protocol");
            return None;
        };

        return Some(Self {
            method,
            path: path.to_string(),
            protocol: protocol.to_string()
        })
    }
    fn to_string(self) {
        todo!()
    }
}

pub struct ResponseHeader {
    protocol: String,
    http_code: HttpCode
}

impl ResponseHeader {
    fn to_string(&self) -> String {
        self.protocol.to_string() + &self.http_code.to_string()
    }
}