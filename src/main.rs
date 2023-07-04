mod bounded_context;
use bounded_context::application::create_task::CreateTaskInput;
use bounded_context::infrastructure::mysql_task_repository::MySQLTaskRepository;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut task_repository = MySQLTaskRepository::new("mysql://root:root@localhost:3306/rust")
        .expect("Failed to create MySQLTaskRepository");

    let mut create_task =
        bounded_context::application::create_task::CreateTask::new(&mut task_repository);

    let title = "Sample Task".to_string();
    let description = "This is a sample task".to_string();
    let input = CreateTaskInput { title, description };
    let output = create_task.execute(input);

    println!("Hello world!");
    println!("{:?}", output.id);

    Ok(())
}
