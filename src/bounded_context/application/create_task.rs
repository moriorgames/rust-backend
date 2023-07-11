use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};
use uuid::Uuid;

pub struct CreateTaskInput {
    pub title: String,
    pub description: String,
}

pub struct CreateTaskOutput {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub struct CreateTask {
    task_repository: Box<dyn TaskRepository>,
}

impl CreateTask {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> Self {
        CreateTask { task_repository }
    }

    pub fn execute(&mut self, input: CreateTaskInput) -> CreateTaskOutput {
        let id = Uuid::new_v4();
        let task = Task::new(id, input.title.clone(), input.description.clone());
        self.task_repository.save(task);

        CreateTaskOutput {
            id: id.to_string(),
            title: input.title.clone(),
            description: input.description.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use uuid::uuid;

    const DEF_TITLE: &str = "Sample Task";
    const DEF_DESCRIPTION: &str = "This is a sample task";

    struct MockTaskRepository {
        saved_task: RefCell<Option<Task>>,
    }

    impl TaskRepository for MockTaskRepository {
        fn save(&mut self, task: Task) {
            self.saved_task.borrow_mut().replace(task);
        }
        fn get_by_id(&mut self, _id: Uuid) -> Result<Task, Box<dyn std::error::Error>> {
            let id = uuid!("00000000-0000-0000-0000-000000000001");
            let title = "Sample Task".to_string();
            let description = "This is a sample task".to_string();
            let task = Task::new(id, title, description);

            return Ok(task);
        }
    }

    #[test]
    fn test_create_task_execute() {
        let mock_repository = MockTaskRepository {
            saved_task: RefCell::new(None),
        };
        let mut sut = CreateTask::new(Box::new(mock_repository));

        let input = CreateTaskInput {
            title: DEF_TITLE.to_string(),
            description: DEF_DESCRIPTION.to_string(),
        };
        let output = sut.execute(input);

        assert_eq!(output.title, DEF_TITLE.to_string());
        assert_eq!(output.description, DEF_DESCRIPTION.to_string());
    }
}
