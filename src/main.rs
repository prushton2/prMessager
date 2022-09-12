#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod object;

use actix_web::{web, App, HttpServer, Responder};

static mut userContainer: object::UserContainer::UserContainer = object::UserContainer::new();

#[actix_web::get("/signUp/{name}")]
async fn signUp(name: web::Path<String>) -> impl Responder {
	let user: object::User::User = object::User::new("Peter");
    user.key
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