use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name.into_inner())
}

#[actix_web::main]
pub async fn server<T:std::net::ToSocketAddrs>(v: T) -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(v)?
        .run()
        .await
}