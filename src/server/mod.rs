use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::net::ToSocketAddrs;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}

#[actix_web::main]
pub async fn server<T: ToSocketAddrs>(v: T) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(v)?
        .run()
        .await
}
