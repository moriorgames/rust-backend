use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};
use mysql::prelude::*;
use  mysql::params;

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
        let query = "INSERT INTO task (title, description, status) VALUES (:title, :description, :status)";

        let params = mysql::params! {
            "title" => task.title,
            "description" => task.description,
            "status" => task.status.to_string(),
        };

        let _ = self.conn.exec_drop(query, params).expect("Failed to execute query");
    }
}
