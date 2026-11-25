# `/proc/<pid>/stat` fields

`monitor::parse_stat` reads one line per process and keeps six values. Field numbers follow `proc(5)`.

| # | Field | Used for |
| --- | --- | --- |
| 1 | `pid` | Taken from the directory name, not the file |
| 2 | `comm` | Process name |
| 3 | `state` | Status label |
| 4 | `ppid` | Parent PID (0 becomes none) |
| 14 | `utime` | CPU ticks in user mode |
| 15 | `stime` | CPU ticks in kernel mode |
| 22 | `starttime` | `start_ticks`, part of the process identity |
| 24 | `rss` | Resident pages |

## Parsing the name

`comm` is wrapped in parentheses but can itself contain spaces, newlines and parentheses. The parser therefore splits at the **first** `(` and the **last** `)`, then reads the remaining fields as whitespace-separated numbers. A unit test covers the name `a)b (c\n`.

Control characters in the name become U+FFFD, and invalid UTF-8 is replaced lossily.

## State codes

| Code | Label |
| --- | --- |
| R | Running |
| S | Sleeping |
| D | Disk sleep |
| Z | Zombie |
| T | Stopped |
| t | Tracing stop |
| I | Idle |
| X, x | Dead |

Any other code is labelled Unknown. A line with missing or non-numeric fields is skipped, which is also what happens when a process exits between listing `/proc` and reading it.
