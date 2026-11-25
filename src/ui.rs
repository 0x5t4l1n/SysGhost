use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Gauge, Paragraph, Row, Table, Wrap},
};
use sysghost::{events::EventKind, models::Process};

use crate::app::{App, Panel};

const BG: Color = Color::Rgb(10, 17, 23);
const FG: Color = Color::Rgb(223, 234, 239);
const MUTED: Color = Color::Rgb(149, 169, 181);
const BORDER: Color = Color::Rgb(89, 112, 128);
const CYAN: Color = Color::Rgb(96, 224, 236);
const GREEN: Color = Color::Rgb(132, 229, 161);
const SELECTION: Color = Color::Rgb(30, 56, 68);

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(Block::new().style(Style::new().bg(BG).fg(FG)), area);
    if area.width < 38 || area.height < 15 {
        frame.render_widget(
            Paragraph::new("Resize to 38 × 15 or larger. Q quits.").wrap(Wrap { trim: true }),
            area,
        );
        return;
    }
    let [header, gauges, info, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);
    let identity = if area.width >= 70 {
        "  Observe. Trace. Understand."
    } else {
        ""
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                " [>] SysGhost",
                Style::new().fg(CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(identity, Style::new().fg(MUTED)),
        ])),
        header,
    );
    resources(frame, gauges, app);
    let scope = app
        .pid_filter
        .map_or_else(|| "All PIDs".to_owned(), |pid| format!("PID {pid}"));
    let status = if app.searching() || !app.query.is_empty() {
        format!(
            " /{}{}  ·  {scope}",
            app.query,
            if app.searching() { "_" } else { "" }
        )
    } else {
        format!(" {scope}  ·  {} ms", app.interval.as_millis())
    };
    frame.render_widget(Paragraph::new(status).style(Style::new().fg(MUTED)), info);

    if let Some(process) = &app.detail {
        detail(frame, body, process, app.detail_exited);
    } else if area.width >= 96 {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
                .spacing(1)
                .areas(body);
        let [proc_area, selected_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(left);
        processes(frame, proc_area, app);
        frame.render_widget(
            Paragraph::new(app.selected().map_or_else(
                || "No process selected".to_owned(),
                |process| format!("{}  {}  ·  {}", process.pid, process.name, process.status),
            ))
            .block(panel(" Selected process ", false)),
            selected_area,
        );
        events(frame, right, app);
    } else if app.panel == Panel::Processes {
        processes(frame, body, app);
    } else {
        events(frame, body, app);
    }

    let shortcuts = if app.searching() {
        " Enter apply · Esc cancel · Ctrl+C quit"
    } else if app.detail.is_some() {
        " Esc back · Q quit"
    } else if area.width >= 70 {
        " ↑↓ Navigate   Enter Details   / Search   Tab Panels   Esc Clear   Q Quit"
    } else {
        " ↑↓  Enter  /  Tab  Esc  Q"
    };
    frame.render_widget(
        Paragraph::new(shortcuts).style(Style::new().fg(CYAN)),
        footer,
    );
}

fn panel(title: impl Into<Line<'static>>, focused: bool) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(if focused { CYAN } else { BORDER }))
        .title_style(Style::new().fg(if focused { CYAN } else { FG }))
        .title(title)
}

fn resources(frame: &mut Frame, area: Rect, app: &App) {
    let [cpu, ram] = Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
        .spacing(1)
        .areas(area);
    let used = app.snapshot.memory_used_bytes;
    let total = app.snapshot.memory_total_bytes;
    let memory_ratio = if total == 0 {
        0.0
    } else {
        used as f64 / total as f64
    };
    let cpu_percent = app.snapshot.cpu_percent;
    let cpu_label = format!("CPU {}", percent(cpu_percent));
    let ram_label = if area.width >= 60 {
        format!("RAM {} / {}", bytes(used), bytes(total))
    } else {
        format!("RAM {:.0}%", memory_ratio * 100.0)
    };
    for (bounds, label, ratio, color) in [
        (
            cpu,
            cpu_label,
            f64::from(cpu_percent.unwrap_or_default()) / 100.0,
            CYAN,
        ),
        (ram, ram_label, memory_ratio, GREEN),
    ] {
        frame.render_widget(
            Gauge::default()
                .gauge_style(Style::new().fg(color).bg(SELECTION))
                .ratio(ratio.clamp(0.0, 1.0))
                .label(Span::styled(label, Style::new().fg(FG).bg(BG))),
            bounds,
        );
    }
}

