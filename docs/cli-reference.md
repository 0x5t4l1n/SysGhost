# Command line reference

```
sysghost [OPTIONS]
```

| Option | Default | Description |
| --- | --- | --- |
| `--interval <MS>` | `1000` | Dashboard refresh interval in milliseconds. Accepted range: 200 to 60,000. |
| `--pid <PID>` | all processes | Show only this process. Must be 1 or greater. |
| `--json` | off | Print one JSON snapshot to stdout and exit. |
| `--help`, `--version` | | Standard clap output. |

## Examples

```bash
sysghost                      # dashboard, all processes
sysghost --interval 500       # refresh twice a second
sysghost --pid 1234           # dashboard for one process
sysghost --json > snap.json   # machine-readable snapshot
sysghost --pid 1234 --json    # snapshot of one process
```

## Validation

Out-of-range values are rejected by clap before anything runs:

```
error: invalid value '100' for '--interval <INTERVAL>': 100 is not in 200..=60000
```

The 200 ms floor matches `sysinfo::MINIMUM_CPU_UPDATE_INTERVAL`, the shortest period over which sysinfo can report CPU usage.

## Interactive terminal requirement

Without `--json`, both stdin and stdout must be terminals. Otherwise SysGhost exits with `the dashboard needs an interactive terminal. Use --json to export a snapshot`.

If `--pid` names a process that does not exist, the dashboard shows an empty table and `--json` prints an empty `processes` array; neither is an error.
