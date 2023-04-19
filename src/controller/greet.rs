use actix_web::{web, HttpResponse, Resource, Responder};

#[allow(dead_code)]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}

pub fn score() -> Resource {
    web::resource("/hello/{name}").route(web::get().to(greet))
}
