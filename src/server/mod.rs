use crate::controller::greet::greet;
use actix_web::{web, App, HttpServer};
use std::net::ToSocketAddrs;

#[actix_web::main]
pub async fn server<T: ToSocketAddrs>(v: T) -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello/{name}", web::get().to(greet)))
        .bind(v)?
        .run()
        .await
}
