use crate::bounded_context::domain::task_repository::TaskRepository;
use uuid::Uuid;

pub struct StartTaskInput {
    pub id: String,
}

pub struct StartTaskOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

pub struct StartTask<'a> {
    task_repository: &'a mut dyn TaskRepository,
}

impl<'a> StartTask<'a> {
    pub fn new(task_repository: &'a mut dyn TaskRepository) -> Self {
        StartTask { task_repository }
    }

    pub fn execute(&mut self, input: StartTaskInput) -> StartTaskOutput {
        let id = Uuid::parse_str(&input.id.to_string()).unwrap();
        let task_result = self.task_repository.get_by_id(id);

        if let Ok(mut task) = task_result {
            task.start_task();
            self.task_repository.save(task.clone());

            StartTaskOutput {
                id: task.id.to_string(),
                title: task.title,
                description: task.description,
                status: task.status.to_string(),
            }
        } else {
            // @TODO manage an exception here
            StartTaskOutput {
                id: String::new(),
                title: String::new(),
                description: String::new(),
                status: String::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::domain::{task::Task, task_status::TaskStatus};
    use std::cell::RefCell;
    use uuid::uuid;

    const DEF_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
    const DEF_TITLE: &str = "Sample Task";
    const DEF_DESCRIPTION: &str = "This is a sample task";

    struct MockTaskRepository {
        selected_task: RefCell<Option<Task>>,
        saved_task: RefCell<Option<Task>>,
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

    #[test]
    fn test_start_task_execute() {
        let mut mock_repository = MockTaskRepository {
            selected_task: RefCell::new(None),
            saved_task: RefCell::new(None),
        };
        let mut sut = StartTask::new(&mut mock_repository);

        let input = StartTaskInput {
            id: DEF_ID.to_string(),
        };
        let output = sut.execute(input);

        let selected_task = mock_repository.selected_task.borrow();
        assert!(selected_task.is_some());

        let saved_task = mock_repository.saved_task.borrow();
        assert!(saved_task.is_some());
        let saved_task = saved_task.as_ref().unwrap();
        assert_eq!(output.id, saved_task.id.to_string());
        assert_eq!(output.title, DEF_TITLE.to_string());
        assert_eq!(output.description, DEF_DESCRIPTION.to_string());
        assert_eq!(output.status, TaskStatus::InProgress.to_string());
    }
}
