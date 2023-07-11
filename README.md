# Rust Backend

```
# Run the project
$ cargo run --bin process
$ cargo run --bin server
# Test the project
$ cargo test
# Run specific integration test
$ cargo test test_index_get
```

# Mysql

```
$ docker-compose up -d
$ docker exec -it db mysql -uroot -proot rust
$ docker exec -i db mysql -uroot -proot < src/bounded_context/infrastructure/mysql/task.sql
```

# Code coverage

To be able to execute the code coverage you can use this tool

```
$ cargo install cargo-tarpaulin
$ cargo tarpaulin --output-dir target/debug/tarpaulin/ --out html
$ open -a "Google Chrome" ./target/debug/tarpaulin/tarpaulin-report.html
# Testing a single file
$ cargo test --package hello_cargo --bin hello_cargo -- bounded_context::application::finish_task::tests --nocapture
```

# Tools

Print cat files to explore them together

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

# Sample Request/Response

```
# Request
$ curl -X POST -H "Content-Type: application/json" -d '{"title": "Amazing task","description": "Description of an amazing task todo"}' http://localhost:8080/api/tasks
# Response
{"id":"26ce4b34-b493-4ffc-a247-333c399371da","title":"Amazing task","description":"Description of an amazing task todo","status":"Fake status"}%
```

# Next possible steps

``` 
- adding more functionality to your application, such as the ability to complete a task
- assign a task to a user
- handle errors in a more robust way
- think about adding a security layer to protect your endpoints
```
