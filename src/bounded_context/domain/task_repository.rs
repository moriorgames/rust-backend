use super::task::Task;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
}
