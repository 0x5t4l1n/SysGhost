# Changelog

## Unreleased

- Labeled the Linux process states `K`, `W` and `P` as Wakekill, Waking and Parked instead of Unknown.

## 0.1.0

- Added a Linux terminal dashboard with live system CPU and memory usage, a searchable process list, and process details.
- Added timestamped process lifecycle events based on consecutive samples and PID/start-tick identity.
- Added a configurable polling interval, PID filtering, and JSON snapshot export.
