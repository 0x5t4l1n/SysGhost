# Testing

Run everything with `cargo test --locked`.

## Integration tests (`tests/monitor_tests.rs`)

| Test | Covers |
| --- | --- |
| `lifecycle_diff_ignores_renames_and_detects_same_second_pid_reuse` | `events::compare` with PID reuse, renames, additions and removals |
| `samples_live_process_and_system_resources` | A real two-sample run against the test process itself: CPU is `None` then finite, memory totals are sane, an unused PID gives an empty list |
| `observes_live_child_start_and_exit` | Spawns `sleep 30`, samples it, kills it, and checks the start and exit events share an identity |

These need a Linux `/proc`, and the child test needs a `sleep` binary.

## Unit tests

`src/monitor.rs` tests `parse_stat`, including names with parentheses and newlines and truncated input.

## Writing new tests

- Put pure logic in the library crate and test it through `tests/` or a `#[cfg(test)]` module.
- Build `Snapshot` values by hand (see the `process()` and `snapshot()` helpers) to test event logic without touching `/proc`.
- Avoid assertions on absolute CPU values; they depend on the machine. Assert ranges and `None` versus `Some` instead.
