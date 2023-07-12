# Rust Task Manager Backend

This project is a simple task management backend written in Rust. 

## Quick Start

Here is how you can get the project up and running:

```
# Clone the repository
$ git clone <repo_url>

# Change into project directory
$ cd rust-backend

# Run the project
$ cargo run

# Test the project
$ cargo test
```

## Setup MySQL Database with Docker

This project uses MySQL for persistent data storage. Here's how you can setup the database using Docker:

```
# Start the Docker services
$ docker-compose up -d

# Initialize the database with a table
$ docker exec -i db mysql -uroot -proot < src/bounded_context/infrastructure/mysql/task.sql

# Alternative you can log into the MySQL container with this command
$ docker exec -it db mysql -uroot -proot rust
```

## Testing and Code Coverage

You can run the test suite and generate a code coverage report with these commands:

```
# Install the code coverage tool
$ cargo install cargo-tarpaulin

# Run the code coverage tool
$ cargo tarpaulin --output-dir target/debug/tarpaulin/ --out html

# Open the code coverage report on Google Chrome
$ open -a "Google Chrome" ./target/debug/tarpaulin/tarpaulin-report.html

# Test a single file
$ cargo test --package backend -- bounded_context::application::finish_task::tests --nocapture

# Test a single integration test
$ cargo test --package backend --test finish_task_controller_test -- tests --nocapture
```

## Tooling

This utility command can be used to print file contents for exploration:

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

## API Examples

Here are some example API requests and responses:

```
# Create Task Request
$ curl -X POST -H "Content-Type: application/json" -d '{"title": "Amazing task","description": "Description of an amazing task todo"}' http://localhost:8080/api/task
# Create Task Response
$ {"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"Todo"}

# Start Task Request
$ curl -X PUT -H "Content-Type: application/json" -d '{"id": "00000000-0000-0000-0000-000000000001"}' http://localhost:8080/api/start_task/00000000-0000-0000-0000-000000000001
# Start Task Response
$ {"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"InProgress"}

# Finish Task Request
$ curl -X PUT -H "Content-Type: application/json" -d '{"id": "00000000-0000-0000-0000-000000000001"}' http://localhost:8080/api/finish_task/00000000-0000-0000-0000-000000000001
# Finish Task Response
$ {"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"Done"}
```

## Future Improvements

Consider implementing the following enhancements:

- Assign a task to a user
- Handle errors in a more robust way
- Consider adding a security layer to protect your endpoints

Feel free to contribute and make this project better! Happy Coding!
