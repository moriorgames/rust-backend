# Rust Backend


```
# Run the project
$ cargo run
# Test the project
$ cargo test
```

# Code coverage

To be able to execute the code coverage you can use this tool

```
$ cargo install cargo-tarpaulin
$ cargo tarpaulin --output-dir target/debug/tarpaulin/ --out html
$ open -a "Google Chrome" ./target/debug/tarpaulin/tarpaulin-report.html
```

# Tools

Print cat files to explore them together

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

# Mysql

```
$ docker exec -it db mysql -uroot -proot rust
$ docker exec -i db mysql -uroot -proot < src/bounded_context/infrastructure/mysql/task.sql
```
