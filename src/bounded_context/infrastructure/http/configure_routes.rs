use actix_web::web;
use crate::bounded_context::infrastructure::http::{
    create_task_controller::create_task,
    start_task_controller::start_task,
};

#[allow(dead_code)]
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_task)
            .service(start_task),
    );
}
