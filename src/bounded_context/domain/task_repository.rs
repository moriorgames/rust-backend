use super::task::Task;
use uuid::Uuid;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
    fn get_by_id(&self, id: Uuid) -> Result<Task, String>;
}
