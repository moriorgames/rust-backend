use super::task_status::TaskStatus;
use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: Uuid, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn from_persistence(
        id: Uuid,
        title: String,
        description: String,
        status: TaskStatus,
    ) -> Task {
        Task {
            id,
            title,
            description,
            status,
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
    use uuid::{uuid, Uuid};

    const DEF_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
    const DEF_TITLE: &str = "Sample Task";
    const DEF_DESCRIPTION: &str = "This is a sample task";
    const DEF_STATUS: TaskStatus = TaskStatus::InProgress;

    fn create_task_with_defaults() -> Task {
        Task::new(DEF_ID, DEF_TITLE.to_string(), DEF_DESCRIPTION.to_string())
    }

    #[test]
    fn test_task_constructor() {
        let sut = create_task_with_defaults();

        assert_eq!(sut.id, DEF_ID);
        assert_eq!(sut.title, DEF_TITLE.to_string());
        assert_eq!(sut.description, DEF_DESCRIPTION.to_string());
    }

    #[test]
    fn test_task_from_persistence() {
        let sut = Task::from_persistence(
            DEF_ID,
            DEF_TITLE.to_string(),
            DEF_DESCRIPTION.to_string(),
            DEF_STATUS,
        );

        assert_eq!(sut.id, DEF_ID);
        assert_eq!(sut.title, DEF_TITLE.to_string());
        assert_eq!(sut.description, DEF_DESCRIPTION.to_string());
        assert_eq!(sut.status, DEF_STATUS);
    }

    #[test]
    fn test_start_task_changes_status_to_in_progress() {
        let mut sut = create_task_with_defaults();
        let expected_status_before = TaskStatus::Todo;
        let expected_status_after = TaskStatus::InProgress;

        assert_eq!(sut.status, expected_status_before);

        sut.start_task();

        assert_eq!(sut.status, expected_status_after);
    }

    #[test]
    fn test_finish_task_changes_status_to_done() {
        let mut sut = create_task_with_defaults();
        let expected_status_before = TaskStatus::Todo;
        let expected_status_after = TaskStatus::Done;

        assert_eq!(sut.status, expected_status_before);

        sut.finish_task();

        assert_eq!(sut.status, expected_status_after);
    }
}
