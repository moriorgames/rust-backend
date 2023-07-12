pub mod task_from_persistence {

    use crate::bounded_context::domain::{task::Task, task_status::TaskStatus};
    use chrono::{DateTime, NaiveDateTime, Utc};
    use uuid::Uuid;

    pub fn create(
        id: String,
        title: String,
        description: String,
        status: String,
        created_at: String,
        updated_at: String,
    ) -> Result<Task, Box<dyn std::error::Error>> {
        let id = Uuid::parse_str(&id)?;
        let status = TaskStatus::from_string(&status)
            .ok_or_else(|| format!("Invalid task status: {}", status))?;
        let naive_created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")?;
        let created_at = DateTime::<Utc>::from_utc(naive_created_at, Utc);
        let naive_updated_at = NaiveDateTime::parse_from_str(&updated_at, "%Y-%m-%d %H:%M:%S")?;
        let updated_at = DateTime::<Utc>::from_utc(naive_updated_at, Utc);

        Ok(Task {
            id,
            title,
            description,
            status,
            created_at,
            updated_at,
        })
    }
}
