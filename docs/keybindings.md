# Key bindings

| Key | Context | Action |
| --- | --- | --- |
| Up / Down | Processes panel | Move the selection |
| Up / Down | Events panel | Scroll toward older or newer events |
| Enter | Processes panel | Open details for the selected process |
| Enter | Search | Apply the query |
| `/` | Normal | Switch to Processes and start editing the filter |
| Tab, Shift+Tab | Normal | Switch between Processes and Events |
| Esc | Search | Cancel the edit and restore the previous filter |
| Esc | Details | Close details |
| Esc | Normal | Clear the filter |
| Backspace | Search | Delete the last character |
| Q | Normal, details | Quit |
| Ctrl+C | Anywhere | Quit |

## Details

- While editing a search, `q` is typed into the query, so use Ctrl+C to quit from there.
- The query accepts printable characters only and is capped at 256 bytes. Characters typed with Ctrl or Alt are ignored.
- Key release events are ignored, so terminals that report them do not trigger actions twice.
- While details are open, only Esc and quit keys are handled.
- Tab has no effect in the details view.

The footer shows the shortcuts that apply to the current mode, and shortens them below 70 columns.
