use crate::bounded_context::domain::{task::Task, task_repository::TaskRepository};
use crate::bounded_context::infrastructure::mysql::mysql_task_mapper::{MysqlTaskMapper, TaskRow};
use crate::bounded_context::infrastructure::mysql::repository_error::RepositoryError;
use mysql::prelude::*;
use mysql::*;
use uuid::Uuid;

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
        // @TODO this implementacion can be improved with other strategies or an ORM
        let query = "REPLACE INTO task (id, title, description, status) VALUES (:id, :title, :description, :status)";

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

        let selected_rows = self.conn.query_map(
            modified_query,
            |(id, title, description, status, created_at, updated_at)| {
                TaskRow::new(id, title, description, status, created_at, updated_at)
            },
        )?;

        let task_mapper = MysqlTaskMapper {};

        if let Some(row) = selected_rows.into_iter().next() {
            let task = task_mapper.map_to_task(row)?;
            return Ok(task);
        }

        let error = RepositoryError::new("Entity Not Found");

        Err(Box::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::domain::task_status::TaskStatus;
    use crate::bounded_context::infrastructure::config::app_config;
    use crate::bounded_context::infrastructure::mysql::task_from_persistence::task_from_persistence;
    use uuid::Uuid;

    #[test]
    fn test_save_and_retrieve_data() {
        let config = app_config::load_config();
        let mut sut = MySQLTaskRepository::new(&config.db_url).unwrap();

        let id = Uuid::new_v4();
        let title = "Test Task".to_string();
        let description = "This is a test task".to_string();
        let status = TaskStatus::Todo.to_string();
        let created_at = "2023-05-05 12:15:00".to_string();
        let updated_at = "2023-05-05 12:15:00".to_string();
        let task = task_from_persistence::create(
            id.to_string(),
            title.clone(),
            description.clone(),
            status,
            created_at,
            updated_at,
        )
        .unwrap();

        sut.save(task.clone());

        let retrieved_task = sut.get_by_id(id).unwrap();

        assert_eq!(retrieved_task.id, id);
    }
}
