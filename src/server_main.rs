use clap::Parser;

use crate::common::infrastructure::{postgresql::init_db, redis::init_redis,mysql::init_mysql_db};
use cfg::LocalConfig;
use config::Config;
use lazy_static::lazy_static;
use log::info;
use server::Server;
use std::sync::{Arc, RwLock};
use tokio;
use utils::error::Result;

extern crate lazy_static;

mod app;
mod cfg;
mod common;
mod controller;
mod domain;
mod infrastructure;
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

const CONFIG_PATH: &'static str = "/src/config/config.toml";

lazy_static! {
    pub static ref SERVERCONFIG: Arc<RwLock<Config>> = {
        let p = format!(
            "{}{}",
            std::env::current_dir()
                .unwrap()
                .as_path()
                .as_os_str()
                .to_str()
                .expect("failed"),
            CONFIG_PATH
        );
        let args = Args::parse();
        let path = args.config_file.unwrap_or(String::from(p));
        let server_config = LocalConfig::new(&path);
        server_config.config
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    init_db(SERVERCONFIG.clone()).await?;

    init_mysql_db(SERVERCONFIG.clone()).await?;

    init_redis(SERVERCONFIG.clone()).await?;

    info!("start wb server");
    let ser = Server::new(SERVERCONFIG.clone()).await?;
    ser.run("v1").await?;

    Ok(())
}
