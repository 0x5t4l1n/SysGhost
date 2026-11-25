# Terminal lifecycle

## Startup

1. Arguments are parsed. With `--json` the program prints and exits before touching the terminal.
2. The dashboard checks that stdin and stdout are terminals.
3. Handlers for `SIGINT`, `SIGTERM` and `SIGHUP` are registered. They only set a shared stop flag.
4. `ratatui::try_init()` enters the alternate screen and raw mode. If it fails, `try_restore()` is still called before the error is returned.

## Main loop

Each iteration checks the stop flag, takes a sample if the interval has elapsed, redraws if needed, then waits for input for at most 100 ms (or until the next sample). The 100 ms cap is what lets a signal stop the program quickly.

## Shutdown

The loop ends on Q, Ctrl+C or any of the three signals. `main` then calls `ratatui::try_restore()` and combines the results: an error from the dashboard is reported first, and a restore failure is reported if the dashboard succeeded.

The terminal is therefore restored on normal exit, on signals and on errors returned from the loop. It cannot be restored after `SIGKILL`; run `reset` if that happens.
