use crate::bounded_context::infrastructure::http::{
    create_task_controller::create_task, finish_task_controller::finish_task,
    start_task_controller::start_task,
};
use actix_web::web;

#[allow(dead_code)]
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_task)
            .service(start_task)
            .service(finish_task),
    );
}
