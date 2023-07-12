use std::fmt;

const TODO_VALUE: &str = "Todo";
const IN_PROGRESS_VALUE: &str = "InProgress";
const DONE_VALUE: &str = "Done";

#[derive(Debug, PartialEq, Clone)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "{}", TODO_VALUE),
            TaskStatus::InProgress => write!(f, "{}", IN_PROGRESS_VALUE),
            TaskStatus::Done => write!(f, "{}", DONE_VALUE),
        }
    }
}

impl TaskStatus {
    pub fn from_string(value: &str) -> Option<TaskStatus> {
        match value {
            TODO_VALUE => Some(TaskStatus::Todo),
            IN_PROGRESS_VALUE => Some(TaskStatus::InProgress),
            DONE_VALUE => Some(TaskStatus::Done),
            &_ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TaskStatus::Todo), TODO_VALUE);
        assert_eq!(format!("{}", TaskStatus::InProgress), IN_PROGRESS_VALUE);
        assert_eq!(format!("{}", TaskStatus::Done), DONE_VALUE);
    }

    #[test]
    fn test_from_string() {
        assert_eq!(TaskStatus::from_string(TODO_VALUE), Some(TaskStatus::Todo));
        assert_eq!(
            TaskStatus::from_string(IN_PROGRESS_VALUE),
            Some(TaskStatus::InProgress)
        );
        assert_eq!(TaskStatus::from_string(DONE_VALUE), Some(TaskStatus::Done));
        assert_eq!(TaskStatus::from_string("InvalidStatus"), None);
    }
}
