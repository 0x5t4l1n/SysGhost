# Troubleshooting

## "the dashboard needs an interactive terminal"

Stdin or stdout is not a terminal, for example when piping output or running from a script. Use `sysghost --json` for non-interactive use.

## "invalid value ... is not in 200..=60000"

The refresh interval must be between 200 and 60,000 ms.

## CPU shows a dash

CPU needs two samples. The first dashboard frame has none; values appear after one interval. In JSON, a process has `null` CPU only if it started after the first of the two internal samples.

## A process is missing

It may belong to another user and be hidden by `hidepid`, live in a different PID namespace, have exited during the scan, or be shorter-lived than the refresh interval. See [permissions-and-visibility.md](permissions-and-visibility.md).

## CPU is above 100%

Per-process CPU is relative to one core, so a multi-threaded process can exceed 100%. System CPU in the header never does.

## The terminal looks broken after a crash

Run `reset`. SysGhost restores the terminal on normal exit and on `SIGINT`, `SIGTERM` and `SIGHUP`, but not after `SIGKILL`.

## "Resize to 38 × 15 or larger"

The terminal is below the minimum size. Enlarge it, or press Q to quit.

## Build fails on a non-Linux system

The crate stops compilation on purpose: `SysGhost requires Linux`.
