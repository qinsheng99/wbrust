use crate::controller;
use crate::utils::error::Result;
use actix_web::{App, HttpServer};
use config::Config;
use std::sync::{Arc, RwLock};

pub struct Server {
    config: Arc<RwLock<Config>>,
}

impl Server {
    pub async fn new(v: Arc<RwLock<Config>>) -> Result<Server> {
        let cfg = v.clone();

        let c = Server { config: cfg };

        Ok(c)
    }

    pub async fn run(&self, path: &'static str) -> Result<()> {
        let address: String = format!(
            "{}:{}",
            self.config.read()?.get_string("address")?,
            self.config.read()?.get_string("port")?
        );

        let http_server = HttpServer::new(|| App::new().service(controller::get_scope(path)));

        http_server.bind(address)?.run().await?;

        Ok(())
    }
}
