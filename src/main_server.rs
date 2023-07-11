mod bounded_context;
use actix_web::{App, HttpServer};
use bounded_context::infrastructure::http::{
    index_controller::index,
    configure_routes
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index = bounded_context::infrastructure::http::index_controller::index;
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(configure_routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
