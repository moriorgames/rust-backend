use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    println!("Hello, World!");
    HttpResponse::Ok().body("Hello, World!")
}
