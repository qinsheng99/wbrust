pub mod repo_info;

use actix_web::{web, Scope};

pub fn get_scope(path: &str) -> Scope {
    web::scope(path).service(repo_info::scope())
}
