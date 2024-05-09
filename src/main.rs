use std::error::Error;
use crate::http::server::Server;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// directory to read files from
    #[arg(long)]
    directory: Option<String>,
}

mod http;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    let directory = args.directory.take().unwrap_or_else(|| "/".to_string());


    let server = Server::init("127.0.0.1:4221", directory)?;
    server.run();
}
