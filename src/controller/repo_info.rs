use crate::{
    app::repo_info::{RepoService, RepoServiceImpl},
    common::infrastructure::postgresql::get_db,
    infrastructure::repositoryimpl::repo_info::RepoInfoImpl,
    utils::error::{Error, Result},
};
use actix_web::{web, HttpResponse, Resource, Responder};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct RepoController<T>
where
    T: RepoServiceImpl,
{
    service: T,
}

impl<T> RepoController<T>
where
    T: RepoServiceImpl,
{
    pub fn new(service: T) -> Self {
        Self { service }
    }
}

async fn repo_detail(id: web::Path<String>) -> Result<impl Responder> {
    let _c = get_ctl()?;
    Ok(HttpResponse::Ok().body(id.into_inner()))
}

type Ctl = RepoController<RepoService<RepoInfoImpl>>;

static CTL: OnceCell<Ctl> = OnceCell::new();

fn get_ctl() -> Result<&'static Ctl> {
    match CTL.get() {
        Some(db) => Ok(db.clone()),
        None => Err(Error::ParamError(String::from("no repo ctl"))),
    }
}

#[allow(dead_code)]
pub fn scope() -> Vec<Resource> {
    let c = RepoController::new(RepoService::new(RepoInfoImpl::new(
        get_db().expect(""),
        String::from("repo_info"),
    )));

    CTL.set(c).expect("no set repo ctl");

    let r: Vec<Resource> = vec![web::resource("/repo/{id}").route(web::get().to(repo_detail))];

    r
}
