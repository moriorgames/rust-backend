use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    println!("Index");
    HttpResponse::Ok().body("Index")    
}
