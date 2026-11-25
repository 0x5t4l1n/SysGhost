# Process identity and lifecycle events

## Identity

A PID alone is not a stable identity because the kernel reuses PIDs. SysGhost identifies a process by `ProcessId { pid, start_ticks }`, where `start_ticks` is the kernel start time from `/proc/<pid>/stat`.

Two consequences:

- If PID 10 exits and a new process takes PID 10 between two samples, the start ticks differ and it is reported as one exit and one start.
- Renaming a process (`prctl(PR_SET_NAME)`) does not change its identity and creates no event.

## Events

`events::compare(previous, current)` builds an index of each snapshot by `ProcessId` and returns:

- `Exited` for identities present only in the previous snapshot
- `Started` for identities present only in the current snapshot

Exits are listed before starts. Every event carries the current snapshot's `sampled_at_unix_ms`, so the time is when SysGhost noticed the change, not when it happened.

## Limits of polling

A process that starts and exits between two samples is never seen. A shorter `--interval` reduces but cannot remove this.

## In the dashboard

The Events panel is newest first, shows times as UTC `HH:MM:SS`, and keeps at most 512 events per session.
