**This project is not to be copied and/or distributed without the express permission of Alexander Nathanail. See [LICENSE.md](./LICENSE.md)**

# Broute

A travelling salesman solver for DRT applications.

```shell
# Run tests with coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/data/cargo-test-%p-%m.profraw' cargo test
grcov ./target/coverage/data --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "**/target/**" -o target/coverage/html

grcov ./target/coverage/data --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "**/target/**" -o target/coverage/tests.lcov
```
