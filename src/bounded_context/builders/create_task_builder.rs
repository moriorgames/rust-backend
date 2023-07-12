use crate::bounded_context::application::create_task::CreateTask;
use crate::bounded_context::infrastructure::config::app_config;
use crate::bounded_context::infrastructure::mysql::mysql_task_repository::MySQLTaskRepository;

pub struct CreateTaskBuilder {}

impl CreateTaskBuilder {
    pub fn build() -> CreateTask {
        let app_config = app_config::load_config();
        let task_repository = MySQLTaskRepository::new(&app_config.db_url)
            .expect("Failed to create MySQLTaskRepository");

        CreateTask::new(Box::new(task_repository))
    }
}
