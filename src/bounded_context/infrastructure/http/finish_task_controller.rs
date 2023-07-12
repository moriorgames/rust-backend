use crate::bounded_context::application::finish_task::FinishTaskInput;
use crate::bounded_context::builders::finish_task_builder::finish_task_builder;
use crate::bounded_context::infrastructure::http::task_response::TaskResponse;
use actix_web::{put, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FinishTaskRequest {
    pub id: String,
}

#[put("/finish_task/{id}")]
async fn finish_task(request: web::Json<FinishTaskRequest>) -> impl Responder {
    println!("Finish Task");
    let mut finish_task = finish_task_builder::build();

    let input = FinishTaskInput {
        id: request.id.to_string(),
    };
    let output = finish_task.execute(input).unwrap();

    let response = TaskResponse {
        id: output.id.clone(),
        title: output.title.clone(),
        description: output.description.clone(),
        status: output.status.clone(),
    };

    HttpResponse::Ok().json(response)
}
