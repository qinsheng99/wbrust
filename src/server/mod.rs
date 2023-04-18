use crate::controller;
use actix_web::{App, HttpServer};
use std::net::ToSocketAddrs;

#[actix_web::main]
pub async fn server<T: ToSocketAddrs>(v: T, path: &'static str) -> std::io::Result<()> {
    let s = HttpServer::new(|| App::new().service(controller::get_greet_scope(path)));

    s.bind(v)?.run().await
}
