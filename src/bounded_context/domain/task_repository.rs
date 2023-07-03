use super::task::Task;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
}

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
