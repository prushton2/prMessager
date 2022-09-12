#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod object;

use actix_web::{web, App, HttpServer, Responder};


#[actix_web::get("/signUp/{name}")]
async fn signUp(name: web::Path<String>) -> impl Responder {
    "Hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    HttpServer::new(|| {
        App::new()
            .service(signUp)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}