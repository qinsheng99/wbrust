pub mod repo_info;
pub mod repo_info_request;

use actix_web::{web, Scope};

pub fn get_repo_scope(path: &str) -> Scope {
    web::scope(path).service(repo_info::scope())
}
