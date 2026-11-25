# Dashboard layout

The screen is split vertically into five rows: header, resource gauges, status line, body and footer.

## Size thresholds

| Terminal size | Behavior |
| --- | --- |
| Under 38 columns or 15 rows | Only "Resize to 38 × 15 or larger. Q quits." is shown |
| 38 columns or more | Single panel; Tab switches between Processes and Events |
| 70 columns or more | Tagline in the header and the long shortcut list in the footer |
| 96 columns or more | Processes (60%) and Events (40%) side by side, with a "Selected process" box under the process table |

## Resource row

CPU and RAM gauges share the row (42% and 58%). The CPU label shows a dash until two samples exist.

## Status line

Shows the scope (`All PIDs` or `PID <n>`) and the refresh interval. During or after a search it shows `/query` instead.

## Process table

Columns: PID, NAME, CPU, RESIDENT. The selected row is marked with `>` and highlighted. The visible window scrolls so the selection always stays on screen.

## Redraws

The screen is redrawn after a new sample, a key press or a resize, not on every loop iteration.
