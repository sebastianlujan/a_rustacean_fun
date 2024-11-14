# Rust notes, a learning thing..

## Cargo watch
Cargo watch observe your code challenges, running Just in time compilation in real time, which increase productivity
reducing the perceived compilation

```sh
cargo install cargo-watch
cargo watch -x check

# -x means execute, as allways..
```
## To check the testing process we use this command

```sh
cargo watch -x check -x test -x run 

# this means, cargo watch && cargo check && cargo test && run
```

## CI Steps
The first CI step must be, testing, testing is a `first class concept in Rust`, 
```sh
cargo build
cargo test
```

## CodeCoverage::tarpaulin
```sh
cargo install cargo-tarpaulin
# for code coverage
```

## Linting::clippy
A way to hack the idiomatic coding and increase productivity for new people, we use linting, 
to ensure code safety and good taste code, and reduce cyclomatic complexity.
```sh
rustup component add clippy
```



