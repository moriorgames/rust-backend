use super::task_status::TaskStatus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: Uuid, title: String, description: String) -> Task {
        let now = Utc::now();
        Task {
            id,
            title,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
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
    use crate::bounded_context::mocks::mock_task_repository::{DEF_DESCRIPTION, DEF_ID, DEF_TITLE};

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
