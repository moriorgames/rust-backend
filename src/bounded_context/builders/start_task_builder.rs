use crate::bounded_context::application::start_task::StartTask;
use crate::bounded_context::infrastructure::config::app_config;
use crate::bounded_context::infrastructure::mysql::mysql_task_repository::MySQLTaskRepository;

pub struct StartTaskBuilder {}

impl StartTaskBuilder {
    pub fn build() -> StartTask {
        let app_config = app_config::load_config();
        let task_repository = MySQLTaskRepository::new(&app_config.db_url)
            .expect("Failed to create MySQLTaskRepository");

        StartTask::new(Box::new(task_repository))
    }
}
