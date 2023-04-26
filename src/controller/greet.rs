use crate::common::controller::{Response, ResponseT};
use actix_web::{web, HttpResponse, Resource, Responder};

#[allow(dead_code)]
async fn get_greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}

#[allow(dead_code)]
async fn post_greet(name: web::Path<String>) -> impl Responder {
    Response::new_success(name.into_inner().as_str()).response_ok()
}

#[allow(dead_code)]
pub fn scope() -> Vec<Resource> {
    let mut r: Vec<Resource> = vec![];

    r.push(
        web::resource("/hello/{name}")
            .route(web::get().to(get_greet))
            .route(web::post().to(post_greet)),
    );

    r
}
