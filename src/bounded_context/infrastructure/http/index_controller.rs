use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    println!("Index");
    HttpResponse::Ok().body("Index")
}
