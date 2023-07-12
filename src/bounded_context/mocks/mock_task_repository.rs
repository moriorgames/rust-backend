use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};
use std::cell::RefCell;
use uuid::{uuid, Uuid};

pub const DEF_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
pub const DEF_TITLE: &str = "Sample Task";
pub const DEF_DESCRIPTION: &str = "This is a sample task";

pub struct MockTaskRepository {
    pub selected_task: RefCell<Option<Task>>,
    pub saved_task: RefCell<Option<Task>>,
}

impl TaskRepository for MockTaskRepository {
    fn save(&mut self, task: Task) {
        self.saved_task.borrow_mut().replace(task);
    }
    fn get_by_id(&mut self, _id: Uuid) -> Result<Task, Box<dyn std::error::Error>> {
        let id = DEF_ID;
        let title = DEF_TITLE.to_string();
        let description = DEF_DESCRIPTION.to_string();
        let task = Task::new(id, title, description);
        self.selected_task.borrow_mut().replace(task.clone());

        return Ok(task);
    }
}

pub struct MockTaskRepositoryError {
    pub error_message: String,
}

impl TaskRepository for MockTaskRepositoryError {
    fn save(&mut self, _task: Task) {
        // Intentionally do nothing
    }
    fn get_by_id(&mut self, _id: Uuid) -> Result<Task, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            self.error_message.clone(),
        )))
    }
}
