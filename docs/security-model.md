# Security model

## Properties

- **Local only.** SysGhost opens no network connections and sends no telemetry.
- **Read-only.** It reads kernel statistics and never writes to `/proc` or signals processes.
- **No unsafe code.** Both crates declare `#![forbid(unsafe_code)]`.
- **Minimal collection.** Only the fields listed in [proc-stat-fields.md](proc-stat-fields.md) and system CPU and memory totals are collected.
- **Locked dependencies.** CI builds with `--locked`, so `Cargo.lock` decides which dependency versions are used.

## Untrusted input

Process names are chosen by whoever runs the process. The parser handles names containing parentheses, spaces and newlines, and replaces control characters with U+FFFD before display so a name cannot inject terminal escape sequences through ASCII control codes.

## Sensitive output

JSON snapshots list every visible process name and PID. Redact them before attaching to public bug reports, as the security policy asks.

## Reporting

See `SECURITY.md` for private vulnerability reporting.
