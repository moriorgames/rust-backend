use crate::bounded_context::domain::{task::Task, task_status::TaskStatus, task_repository::TaskRepository};
use mysql::params;
use mysql::prelude::*;
use mysql::Value;
use mysql::Row;
use uuid::{uuid, Uuid};

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

    fn get_by_id(&self, id: Uuid) -> Result<Task, String> {

        let id= uuid!("00000000-0000-0000-0000-000000000001");
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();
        let task = Task::new(id, title, description);
        
        return Ok(task);

        // let query = "SELECT id, title, description, status FROM task WHERE id = :id";
        
        // let params = params! {
        //     "id" => id.hyphenated().to_string(),
        // };

        // let result = self.conn.exec_first::<Value, _, _>(query, params).map_err(|err| err.to_string())?;

        // if let Some(row) = result {

        //     let id_value = row.as_sql(true);

        //     let id_value = row
        //     .get::<&str, Value>("id")
        //     .and_then(|value| value.as_str().ok_or("Invalid ID"))?;

        //     let title = row.get::<Value, &str>("title").map_err(|_| "Invalid title")?;
        //     let description = row.get::<Value, &str>("description").map_err(|_| "Invalid description")?;
        //     let status = row.get::<Value, &str>("status").map_err(|_| "Invalid status")?;

        //     let task_id = Uuid::parse_str(id_value).map_err(|_| "Invalid UUID")?;
        //     let task_status = TaskStatus::from_str(status).map_err(|_| "Invalid status")?;

        //     let task = Task::from_persistence(task_id, title.to_string(), description.to_string(), task_status);


        //     let id= uuid!("00000000-0000-0000-0000-000000000001");
        //     let title = "Sample Task".to_string();
        //     let description = "This is a sample task".to_string();
        //     let task = Task::new(id, title, description);
        //     Ok(task)
        // } else {
        //     Err("Task not found".to_string())
        // }
    }
}
