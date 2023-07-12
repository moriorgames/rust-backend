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
    pub status: String,
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
        let status = task.status.clone();
        self.task_repository.save(task);

        CreateTaskOutput {
            id: id.to_string(),
            title: input.title.clone(),
            description: input.description.clone(),
            status: status.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::domain::task_status::TaskStatus;
    use crate::bounded_context::mocks::mock_task_repository::MockTaskRepository;
    use crate::bounded_context::mocks::mock_task_repository::{DEF_DESCRIPTION, DEF_TITLE};
    use std::cell::RefCell;

    #[test]
    fn test_create_task_execute() {
        let mock_repository = MockTaskRepository {
            selected_task: RefCell::new(None),
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
        assert_eq!(output.status, TaskStatus::Todo.to_string());
    }
}
