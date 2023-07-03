mod bounded_context;

fn main() {

    let mut task_repository = bounded_context::infrastructure::in_memory_task_repository::InMemoryTaskRepository::new();
    let mut create_task = bounded_context::application::create_task::CreateTask::new(&mut task_repository);

    let title = "Sample Task".to_string();
    let description = "This is a sample task".to_string();
    let id = create_task.execute(title, description);

    println!("Hello world!");
    println!("{:?}", id);
}
