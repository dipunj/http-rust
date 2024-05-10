use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::http::request::Request;
use crate::http::response::Response;

// the server object which is exposed to the main function
pub struct Server {
    file_dir: String,
    listener: TcpListener,
    handler: fn(Request, &String) -> Response,
}

impl Server {
    pub fn init(addr: &str, dir: String, handler: fn(Request, &String) -> Response) -> Result<Self, std::io::Error> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            file_dir: dir,
            handler,
        })
    }

    pub fn run(&self) -> ! {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let dir = self.file_dir.clone();
                    let handler = self.handler;
                    thread::spawn(move || {
                        let req = Self::parse_request(&stream);
                        let response = handler(req, &dir);

                        stream.write_all(response.to_string().as_bytes()).unwrap();
                    });
                }
                Err(_) => {
                    // Handle error
                }
            }
        }
        unreachable!();
    }

    pub fn parse_request(stream: &TcpStream) -> Request {
        Request::from_tcp_stream(&stream).unwrap() // unwrap() here is safe because parsing always succeeds
    }
}