use crate::{
    app::{
        dto::RepoInfoDTO,
        repo_info::{RepoService, RepoServiceImpl},
    },
    common::{
        controller::{Response, ResponseT},
        infrastructure::postgresql::get_db,
    },
    infrastructure::repositoryimpl::repo_info::RepoInfoImpl,
    utils::error::Result,
};
use actix_web::{web, Resource, Responder};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct RepoController<T>
where
    T: RepoServiceImpl + Send,
{
    service: Arc<Box<T>>,
}

#[async_trait]
pub trait RepoCtl: Send + Sync {
    async fn repo_datail(&self, id: String) -> Result<RepoInfoDTO>;
}

impl<T> RepoController<T>
where
    T: RepoServiceImpl,
{
    pub fn new(service: Box<T>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

#[async_trait]
impl<T> RepoCtl for RepoController<T>
where
    T: RepoServiceImpl,
{
    async fn repo_datail(&self, id: String) -> Result<RepoInfoDTO> {
        Ok(self.service.repo_info(id).await?)
    }
}

async fn repo_detail(id: web::Path<String>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    let v = ctl.repo_datail(id.into_inner()).await?;
    Ok(Response::new_success(v).response_ok())
}

#[allow(dead_code)]
pub fn scope() -> Vec<Resource> {
    let repo_ctl = web::Data::from(Arc::new(RepoController::new(Box::new(RepoService::new(
        Box::new(RepoInfoImpl::new(
            get_db().expect("no get db").clone(),
            String::from("repo_info"),
        )),
    )))) as Arc<dyn RepoCtl>);

    let mut r: Vec<Resource> = vec![];

    r.push(
        web::resource("/repo/{id}")
            .app_data(repo_ctl.clone())
            .route(web::get().to(repo_detail)),
    );

    r
}
