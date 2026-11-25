# CPU calculation

SysGhost reports two kinds of CPU figure.

## System CPU

`Snapshot::cpu_percent` comes from `sysinfo` (`refresh_cpu_usage()` followed by `global_cpu_usage()`). It is a percentage of total capacity, so it stays between 0 and 100. It is `None` on the first sample because sysinfo needs two readings.

## Per-process CPU

Each sample reads `utime` and `stime` from `/proc/<pid>/stat` and adds them into a tick counter. For a process seen in the previous sample:

```
cpu_percent = 100 * (ticks_now - ticks_before) / (elapsed_seconds * clock_ticks_per_second)
```

- `elapsed_seconds` is measured with a monotonic `Instant`, not the requested interval, so a slow sample does not skew the result.
- `clock_ticks_per_second` comes from `rustix::param::clock_ticks_per_second()`.
- The value is 100 per fully used logical CPU. A process running on four cores can show up to 400.
- It is `None` for a process on its first observation, when elapsed time is zero, or when the tick counter went backwards.

Counters are keyed by `ProcessId` (PID plus start ticks), so a reused PID never inherits the previous process's counter.
