use actix_web::{get, HttpResponse, Responder};
use crate::bounded_context::infrastructure::bootstrapper::Bootstrapper;

#[get("/tmp")]
pub async fn tmp() -> impl Responder {
    let bootstrap = Bootstrapper::new();
    let db_url = bootstrap.bootstrap().db_url.clone();

    println!("Print some value: {}", db_url);
    HttpResponse::Ok().body(format!("Print some value: {}", db_url))
}
