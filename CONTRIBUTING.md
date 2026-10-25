# Contributing

Open an issue before large API changes. Keep changes focused, with tests and updated knowledge docs. Run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run --example consume
```

Do not submit secrets, proprietary application logs, or unauthorised test targets. Every security-relevant PR needs rationale and regression tests. Reviews are requested from CODEOWNERS.
