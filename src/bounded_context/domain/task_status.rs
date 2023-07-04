use std::fmt;

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
    pub fn from_string(value: &str) -> TaskStatus {
        match value {
            "Todo" => TaskStatus::Todo,
            "InProgress" => TaskStatus::InProgress,
            "Done" => TaskStatus::Done,
            &_ => todo!(),
        }
    }
}
