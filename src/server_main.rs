use clap::Parser;

use crate::common::infrastructure::postgresql::init_db;
use config::Config;
use lazy_static::lazy_static;
use local_config::LocalConfig;
use log::info;
use server::Server;
use std::sync::{Arc, RwLock};
use tokio;
use utils::error::Result;

extern crate lazy_static;

mod app;
mod common;
mod controller;
mod domain;
mod infrastructure;
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
    pub static ref SERVERCONFIG: Arc<RwLock<Config>> = {
        let args = Args::parse();
        let path = args
            .config_file
            .unwrap_or(String::from("/root/project/wbrust/src/config/config.toml"));
        let server_config = LocalConfig::new(&path);
        server_config.config
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    init_db(SERVERCONFIG.clone()).await?;

    info!("start wb server");
    let ser = Server::new(SERVERCONFIG.clone()).await?;
    ser.run("v1").await?;

    Ok(())
}
