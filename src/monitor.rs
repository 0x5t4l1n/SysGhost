use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use sysinfo::System;

use crate::models::{Process, ProcessId, Snapshot};

pub struct Monitor {
    system: System,
    previous: HashMap<ProcessId, u64>,
    sampled_at: Option<Instant>,
    clock_ticks: f64,
    page_size: u64,
    stat: Vec<u8>,
}

impl Monitor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            system: System::new().context("initializing system statistics")?,
            previous: HashMap::new(),
            sampled_at: None,
            clock_ticks: rustix::param::clock_ticks_per_second() as f64,
            page_size: rustix::param::page_size() as u64,
            stat: Vec::with_capacity(1024),
        })
    }

    pub fn sample(&mut self, pid_filter: Option<u32>) -> Result<Snapshot> {
        let now = Instant::now();
        let sampled_at_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let elapsed = self.sampled_at.map(|time| (now - time).as_secs_f64());
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        let mut processes = Vec::new();
        let mut counters = HashMap::with_capacity(self.previous.len());
        let mut read = |pid| {
            if let Some((mut process, cpu_ticks)) = self.read_process(pid) {
                let id = process.id();
                process.cpu_percent = self
                    .previous
                    .get(&id)
                    .zip(elapsed)
                    .filter(|(_, seconds)| *seconds > 0.0)
                    .and_then(|(&previous, seconds)| {
                        let delta = cpu_ticks.checked_sub(previous)?;
                        Some((100.0 * delta as f64 / (seconds * self.clock_ticks)) as f32)
                    });
                counters.insert(id, cpu_ticks);
                processes.push(process);
            }
        };

        if let Some(pid) = pid_filter {
            read(pid);
        } else {
            for entry in fs::read_dir("/proc").context("reading /proc")?.flatten() {
                if let Some(pid) = entry
                    .file_name()
                    .to_str()
                    .and_then(|name| name.parse().ok())
                {
                    read(pid);
                }
            }
        }
        processes.sort_unstable_by_key(|process| process.pid);
        self.previous = counters;
        self.sampled_at = Some(now);

        Ok(Snapshot {
            sampled_at_unix_ms,
            cpu_percent: elapsed.map(|_| self.system.global_cpu_usage()),
            memory_used_bytes: self.system.used_memory(),
            memory_total_bytes: self.system.total_memory(),
            processes,
        })
    }

    fn read_process(&mut self, pid: u32) -> Option<(Process, u64)> {
        self.stat.clear();
        File::open(format!("/proc/{pid}/stat"))
            .ok()?
            .read_to_end(&mut self.stat)
            .ok()?;
        parse_stat(pid, &String::from_utf8_lossy(&self.stat), self.page_size)
    }
}

fn parse_stat(pid: u32, stat: &str, page_size: u64) -> Option<(Process, u64)> {
    // The command name can contain spaces, newlines, and parentheses.
    let (_, data) = stat.split_once('(')?;
    let (name, fields) = data.rsplit_once(')')?;
    let mut fields = fields.split_ascii_whitespace();
    let status = match fields.next()? {
        "R" => "Running",
        "S" => "Sleeping",
        "D" => "Disk sleep",
        "Z" => "Zombie",
        "T" => "Stopped",
        "t" => "Tracing stop",
        "I" => "Idle",
        "X" | "x" => "Dead",
        "K" => "Wakekill",
        "W" => "Waking",
        "P" => "Parked",
        _ => "Unknown",
    };
    let parent_pid = fields.next()?.parse::<u32>().ok().filter(|&pid| pid != 0);
    let user_ticks = fields.nth(9)?.parse::<u64>().ok()?;
    let kernel_ticks = fields.next()?.parse::<u64>().ok()?;
    let start_ticks = fields.nth(6)?.parse().ok()?;
    let memory_pages = fields.nth(1)?.parse::<i64>().ok()?.max(0) as u64;
    Some((
        Process {
            pid,
            start_ticks,
            name: name
                .chars()
                .map(|ch| if ch.is_control() { '�' } else { ch })
                .collect(),
            parent_pid,
            status: status.to_owned(),
            cpu_percent: None,
            memory_bytes: memory_pages.saturating_mul(page_size),
        },
        user_ticks.saturating_add(kernel_ticks),
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_stat;

    #[test]
    fn stat_handles_delimiters_in_process_names() {
        let stat = "42 (a)b (c\n) S 7 0 0 0 0 0 0 0 0 0 14 15 0 0 0 0 0 0 22 0 24";
        let (process, cpu_ticks) = parse_stat(42, stat, 4096).unwrap();
        assert_eq!(process.name, "a)b (c�");
        assert_eq!(process.parent_pid, Some(7));
        assert_eq!(process.start_ticks, 22);
        assert_eq!(process.memory_bytes, 24 * 4096);
        assert_eq!(cpu_ticks, 29);
        assert!(parse_stat(42, "42 (incomplete) S 7", 4096).is_none());
    }

    #[test]
    fn stat_labels_every_documented_state() {
        let states = [
            ("R", "Running"),
            ("S", "Sleeping"),
            ("D", "Disk sleep"),
            ("Z", "Zombie"),
            ("T", "Stopped"),
            ("t", "Tracing stop"),
            ("I", "Idle"),
            ("X", "Dead"),
            ("x", "Dead"),
            ("K", "Wakekill"),
            ("W", "Waking"),
            ("P", "Parked"),
            ("?", "Unknown"),
        ];
        for (code, label) in states {
            let stat = format!("42 (x) {code} 7 0 0 0 0 0 0 0 0 0 14 15 0 0 0 0 0 0 22 0 24");
            let (process, _) = parse_stat(42, &stat, 4096).unwrap();
            assert_eq!(process.status, label, "state code {code}");
        }
    }

    #[test]
    fn stat_handles_unusual_field_values() {
        let line = |parent: &str, rss: &str| {
            format!("42 (x) S {parent} 0 0 0 0 0 0 0 0 0 14 15 0 0 0 0 0 0 22 0 {rss}")
        };
        let (process, _) = parse_stat(42, &line("0", "-5"), 4096).unwrap();
        assert_eq!(process.parent_pid, None);
        assert_eq!(process.memory_bytes, 0);
        let (process, _) = parse_stat(42, &line("1", &i64::MAX.to_string()), 4096).unwrap();
        assert_eq!(process.memory_bytes, u64::MAX);
        assert!(parse_stat(42, "42 x S 7", 4096).is_none());
        assert!(parse_stat(42, &line("7", "abc"), 4096).is_none());
    }
}
