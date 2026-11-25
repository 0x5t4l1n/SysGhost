use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;
use sysghost::{
    events::{self, ProcessEvent},
    models::{Process, Snapshot},
    monitor::Monitor,
};

use crate::ui;

const EVENT_LIMIT: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Processes,
    Events,
}

pub struct App {
    pub snapshot: Snapshot,
    pub rows: Vec<usize>,
    pub selection: usize,
    pub table_offset: usize,
    pub events: VecDeque<ProcessEvent>,
    pub event_offset: usize,
    pub panel: Panel,
    pub query: String,
    pub detail: Option<Process>,
    pub detail_exited: bool,
    pub interval: Duration,
    pub pid_filter: Option<u32>,
    previous_query: Option<String>,
}

impl App {
    fn new(snapshot: Snapshot, interval: Duration, pid_filter: Option<u32>) -> Self {
        let mut app = Self {
            snapshot,
            rows: Vec::new(),
            selection: 0,
            table_offset: 0,
            events: VecDeque::new(),
            event_offset: 0,
            panel: Panel::Processes,
            query: String::new(),
            detail: None,
            detail_exited: false,
            interval,
            pid_filter,
            previous_query: None,
        };
        app.rebuild_rows();
        app
    }

    pub fn searching(&self) -> bool {
        self.previous_query.is_some()
    }

    pub fn selected(&self) -> Option<&Process> {
        self.rows
            .get(self.selection)
            .map(|&row| &self.snapshot.processes[row])
    }

    fn update(&mut self, snapshot: Snapshot) {
        let selected = self.selected().map(Process::id);
        let events = events::compare(&self.snapshot, &snapshot);
        if self.event_offset > 0 {
            self.event_offset += events.len();
        }
        for event in events {
            self.events.push_front(event);
        }
        self.events.truncate(EVENT_LIMIT);
        self.event_offset = self.event_offset.min(self.events.len().saturating_sub(1));
        if let Some(detail) = &mut self.detail {
            let current = snapshot
                .processes
                .iter()
                .find(|process| process.id() == detail.id());
            self.detail_exited = current.is_none();
            if let Some(current) = current {
                *detail = current.clone();
            }
        }

        self.snapshot = snapshot;
        self.rebuild_rows();
        if let Some(index) = self
            .rows
            .iter()
            .position(|&row| Some(self.snapshot.processes[row].id()) == selected)
        {
            self.selection = index;
        }
    }

    fn rebuild_rows(&mut self) {
        let query = self.query.to_lowercase();
        self.rows.clear();
        self.rows
            .extend(
                self.snapshot
                    .processes
                    .iter()
                    .enumerate()
                    .filter_map(|(row, process)| {
                        (query.is_empty()
                            || process.name.to_lowercase().contains(&query)
                            || process.pid.to_string().contains(&query))
                        .then_some(row)
                    }),
            );
        self.rows.sort_unstable_by(|&left, &right| {
            let left = &self.snapshot.processes[left];
            let right = &self.snapshot.processes[right];
            right
                .cpu_percent
                .partial_cmp(&left.cpu_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.pid.cmp(&right.pid))
        });
        self.selection = self.selection.min(self.rows.len().saturating_sub(1));
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }

        if self.searching() {
            match key.code {
                KeyCode::Esc => self.query = self.previous_query.take().unwrap_or_default(),
                KeyCode::Enter => self.previous_query = None,
                KeyCode::Backspace => {
                    self.query.pop();
                }
                KeyCode::Char(ch)
                    if !key
                        .modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                        && !ch.is_control()
                        && self.query.len() < 256 =>
                {
                    self.query.push(ch)
                }
                _ => return false,
            }
            self.selection = 0;
            self.rebuild_rows();
            return false;
        }

        if matches!(key.code, KeyCode::Char('q' | 'Q')) {
            return true;
        }

        if self.detail.is_some() {
            if key.code == KeyCode::Esc {
                self.detail = None;
            }
            return false;
        }

        match key.code {
            KeyCode::Tab | KeyCode::BackTab => {
                self.panel = match self.panel {
                    Panel::Processes => Panel::Events,
                    Panel::Events => Panel::Processes,
                };
            }
            KeyCode::Char('/') => {
                self.panel = Panel::Processes;
                self.previous_query = Some(self.query.clone());
            }
            KeyCode::Esc => {
                if !self.query.is_empty() {
                    self.query.clear();
                    self.selection = 0;
                    self.rebuild_rows();
                }
            }
            KeyCode::Enter if self.panel == Panel::Processes => {
                self.detail = self.selected().cloned();
                self.detail_exited = false;
            }
            KeyCode::Up | KeyCode::Down => {
                let (position, length) = match self.panel {
                    Panel::Processes => (&mut self.selection, self.rows.len()),
                    Panel::Events => (&mut self.event_offset, self.events.len()),
                };
                *position = if key.code == KeyCode::Up {
                    position.saturating_sub(1)
                } else {
                    (*position + 1).min(length.saturating_sub(1))
                };
            }
            _ => {}
        }
        false
    }
}

pub fn run(
    terminal: &mut DefaultTerminal,
    interval: Duration,
    pid: Option<u32>,
    stop: Arc<AtomicBool>,
) -> Result<()> {
    let mut monitor = Monitor::new()?;
    let mut next_sample = Instant::now() + interval;
    let mut app = App::new(monitor.sample(pid)?, interval, pid);
    let mut redraw = true;

    while !stop.load(Ordering::Relaxed) {
        let now = Instant::now();
        if now >= next_sample {
            next_sample = now + interval;
            app.update(monitor.sample(pid)?);
            redraw = true;
        }

        if redraw {
            terminal.draw(|frame| ui::draw(frame, &mut app))?;
            redraw = false;
        }

        let wait = next_sample.saturating_duration_since(Instant::now());
        if !event::poll(wait.min(Duration::from_millis(100)))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) if key.kind != KeyEventKind::Release => {
                if app.handle_key(key) {
                    break;
                }
                redraw = true;
            }
            Event::Resize(..) => redraw = true,
            _ => {}
        }
    }
    Ok(())
}
