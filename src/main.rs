use clap::Parser;

use utils::error::err;

mod common;
mod config;
mod controller;
mod server;
mod utils;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config_file: Option<String>,

    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    #[arg(help = "server address")]
    address: String,

    #[arg(short, default_value_t = 8080)]
    #[arg(help = "server port")]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let (_file_name, _): (String, ()) = match args.config_file {
        Some(p) => (p, ()),
        None => ("".to_string(), err("empty config file")),
    };

    server::server((args.address, args.port), "v1")
}
