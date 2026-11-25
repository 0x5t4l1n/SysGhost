use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProcessId {
    pub pid: u32,
    pub start_ticks: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Process {
    pub pid: u32,
    pub start_ticks: u64,
    pub name: String,
    pub parent_pid: Option<u32>,
    pub status: String,
    pub cpu_percent: Option<f32>,
    pub memory_bytes: u64,
}

impl Process {
    pub fn id(&self) -> ProcessId {
        ProcessId {
            pid: self.pid,
            start_ticks: self.start_ticks,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Snapshot {
    pub sampled_at_unix_ms: u64,
    pub cpu_percent: Option<f32>,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub processes: Vec<Process>,
}
