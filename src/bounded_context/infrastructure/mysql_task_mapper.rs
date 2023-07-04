use crate::bounded_context::domain::{
    task::Task, task_status::TaskStatus,
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct TaskRow {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

pub struct MysqlTaskMapper {}

impl MysqlTaskMapper {
    pub fn map_to_task(&self, row: TaskRow) -> Result<Task, Box<dyn std::error::Error>> {
        let status = TaskStatus::from_string(&row.status);
        let task = Task::from_persistence(
            Uuid::parse_str(&row.id)?,
            row.title.clone(),
            row.description.clone(),
            status,
        );
        Ok(task)
    }
}
