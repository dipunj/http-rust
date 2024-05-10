use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::http::server::Server;
use clap::Parser;
use crate::http::code::HttpCode;
use crate::http::media_type::MediaType;
use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::version::Version::Http1_1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// directory to read files from
    #[arg(long)]
    directory: Option<String>,
}

mod http;

// user application logic, which can be modified by the consumer of the http crate
fn handle_request(request: Request, dir: &String) -> Response  {
    return if request.first_line.path == "/" {
        Response::new(Http1_1, HttpCode::OK, MediaType::TextPlain, String::new())
    } else if let Some(content) = request.first_line.path.strip_prefix("/echo/") {
        Response::new(Http1_1, HttpCode::OK, MediaType::TextPlain, String::from(content))
    } else if request.first_line.path == "/user-agent" {
        Response::new(Http1_1, HttpCode::OK, MediaType::TextPlain, request.fields["User-Agent"].to_string())
    } else if let Some(filename) = request.first_line.path.strip_prefix("/files/") {
        let file_path = Path::new(dir).join(filename);
        if request.first_line.method == Method::GET {
            return if let Ok(contents) = fs::read_to_string(file_path) {
                Response::new(Http1_1, HttpCode::OK, MediaType::OctetStream, contents)
            } else {
                Response::new(Http1_1, HttpCode::NotFound, MediaType::TextPlain, String::new())
            }
        } else if request.first_line.method == Method::POST {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_path) {
                return if let Ok(_contents) = file.write_all(request.body.as_bytes()) {
                    Response::new(Http1_1, HttpCode::CREATED, MediaType::TextPlain, String::new())
                } else {
                    Response::new(Http1_1, HttpCode::NotFound, MediaType::TextPlain, String::new())
                }
            } else {
                Response::new(Http1_1, HttpCode::NotFound, MediaType::TextPlain, String::new())
            }
        } else {
            Response::new(Http1_1, HttpCode::NotFound, MediaType::TextPlain, String::new())
        }
    } else {
        Response::new(Http1_1, HttpCode::NotFound, MediaType::TextPlain, String::new())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    let directory = args.directory.take().unwrap_or_else(|| "/".to_string());

    let server = Server::init("127.0.0.1:4221", directory, handle_request)?;
    server.run();
}