# JSON output

`sysghost --json` takes two samples separated by `sysinfo::MINIMUM_CPU_UPDATE_INTERVAL`, so CPU figures can be calculated, then prints the second one as pretty-printed JSON followed by a newline.

```json
{
  "sampled_at_unix_ms": 1795626741843,
  "cpu_percent": 24.159021,
  "memory_used_bytes": 8489635840,
  "memory_total_bytes": 16374165504,
  "processes": [
    {
      "pid": 84036,
      "start_ticks": 1674331,
      "name": "zsh",
      "parent_pid": 57555,
      "status": "Sleeping",
      "cpu_percent": 0.0,
      "memory_bytes": 4530176
    }
  ]
}
```

## Snapshot fields

| Field | Type | Meaning |
| --- | --- | --- |
| `sampled_at_unix_ms` | integer | Wall-clock time of the sample, Unix milliseconds |
| `cpu_percent` | number or null | System-wide CPU usage |
| `memory_used_bytes` | integer | Used system memory |
| `memory_total_bytes` | integer | Total system memory |
| `processes` | array | Processes sorted by `pid` ascending |

## Process fields

| Field | Type | Meaning |
| --- | --- | --- |
| `pid` | integer | Process ID |
| `start_ticks` | integer | Kernel start time in clock ticks since boot. Together with `pid` it identifies a process. |
| `name` | string | Command name (`comm`), with control characters replaced by U+FFFD |
| `parent_pid` | integer or null | Parent PID; null when the kernel reports 0 |
| `status` | string | Running, Sleeping, Disk sleep, Zombie, Stopped, Tracing stop, Idle, Dead, or Unknown |
| `cpu_percent` | number or null | CPU use over the sampling interval; can exceed 100 on multi-core systems |
| `memory_bytes` | integer | Resident set size in bytes |

Command-line arguments are never read, so they are not part of the output.
