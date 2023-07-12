use crate::bounded_context::application::create_task::CreateTaskInput;
use crate::bounded_context::builders::create_task_builder::CreateTaskBuilder;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[post("/task")]
async fn create_task(request: web::Json<CreateTaskRequest>) -> impl Responder {
    println!("Create Task");
    let mut create_task = CreateTaskBuilder::build();

    let input = CreateTaskInput {
        title: request.title.clone(),
        description: request.description.clone(),
    };
    let output = create_task.execute(input);

    let response = CreateTaskResponse {
        id: output.id.clone(),
        title: output.title.clone(),
        description: output.description.clone(),
        status: output.status.clone(),
    };

    HttpResponse::Ok().json(response)
}
