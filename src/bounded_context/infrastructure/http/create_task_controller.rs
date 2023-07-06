use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::bounded_context::application::create_task::{CreateTask, CreateTaskInput, CreateTaskOutput};
use crate::bounded_context::infrastructure::mysql::mysql_task_repository::MySQLTaskRepository;

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[post("/tasks")]
async fn create_task(request: web::Json<CreateTaskRequest>) -> impl Responder {

    let mut task_repository = MySQLTaskRepository::new("mysql://root:root@localhost:3306/rust",)
                .expect("Failed to create MySQLTaskRepository");

    let mut create_task = CreateTask::new(&mut task_repository);


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

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_task),
    );
}
