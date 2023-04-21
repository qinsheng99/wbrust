use clap::Parser;

use lazy_static::lazy_static;
#[allow(unused_imports)]
use local_config::{load_config, ConfigImpl, LocalConfig};
use log::info;
use std::sync::{Arc, RwLock};
use utils::error::err;

extern crate lazy_static;

mod common;
mod controller;
mod local_config;
mod server;
mod utils;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    config_file: Option<String>,

    #[arg(short, long, default_value_t = String::from("192.168.1.218"))]
    #[arg(help = "server address")]
    address: String,

    #[arg(short, default_value_t = 9000)]
    #[arg(help = "server port")]
    port: u16,
}

lazy_static! {
    pub static ref SERVERCONFIG: Arc<RwLock<LocalConfig>> = {
        let args = Args::parse();
        let path = Args.config_file.unwrap_or("local_config/config.toml");
        let server_config = util::config::ServerConfig::new(path);
        server_config.config
    };
}

async fn main() -> () {
    env_logger::init();

    let path = Args.config_file.unwrap_or("config.yaml");

    let _cfg = load_config(path.as_str());

    info!("start server {}:{}", args.address, args.port);

    ()
    // server::server((args.address, args.port), "v1")
}
