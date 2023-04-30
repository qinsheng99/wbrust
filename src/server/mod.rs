use crate::{
    app::repo_info::RepoService,
    common::infrastructure::postgresql::get_db,
    controller as ctl,
    controller::repo_info::{RepoController, RepoCtl},
    infrastructure::repositoryimpl::repo_info::RepoInfoImpl,
    utils::error::Result,
};

use actix_web::{web, App, HttpServer};
use config::Config;
use std::sync::{Arc, RwLock};

pub struct Server {
    config: Arc<RwLock<Config>>,
    repo_ctl: Arc<dyn RepoCtl>,
}

impl Server {
    pub async fn new(v: Arc<RwLock<Config>>) -> Result<Server> {
        let config = v.clone();

        let repo_ctl = Arc::new(RepoController::new(Box::new(RepoService::new(Box::new(
            RepoInfoImpl::new(get_db()?.clone(), String::from("repo_info")),
        ))))) as Arc<dyn RepoCtl>;

        let c = Server { config, repo_ctl };

        Ok(c)
    }

    pub async fn run(&self, path: &'static str) -> Result<()> {
        let address: String = format!(
            "{}:{}",
            self.config.read()?.get_string("address")?,
            self.config.read()?.get_string("port")?
        );

        let repo_ctl_service = web::Data::from(self.repo_ctl.clone());

        let http_server = HttpServer::new(move || {
            App::new()
                .app_data(repo_ctl_service.clone())
                .service(ctl::get_scope(path))
        });

        http_server.bind(address)?.run().await?;

        Ok(())
    }
}
