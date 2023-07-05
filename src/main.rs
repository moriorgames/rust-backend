mod bounded_context;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut task_repository =
        bounded_context::infrastructure::mysql_task_repository::MySQLTaskRepository::new(
            "mysql://root:root@localhost:3306/rust",
        )
        .expect("Failed to create MySQLTaskRepository");

    let mut create_task =
        bounded_context::application::create_task::CreateTask::new(&mut task_repository);

    let title = "Sample Task".to_string();
    let description = "This is a sample task".to_string();
    let input = bounded_context::application::create_task::CreateTaskInput { title, description };
    let output = create_task.execute(input);

    println!("");
    println!("Created task with ID:  {:?}", output.id);
    println!("");

    let mut start_task =
        bounded_context::application::start_task::StartTask::new(&mut task_repository);

    let id = output.id;
    let input = bounded_context::application::start_task::StartTaskInput { id };
    let output = start_task.execute(input);

    println!("");
    println!("Started task with ID:  {:?}", output.id);
    println!("");

    Ok(())
}
