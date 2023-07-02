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
