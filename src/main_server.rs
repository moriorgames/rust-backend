mod bounded_context;
use actix_web::{App, HttpServer};
use bounded_context::infrastructure::http::{
    index_controller::index,
    tmp_controller::tmp,
    configure_routes
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index = bounded_context::infrastructure::http::index_controller::index;
    let tmp = bounded_context::infrastructure::http::tmp_controller::tmp;
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(tmp)
            .configure(configure_routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
