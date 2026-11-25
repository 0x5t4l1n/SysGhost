# Development

## Requirements

Linux, a stable Rust toolchain with rustfmt and Clippy, and a C linker.

## Common commands

```bash
cargo build --locked            # debug build
cargo run --locked              # dashboard (needs a real terminal)
cargo run --locked -- --json    # snapshot, works anywhere
cargo test --locked
```

## Before opening a pull request

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

These are the same checks as `.github/workflows/ci.yml`, which runs them on `ubuntu-24.04`.

## Where to change things

| Goal | File |
| --- | --- |
| Collect a new `/proc` field | `src/monitor.rs`, `src/models.rs` |
| Change what counts as a process event | `src/events.rs` |
| Add a key binding | `src/app.rs` (`handle_key`) and the footer text in `src/ui.rs` |
| Change layout or colours | `src/ui.rs` |
| Add a flag | `src/cli.rs`, then `src/main.rs` |

Logic that does not need a terminal belongs in the library crate so it can be tested from `tests/`.

## Manual checks

For UI changes try a wide terminal (96+ columns) and a compact one. For monitoring changes, start and kill short-lived processes and watch for PID reuse.