fn processes(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = panel(
        format!(" Processes · {} ", app.rows.len()),
        app.panel == Panel::Processes,
    );
    if app.rows.is_empty() {
        frame.render_widget(Paragraph::new(" No matching processes").block(block), area);
        return;
    }

    let visible = usize::from(area.height.saturating_sub(3)).max(1);
    let earliest = app.selection.saturating_sub(visible - 1);
    let latest = app.selection.min(app.rows.len().saturating_sub(visible));
    app.table_offset = app.table_offset.clamp(earliest, latest);
    let rows = app
        .rows
        .iter()
        .enumerate()
        .skip(app.table_offset)
        .take(visible)
        .map(|(index, &row)| {
            let process = &app.snapshot.processes[row];
            let selected = index == app.selection;
            let cells = [
                format!("{}{}", if selected { ">" } else { " " }, process.pid),
                process.name.clone(),
                percent(process.cpu_percent),
                bytes(process.memory_bytes),
            ];
            Row::new(cells).style(if selected {
                Style::new()
                    .fg(FG)
                    .bg(SELECTION)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(FG)
            })
        });
    let widths = [
        Constraint::Length(9),
        Constraint::Min(5),
        Constraint::Length(8),
        Constraint::Length(9),
    ];
    frame.render_widget(
        Table::new(rows, widths)
            .header(Row::new([" PID", "NAME", "CPU", "RESIDENT"]).style(Style::new().fg(MUTED)))
            .block(block),
        area,
    );
}

fn events(frame: &mut Frame, area: Rect, app: &App) {
    let visible = usize::from(area.height.saturating_sub(2));
    let title = if app.events.is_empty() {
        " Events · UTC ".to_owned()
    } else {
        format!(
            " Events · UTC · {}/{} ",
            app.event_offset + 1,
            app.events.len()
        )
    };
    let mut lines = Vec::new();
    for event in app.events.iter().skip(app.event_offset).take(visible) {
        let seconds = event.time_ms / 1000;
        let time = format!(
            "{:02}:{:02}:{:02} ",
            seconds / 3600 % 24,
            seconds / 60 % 60,
            seconds % 60
        );
        let (label, color) = match event.kind {
            EventKind::Started => ("START", GREEN),
            EventKind::Exited => (" EXIT", MUTED),
        };
        lines.push(Line::from(vec![
            Span::styled(time, Style::new().fg(MUTED)),
            Span::styled(label, Style::new().fg(color)),
            Span::raw(format!(" {} {}", event.id.pid, event.name)),
        ]));
    }
    if app.events.is_empty() {
        lines.push(Line::styled(
            "Waiting for process changes",
            Style::new().fg(MUTED),
        ));
    }
    frame.render_widget(
        Paragraph::new(lines).block(panel(title, app.panel == Panel::Events)),
        area,
    );
}

fn detail(frame: &mut Frame, area: Rect, process: &Process, exited: bool) {
    let parent = process
        .parent_pid
        .map_or_else(|| "—".to_owned(), |pid| pid.to_string());
    let mut lines = vec![
        Line::styled(
            process.name.as_str(),
            Style::new().fg(GREEN).add_modifier(Modifier::BOLD),
        ),
        Line::raw(format!("PID           {}", process.pid)),
        Line::raw(format!("Parent PID    {parent}")),
        Line::raw(format!("State         {}", process.status)),
        Line::raw(format!("CPU           {}", percent(process.cpu_percent))),
        Line::raw(format!(
            "Resident      {} ({} B)",
            bytes(process.memory_bytes),
            process.memory_bytes
        )),
        Line::raw(format!("Start ticks   {}", process.start_ticks)),
    ];
    if exited {
        lines.push(Line::styled(
            "Process no longer visible · last sample",
            Style::new().fg(CYAN),
        ));
    }
    frame.render_widget(
        Paragraph::new(lines)
            .block(panel(" Process details ", true))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn percent(value: Option<f32>) -> String {
    value.map_or_else(|| "—".to_owned(), |value| format!("{value:.1}%"))
}

fn bytes(value: u64) -> String {
    let mut size = value as f64;
    for unit in ["B", "KiB", "MiB", "GiB", "TiB"] {
        if size < 1024.0 || unit == "TiB" {
            return if unit == "B" {
                format!("{value} B")
            } else {
                format!("{size:.1} {unit}")
            };
        }
        size /= 1024.0;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{bytes, percent};

    #[test]
    fn bytes_uses_binary_units() {
        assert_eq!(bytes(0), "0 B");
        assert_eq!(bytes(1023), "1023 B");
        assert_eq!(bytes(1024), "1.0 KiB");
        assert_eq!(bytes(1536), "1.5 KiB");
        assert_eq!(bytes(1 << 20), "1.0 MiB");
        assert_eq!(bytes(1 << 30), "1.0 GiB");
        assert_eq!(bytes(1 << 40), "1.0 TiB");
    }

    #[test]
    fn bytes_stops_at_tebibytes() {
        assert_eq!(bytes(2048 << 40), "2048.0 TiB");
    }

    #[test]
    fn percent_formats_one_decimal_or_dash() {
        assert_eq!(percent(None), "—");
        assert_eq!(percent(Some(0.0)), "0.0%");
        assert_eq!(percent(Some(12.34)), "12.3%");
        assert_eq!(percent(Some(250.0)), "250.0%");
    }
}
