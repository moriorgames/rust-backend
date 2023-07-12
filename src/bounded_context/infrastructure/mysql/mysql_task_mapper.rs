use crate::bounded_context::domain::task::Task;
use crate::bounded_context::infrastructure::mysql::task_from_persistence::task_from_persistence;

#[derive(Debug)]
pub struct TaskRow {
    id: String,
    title: String,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl TaskRow {
    pub fn new(
        id: String,
        title: String,
        description: String,
        status: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
            created_at,
            updated_at,
        }
    }
}

pub struct MysqlTaskMapper {}

impl MysqlTaskMapper {
    pub fn map_to_task(&self, row: TaskRow) -> Result<Task, Box<dyn std::error::Error>> {
        match task_from_persistence::create(
            row.id,
            row.title.clone(),
            row.description.clone(),
            row.status,
            row.created_at,
            row.updated_at,
        ) {
            Ok(task) => Ok(task),
            Err(e) => Err(e),
        }
    }
}
