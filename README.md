**This project is not to be copied and/or distributed without the express permission of Alexander Nathanail. See [LICENSE.md](./LICENSE.md)**

# Broute

A travelling salesman solver for DRT applications.

## To run test coverage

[//]: # (https://blog.rng0.io/how-to-do-code-coverage-in-rust)

```shell
rm -rf target/coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/data/cargo-test-%p-%m.profraw' cargo test
grcov ./target/coverage/data --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "**/target/**" --ignore "**/src/bin/**" -o target/coverage/html
```
