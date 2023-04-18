use actix_web::{web, HttpResponse, Responder};

#[allow(dead_code)]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}
