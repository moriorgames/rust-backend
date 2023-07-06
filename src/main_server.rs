mod bounded_context;
use actix_web::{App, HttpServer};
use bounded_context::infrastructure::http::hello_world_controller::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hello = bounded_context::infrastructure::http::hello_world_controller::hello;
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
