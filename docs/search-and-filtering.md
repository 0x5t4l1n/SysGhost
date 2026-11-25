# Search, sorting and selection

## Filtering

Press `/` and type. A process matches when either condition holds:

- its name contains the query, ignoring case
- its PID, written in decimal, contains the query as a substring

So `12` matches PIDs 12, 120 and 4123. For an exact process use `--pid`, which restricts sampling to one PID.

The filter is applied on every keystroke. Enter keeps it, Esc during editing restores the previous one, and Esc in normal mode clears it.

## Sorting

Rows are sorted by CPU, highest first. Processes with no CPU reading yet sort after those that have one. Ties are broken by PID ascending.

## Keeping the selection

On each refresh the app remembers the selected `ProcessId` and moves the cursor to the row where that process now sits, so the selection follows a process as it moves up or down the list. If the process is gone, the numeric position is kept and clamped to the list length.

Changing the query moves the selection back to the top.

## Details view

Enter copies the selected process into a details view that refreshes every sample. If the process disappears, the last known values stay visible with the note "Process no longer visible · last sample".
