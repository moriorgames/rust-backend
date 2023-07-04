use crate::bounded_context::domain::{
    task::Task, task_repository::TaskRepository, task_status::TaskStatus,
};
use mysql::prelude::*;
use mysql::*;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
struct TaskRow {
    id: String,
    title: String,
    description: String,
    status: String,
}

#[derive(Debug)]
struct NotFoundError {
    message: String,
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NotFoundError {}

pub struct MySQLTaskRepository {
    conn: mysql::PooledConn,
}

impl MySQLTaskRepository {
    pub fn new(connection_url: &str) -> Result<Self, mysql::Error> {
        let pool = mysql::Pool::new(connection_url)?;
        let conn = pool.get_conn()?;
        Ok(Self { conn })
    }
}

impl TaskRepository for MySQLTaskRepository {
    fn save(&mut self, task: Task) {
        let query = "INSERT INTO task (id, title, description, status) VALUES (:id, :title, :description, :status)";

        let params = mysql::params! {
            "id" => task.id.hyphenated().to_string(),
            "title" => task.title,
            "description" => task.description,
            "status" => task.status.to_string(),
        };

        let _ = self
            .conn
            .exec_drop(query, params)
            .expect("Failed to execute query");
    }

    fn get_by_id(&mut self, id: Uuid) -> Result<Task, Box<dyn std::error::Error>> {
        let query = "SELECT * FROM task WHERE id = ?";
        let id_value = id.to_string();
        let modified_query = query.replace("?", &format!("'{}'", id_value));

        let selected_tasks =
            self.conn
                .query_map(modified_query, |(id, title, description, status)| TaskRow {
                    id,
                    title,
                    description,
                    status,
                })?;
        for item in &selected_tasks {
            let status = TaskStatus::from_string(&item.status);
            let task = Task::from_persistence(
                Uuid::parse_str(&item.id)?,
                item.title.clone(),
                item.description.clone(),
                status,
            );
            println!("{:?}", item);

            return Ok(task);
        }

        let error = NotFoundError {
            message: "Entity Not Found".to_string(),
        };

        Err(Box::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::domain::task_status::TaskStatus;
    use uuid::Uuid;

    #[test]
    fn test_save() {
        let url = "mysql://root:root@localhost:3306/rust";
        let mut repository = MySQLTaskRepository::new(url).unwrap();

        let id = Uuid::new_v4();
        let title = "Test Task".to_string();
        let description = "This is a test task".to_string();
        let status = TaskStatus::Todo;
        let task = Task::from_persistence(id, title.clone(), description.clone(), status);

        repository.save(task.clone());

        let retrieved_task = repository.get_by_id(id).unwrap();

        assert_eq!(retrieved_task.id, id);
    }
}
