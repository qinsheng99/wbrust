pub mod greet;

use actix_web::{web, Scope};

pub fn get_greet_scope(path: &str) -> Scope {
    web::scope(path).service(web::resource("/hello/{name}").route(web::get().to(greet::greet)))
}
