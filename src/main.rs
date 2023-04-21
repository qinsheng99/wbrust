use clap::Parser;

use config::{load_config, ConfigImpl};
use log::info;
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

    #[arg(short, long, default_value_t = String::from("192.168.1.218"))]
    #[arg(help = "server address")]
    address: String,

    #[arg(short, default_value_t = 9000)]
    #[arg(help = "server port")]
    port: u16,
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let (file_name, _): (String, ()) = match args.config_file {
        Some(p) => (p, ()),
        None => ("".to_string(), err("empty config file")),
    };

    let _cfg = load_config(file_name.as_str()).unwrap();

    info!("start server {}:{}", args.address, args.port);

    server::server((args.address, args.port), "v1")
}
