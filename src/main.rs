use actix_web::{web, App, HttpServer, Responder};
use rand::prelude::*;

#[actix_web::get("/signUp/{name}")]
async fn ping(name: web::Path<String>) -> impl Responder {
    createKey()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn createKey() -> String {
    let mut rng = rand::thread_rng();
    let validChars: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    
    let mut key: String = String::from("");
    
    for i in 0..32 {
        let mut flt: f64 = rng.gen();
        flt *= 26.0;
        key += validChars[flt as usize];
    }
    
    key.to_string()
}