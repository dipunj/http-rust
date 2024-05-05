
mod http;

use std::error::Error;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use crate::http::code::HttpCode;
use crate::http::content_type::ContentType;

use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::version::Version::Http1_1;

fn handle_request(req: &TcpStream) -> Result<String,  Box<dyn std::error::Error>>  {
    match Request::from_tcp_stream(&req) {
        Ok(request) => {
            return if request.header.path == "/" {
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, String::new());
                Ok(res.to_string())
            } else if let Some(content) = request.header.path.strip_prefix("/echo/") {
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, String::from(content));
                Ok(res.to_string())
            } else if request.header.path == "/user-agent" {
                println!("HEREEEE {}", request.user_agent.to_string());
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, request.user_agent.to_string());
                Ok(res.to_string())
            } else {
                let res = Response::new(Http1_1, HttpCode::NotFound, ContentType::TextPlain, String::new());
                Ok(res.to_string())
            }
        }
        Err(err) => {
            // Handle the error
            println!("Error: {}", err);
            Err(err)
        }
    }

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                match handle_request(&s) {
                    Ok(response) => {
                        s.write_all(response.as_bytes()).unwrap();
                    }

                    _ => {}
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
