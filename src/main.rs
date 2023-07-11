mod bounded_context;
use bounded_context::infrastructure::config::app_config;
use std::error::Error;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {

    // let config = app_config::load_config();

    // let mut task_repository =
    //     bounded_context::infrastructure::mysql::mysql_task_repository::MySQLTaskRepository::new(
    //         &config.db_url,
    //     )
    //     .expect("Failed to create MySQLTaskRepository");

    // let mut create_task =
    //     bounded_context::application::create_task::CreateTask::new(&mut task_repository);

    // let title = "Sample Task".to_string();
    // let description = "This is a sample task".to_string();
    // let input = bounded_context::application::create_task::CreateTaskInput { title, description };
    // let output = create_task.execute(input);

    // println!("");
    // println!("Created task with ID:  {:?}", output.id);
    // println!("");
    // let millis = time::Duration::from_millis(3000);
    // let now = time::Instant::now();
    // thread::sleep(millis);
    // assert!(now.elapsed() >= millis);
    // println!("Elasped: {:?}", millis);

    // let mut start_task =
    //     bounded_context::application::start_task::StartTask::new(&mut task_repository);

    // let id = output.id;
    // let input = bounded_context::application::start_task::StartTaskInput { id };
    // let output = start_task.execute(input);

    // println!("");
    // println!("Started task with ID: {:?} and finished with status {:?}", output.id, output.status);
    // println!("");
    // let millis = time::Duration::from_millis(3000);
    // let now = time::Instant::now();
    // thread::sleep(millis);
    // assert!(now.elapsed() >= millis);
    // println!("Elasped: {:?}", millis);

    // let mut finish_task =
    //     bounded_context::application::finish_task::FinishTask::new(&mut task_repository);

    // let id = output.id;
    // let input = bounded_context::application::finish_task::FinishTaskInput { id };
    // let output = finish_task.execute(input);

    // println!("");
    // println!("Started task with ID: {:?} and finished with status {:?}", output.id, output.status);
    // println!("");

    Ok(())
}
