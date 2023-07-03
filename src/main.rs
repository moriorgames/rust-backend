mod bounded_context;

fn main() {    
    
    let id = 1;
    let title = "Sample Task".to_string();
    let description = "This is a sample task".to_string();
    let task = bounded_context::domain::task::Task::new(id, title, description);

    println!("world!");
    println!("{:?}", task.title);
}
