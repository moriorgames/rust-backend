pub mod finish_task_builder {
    use crate::bounded_context::application::finish_task::FinishTask;
    use crate::bounded_context::infrastructure::config::app_config;
    use crate::bounded_context::infrastructure::mysql::mysql_task_repository::MySQLTaskRepository;

    pub fn build() -> FinishTask {
        let app_config = app_config::load_config();
        let task_repository = MySQLTaskRepository::new(&app_config.db_url)
            .expect("Failed to create MySQLTaskRepository");

        FinishTask::new(Box::new(task_repository))
    }
}
