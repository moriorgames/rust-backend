# Rust Backend

```
# Run the project
$ cargo run
# Test the project
$ cargo test
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
