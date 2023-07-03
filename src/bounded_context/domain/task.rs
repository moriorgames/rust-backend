use super::task_status::TaskStatus;

pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn start_task(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    pub fn finish_task(&mut self) {
        self.status = TaskStatus::Done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_constructor() {
        let id = 1;
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();

        let sut = Task::new(id, title.clone(), description.clone());

        assert_eq!(sut.id, id);
        assert_eq!(sut.title, title);
        assert_eq!(sut.description, description);
    }

    #[test]
    fn test_start_task_changes_status_to_in_progress() {
        let id = 1;
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();
        let expected_status_before = TaskStatus::Todo;
        let expected_status_after = TaskStatus::InProgress;

        let mut sut = Task::new(id, title, description);

        assert_eq!(sut.status, expected_status_before);

        sut.start_task();

        assert_eq!(sut.status, expected_status_after);
    }

    #[test]
    fn test_finisht_task_changes_status_to_done() {
        let id = 1;
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();
        let expected_status_before = TaskStatus::Todo;
        let expected_status_after = TaskStatus::Done;

        let mut sut = Task::new(id, title, description);

        assert_eq!(sut.status, expected_status_before);

        sut.finish_task();

        assert_eq!(sut.status, expected_status_after);
    }
}
