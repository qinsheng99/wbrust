use {
    crate::{
        app::{
            dto::{RepoInfoDTO, RepoInfoListDTO},
            repo_info::{NewRepoService, NewRepoServiceImpl, RepoService, RepoServiceImpl},
        },
        common::{
            controller::{Response, ResponseT, Str},
            infrastructure::{mysql::get_db_connection, postgresql::get_db, redis::get_redis_db},
        },
        controller::repo::repo_info_request::{ListQuery, RepoInfoRequest},
        infrastructure::repositoryimpl::repo::repo_info::{NewRepoInfo, RepoInfoImpl},
        utils::error::Result,
    },
    actix_web::{web, Resource, Responder},
    async_trait::async_trait,
    redis::AsyncCommands,
    std::sync::Arc,
};

#[derive(Debug)]
pub struct RepoController<T>
where
    T: RepoServiceImpl + Send,
{
    service: Arc<Box<T>>,
}

#[derive(Debug)]
pub struct NewRepoController<T>
where
    T: NewRepoServiceImpl + Send,
{
    service: Arc<Box<T>>,
}

#[async_trait]
pub trait RepoCtl: Send + Sync {
    async fn repo_detail(&self, id: String) -> Result<RepoInfoDTO>;
    async fn add(&self, v: RepoInfoRequest) -> Result<()>;
    async fn list(&self, v: ListQuery) -> Result<RepoInfoListDTO>;
}

#[async_trait]
pub trait NewRepoCtl: Send + Sync {
    async fn repo_detail(&self, id: u64) -> Result<()>;
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

impl<T> NewRepoController<T>
where
    T: NewRepoServiceImpl,
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
    async fn repo_detail(&self, id: String) -> Result<RepoInfoDTO> {
        Ok(self.service.repo_info(id).await?)
    }

    async fn add(&self, v: RepoInfoRequest) -> Result<()> {
        let cmd = v.to_cmd();
        Ok(self.service.add(cmd).await?)
    }

    async fn list(&self, v: ListQuery) -> Result<RepoInfoListDTO> {
        let cmd = v.to_cmd();
        Ok(self.service.list(cmd).await?)
    }
}

#[async_trait]
impl<T> NewRepoCtl for NewRepoController<T>
where
    T: NewRepoServiceImpl,
{
    async fn repo_detail(&self, id: u64) -> Result<()> {
        self.service.repo_info(id).await?;
        Ok(())
    }
}

async fn repo_detail(id: web::Path<String>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    let v = ctl.repo_detail(id.into_inner()).await?;
    Ok(Response::new_success(v).response_ok())
}

async fn add(v: web::Json<RepoInfoRequest>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    let _v = ctl.add(v.into_inner()).await?;
    Ok(Response::new_success(Str::new("success".to_string())).response_ok())
}

async fn list(v: web::Query<ListQuery>, ctl: web::Data<dyn RepoCtl>) -> Result<impl Responder> {
    test_redis().await.expect("TODO: panic message");
    let c = ctl.list(v.into_inner()).await?;
    Ok(Response::new_success(c).response_ok())
}

async fn new_repo_detail(
    id: web::Path<u64>,
    ctl: web::Data<dyn NewRepoCtl>,
) -> Result<impl Responder> {
    let _v = ctl.repo_detail(id.into_inner()).await?;
    Ok(Response::new_success("s".to_string()).response_ok())
}

async fn test_redis() -> Result<()> {
    let mut c = get_redis_db()?;
    let value: String = c.get("name").await?;
    println!("{}", value);
    Ok(())
}

#[allow(dead_code)]
pub fn scope() -> Vec<Resource> {
    let repo_ctl = web::Data::from(Arc::new(RepoController::new(Box::new(RepoService::new(
        Box::new(RepoInfoImpl::new(
            get_db().expect("no get db").clone(),
            String::from("repo_info"),
        )),
    )))) as Arc<dyn RepoCtl>);

    let new_repo_ctl = web::Data::from(Arc::new(NewRepoController::new(Box::new(
        NewRepoService::new(Box::new(NewRepoInfo::new(get_db_connection().unwrap()))),
    ))) as Arc<dyn NewRepoCtl>);

    let r: Vec<Resource> = vec![
        web::resource("/repo/{id}")
            .app_data(repo_ctl.clone())
            .route(web::get().to(repo_detail)),
        web::resource("/new_repo/{id}")
            .app_data(new_repo_ctl.clone())
            .route(web::get().to(new_repo_detail)),
        web::resource("/repo")
            .app_data(repo_ctl.clone())
            .route(web::post().to(add)),
        web::resource("/list/repo")
            .app_data(repo_ctl.clone())
            .route(web::get().to(list)),
    ];

    r
}
