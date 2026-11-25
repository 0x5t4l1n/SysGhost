# Architecture

SysGhost is a library crate (`sysghost`) plus a binary. The library holds everything that can be tested without a terminal. The binary adds the command line and the dashboard.

| Module | Crate | Responsibility |
| --- | --- | --- |
| `models` | library | `Process`, `ProcessId` and `Snapshot` data types, serializable to JSON |
| `monitor` | library | `Monitor` samples system CPU and memory and reads `/proc/<pid>/stat` |
| `events` | library | `compare()` turns two snapshots into start and exit events |
| `cli` | binary | Argument definitions (`--interval`, `--pid`, `--json`) |
| `app` | binary | Dashboard state: selection, search, event history, key handling, main loop |
| `ui` | binary | Rendering with Ratatui |
| `main` | binary | Wires the pieces together, handles signals and terminal setup |

Both crates use `#![forbid(unsafe_code)]`, and the library refuses to compile on non-Linux targets.

## Data flow

```
/proc/<pid>/stat + sysinfo  ->  Monitor::sample()  ->  Snapshot
Snapshot (previous, current) ->  events::compare()  ->  Vec<ProcessEvent>
Snapshot + events + key input -> App                ->  ui::draw()
```

`--json` uses only the first line of that flow. The dashboard runs the whole loop at the chosen interval.
