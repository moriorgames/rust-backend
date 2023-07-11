use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::bounded_context::builders::create_task_builder::CreateTaskBuilder;
use crate::bounded_context::application::create_task::{CreateTask, CreateTaskInput};

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

#[post("/tasks")]
async fn create_task(request: web::Json<CreateTaskRequest>) -> impl Responder {

    let mut create_task = CreateTaskBuilder::build();

    let create_task_input = CreateTaskInput {
        title: request.title.clone(),
        description: request.description.clone(),
    };
    let result = create_task.execute(create_task_input);

    let response = CreateTaskResponse {
        id: result.id,
        title: result.title.clone(),
        description: result.description.clone(),
        status: "Fake status".to_string(),
    };

    HttpResponse::Ok().json(response)
}
