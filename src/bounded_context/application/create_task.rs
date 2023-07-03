
use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};

pub struct CreateTask<'a> {
    task_repository: &'a mut dyn TaskRepository,
}

impl<'a> CreateTask<'a> {
    pub fn new(task_repository: &'a mut dyn TaskRepository) -> Self {
        CreateTask { task_repository }
    }

    pub fn execute(&mut self, title: String, description: String) -> u32 {
        let id = 234;
        let task = Task::new(id, title, description);
        self.task_repository.save(task);

        return id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockTaskRepository {
        saved_task: RefCell<Option<Task>>,
    }

    impl TaskRepository for MockTaskRepository {
        fn save(&mut self, task: Task) {
            self.saved_task.borrow_mut().replace(task);
        }
    }

    #[test]
    fn test_create_task_execute() {
        let mut mock_repository = MockTaskRepository {
            saved_task: RefCell::new(None),
        };
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();

        let mut sut = CreateTask::new(&mut mock_repository);
        sut.execute(title.clone(), description.clone());

        let saved_task = mock_repository.saved_task.borrow();
        assert!(saved_task.is_some());
        let saved_task = saved_task.as_ref().unwrap();
        assert_eq!(saved_task.title, title);
        assert_eq!(saved_task.description, description);
    }
}
