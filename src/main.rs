use std::io::Write;
use std::net::TcpListener;

fn ok_response(mut stream: std::net::TcpStream) {
    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                ok_response(s);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
