use crate::{common::controller::middleware::Auth, controller as ctl, utils::error::Result};

use actix_web::{middleware, App, HttpServer};
use config::Config;
use std::sync::{Arc, RwLock};

pub struct Server {
    config: Arc<RwLock<Config>>,
}

impl Server {
    pub async fn new(v: Arc<RwLock<Config>>) -> Result<Server> {
        let config = v.clone();

        let c = Server { config };

        Ok(c)
    }

    pub async fn run(&self, path: &'static str) -> Result<()> {
        let address: String = format!(
            "{}:{}",
            self.config.read()?.get_string("address")?,
            self.config.read()?.get_string("port")?
        );

        let http_server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(Auth)
                .service(ctl::get_scope(path))
        });

        http_server.bind(address)?.run().await?;

        Ok(())
    }
}
