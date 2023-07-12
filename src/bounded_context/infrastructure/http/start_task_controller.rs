use crate::bounded_context::application::start_task::StartTaskInput;
use crate::bounded_context::builders::start_task_builder::start_task_builder;
use crate::bounded_context::infrastructure::http::task_response::TaskResponse;
use actix_web::{put, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StartTaskRequest {
    pub id: String,
}

#[put("/start_task/{id}")]
async fn start_task(request: web::Json<StartTaskRequest>) -> impl Responder {
    println!("Start Task");
    let mut start_task = start_task_builder::build();

    let input = StartTaskInput {
        id: request.id.to_string(),
    };
    let output = start_task.execute(input).unwrap();

    let response = TaskResponse {
        id: output.id.clone(),
        title: output.title.clone(),
        description: output.description.clone(),
        status: output.status.clone(),
    };

    HttpResponse::Ok().json(response)
}
