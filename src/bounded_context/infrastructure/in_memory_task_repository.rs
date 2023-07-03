
use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};

pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        InMemoryTaskRepository {
            tasks: Vec::new(),
        }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
