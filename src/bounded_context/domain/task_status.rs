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
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Done => write!(f, "Done"),
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
