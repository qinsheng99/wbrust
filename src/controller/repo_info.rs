use crate::{
    app::{
        dto::RepoInfoDTO,
        repo_info::{RepoService, RepoServiceImpl},
    },
    common::{
        controller::{Response, ResponseT, Str},
        infrastructure::postgresql::get_db,
    },
    controller::repo_info_request::RepoInfoRequest,
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
    async fn add(&self, v: RepoInfoRequest) -> Result<()>;
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

    async fn add(&self, v: RepoInfoRequest) -> Result<()> {
        let cmd = v.new();
        Ok(self.service.add(cmd).await?)
    }
}

async fn repo_detail(id: web::Path<String>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    let v = ctl.repo_datail(id.into_inner()).await?;
    Ok(Response::new_success(v).response_ok())
}

async fn add(v: web::Path<RepoInfoRequest>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    let _v = ctl.add(v.into_inner()).await?;
    Ok(Response::new_success(Str::new("success".to_string())).response_ok())
}

#[allow(dead_code)]
pub fn scope() -> Vec<Resource> {
    let repo_ctl = web::Data::from(Arc::new(RepoController::new(Box::new(RepoService::new(
        Box::new(RepoInfoImpl::new(
            get_db().expect("no get db").clone(),
            String::from("repo_info"),
        )),
    )))) as Arc<dyn RepoCtl>);

    let r: Vec<Resource> = vec![
        web::resource("/repo/{id}")
            .app_data(repo_ctl.clone())
            .route(web::get().to(repo_detail)),
        web::resource("/repo")
            .app_data(repo_ctl.clone())
            .route(web::post().to(add)),
    ];

    r
}
