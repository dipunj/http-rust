
mod http;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn read_contents(req: &TcpStream) -> String {
    let mut reader = BufReader::new(req);
    let mut data = String::new();

    reader.read_line(&mut data).unwrap();
    return data;
}

fn parse_contents(contents: &str) -> (String, String, String) {
    let mut parts = contents.split("\r\n").next().unwrap_or("").splitn(3, " ");

    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    let http_version = parts.next().unwrap_or("").to_string();

    (method, path, http_version)
}


fn handle_request(req: TcpStream) {
    let data = read_contents(&req);
    let (_method, path, _http_version) = parse_contents(&data);

    return if path.starts_with("/echo/") {
        ok_response(req)
    } else {
        not_found_response(req)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_request(s);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
