pub mod task_from_persistence {

    use crate::bounded_context::domain::{task::Task, task_status::TaskStatus};
    use uuid::Uuid;

    pub fn create(
        id: String,
        title: String,
        description: String,
        status: String,
    ) -> Result<Task, Box<dyn std::error::Error>> {
        let id = Uuid::parse_str(&id)?;
        let status = TaskStatus::from_string(&status)
            .ok_or_else(|| format!("Invalid task status: {}", status))?;

        Ok(Task {
            id,
            title,
            description,
            status,
        })
    }
}
