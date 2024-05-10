use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};
use std::cmp::PartialEq;
use std::path::Path;
use crate::http::code::HttpCode;
use crate::http::content_type::ContentType;
use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::version::Version::Http1_1;

pub struct Server {
    addr: String,
    file_dir: String,
    listener: TcpListener
}

fn handle_request(req: &TcpStream, dir: &String) -> Result<String,  Box<dyn std::error::Error>>  {
    match Request::from_tcp_stream(&req) {
        Ok(request) => {
            return if request.first_line.path == "/" {
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, String::new());
                Ok(res.to_string())
            } else if let Some(content) = request.first_line.path.strip_prefix("/echo/") {
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, String::from(content));
                Ok(res.to_string())
            } else if request.first_line.path == "/user-agent" {
                let res = Response::new(Http1_1, HttpCode::OK, ContentType::TextPlain, request.fields["User-Agent"].to_string());
                Ok(res.to_string())
            } else if let Some(filename) = request.first_line.path.strip_prefix("/files/") {
                let file_path = Path::new(dir).join(filename);
                if request.first_line.method == Method::GET {
                    return if let Ok(contents) = fs::read_to_string(file_path) {
                        let res = Response::new(Http1_1, HttpCode::OK, ContentType::OctetStream, contents);
                        Ok(res.to_string())
                    } else {
                        let res = Response::new(Http1_1, HttpCode::NotFound, ContentType::TextPlain, String::new());
                        Ok(res.to_string())
                    }
                } else if request.first_line.method == Method::POST {
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(file_path)?;
                    return if let Ok(_contents) = file.write_all(request.fields["body"].as_bytes()) {
                        let res = Response::new(Http1_1, HttpCode::CREATED, ContentType::TextPlain, String::new());
                        Ok(res.to_string())
                    } else {
                        let res = Response::new(Http1_1, HttpCode::NotFound, ContentType::TextPlain, String::new());
                        Ok(res.to_string())
                    }
                } else {
                    let res = Response::new(Http1_1, HttpCode::NotFound, ContentType::TextPlain, String::new());
                    Ok(res.to_string())
                }
            } else {
                let res = Response::new(Http1_1, HttpCode::NotFound, ContentType::TextPlain, String::new());
                Ok(res.to_string())
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(err)
        }
    }
}

impl Server {
    pub fn init(addr: &str, dir: String) -> Result<Self, std::io::Error> {
        Ok(Self {
            addr: addr.to_string(),
            listener: TcpListener::bind(addr)?,
            file_dir: dir,
        })
    }

    pub fn run(&self) -> ! {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let dir = self.file_dir.clone();
                    thread::spawn(move || {
                        match handle_request(&stream, &dir) {
                            Ok(response) => {
                                stream.write_all(response.as_bytes()).unwrap();
                            }
                            _ => {}
                        }
                    });
                }
                Err(_) => {
                    // Handle error
                }
            }
        }
        unreachable!();
    }

}
