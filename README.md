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
$ cargo test --package backend -- bounded_context::application::finish_task::tests --nocapture
# Testing a single integration test
$ cargo test --package backend --test finish_task_controller_test -- tests --nocapture
```

# Tools

Print cat files to explore them together

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

# Sample Request/Response

```
# Create Task
$ curl -X POST -H "Content-Type: application/json" -d '{"title": "Amazing task","description": "Description of an amazing task todo"}' http://localhost:8080/api/task
# Response
{"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"Todo"}
#
# Start Task
$ curl -X PUT -H "Content-Type: application/json" -d '{"id": "00000000-0000-0000-0000-000000000001"}' http://localhost:8080/api/start_task/00000000-0000-0000-0000-000000000001
# Response
{"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"InProgress"}
#
# Finish Task
$ curl -X PUT -H "Content-Type: application/json" -d '{"id": "f2894443-ed1d-4010-9238-28075d077c1d"}' http://localhost:8080/api/finish_task/f2894443-ed1d-4010-9238-28075d077c1d
# Response
{"id":"00000000-0000-0000-0000-000000000001","title":"Amazing task","description":"Description of an amazing task todo","status":"Done"}
```



# Next possible steps

``` 
- adding more functionality to your application, such as the ability to complete a task
- assign a task to a user
- handle errors in a more robust way
- think about adding a security layer to protect your endpoints
```
