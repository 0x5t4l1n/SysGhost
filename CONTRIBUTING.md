# Contributing

Develop on Linux with a stable Rust toolchain, Cargo, rustfmt, and Clippy. Build with `cargo build --locked` and use `cargo run --locked` in an interactive terminal.

Keep changes focused on polling, observability, and terminal usability. For proposals that change what SysGhost collects, describe the use case and the collection and permission requirements.

Run these checks before opening a pull request.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

For UI changes, exercise wide and compact terminals. For monitoring changes, review lifecycle behavior with short-lived processes and PID reuse. In bug reports, include the SysGhost version, Linux distribution, relevant command, and steps to reproduce the issue.
