use std::{process::Command, thread};
use sysghost::{
    events::{EventKind, compare},
    models::{Process, Snapshot},
    monitor::Monitor,
};

fn process(pid: u32, start_ticks: u64, name: &str) -> Process {
    Process {
        pid,
        start_ticks,
        name: name.to_owned(),
        parent_pid: None,
        status: "Sleeping".to_owned(),
        cpu_percent: None,
        memory_bytes: 0,
    }
}

fn snapshot(time: u64, processes: Vec<Process>) -> Snapshot {
    Snapshot {
        sampled_at_unix_ms: time,
        cpu_percent: None,
        memory_used_bytes: 0,
        memory_total_bytes: 0,
        processes,
    }
}

#[test]
fn lifecycle_diff_ignores_renames_and_detects_same_second_pid_reuse() {
    let previous = snapshot(
        100,
        vec![
            process(10, 101, "old"),
            process(20, 102, "before rename"),
            process(30, 103, "removed"),
        ],
    );
    let current = snapshot(
        200,
        vec![
            process(10, 104, "replacement"),
            process(20, 102, "after rename"),
            process(40, 105, "added"),
        ],
    );
    let changes = compare(&previous, &current);
    let observed: Vec<_> = changes
        .iter()
        .map(|event| {
            (
                event.kind,
                event.id.pid,
                event.id.start_ticks,
                event.name.as_str(),
                event.time_ms,
            )
        })
        .collect();
    assert_eq!(
        observed,
        [
            (EventKind::Exited, 10, 101, "old", 200),
            (EventKind::Exited, 30, 103, "removed", 200),
            (EventKind::Started, 10, 104, "replacement", 200),
            (EventKind::Started, 40, 105, "added", 200),
        ]
    );
    assert!(compare(&current, &current).is_empty());
}

#[test]
fn samples_live_process_and_system_resources() {
    let mut monitor = Monitor::new().unwrap();
    let pid = std::process::id();
    let first = monitor.sample(Some(pid)).unwrap();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let second = monitor.sample(Some(pid)).unwrap();

    assert_eq!(first.processes.len(), 1);
    assert_eq!(second.processes.len(), 1);
    let process = &second.processes[0];
    assert_eq!(process.pid, pid);
    assert_eq!(process.id(), first.processes[0].id());
    assert!(first.cpu_percent.is_none());
    assert!(first.processes[0].cpu_percent.is_none());
    assert!(
        process
            .cpu_percent
            .is_some_and(|cpu| cpu.is_finite() && cpu >= 0.0)
    );
    assert!(
        second
            .cpu_percent
            .is_some_and(|cpu| (0.0..=100.0).contains(&cpu))
    );
    assert!(second.memory_total_bytes > 0);
    assert!(second.memory_used_bytes <= second.memory_total_bytes);
    assert!(monitor.sample(Some(u32::MAX)).unwrap().processes.is_empty());
}

#[test]
fn observes_live_child_start_and_exit() {
    let mut monitor = Monitor::new().unwrap();
    let baseline = snapshot(0, vec![]);
    let mut child = Command::new("sleep").arg("30").spawn().unwrap();
    let pid = child.id();
    let live = monitor.sample(Some(pid));
    let _ = child.kill();
    child.wait().unwrap();
    let live = live.unwrap();
    let gone = monitor.sample(Some(pid)).unwrap();

    let started = compare(&baseline, &live);
    let exited = compare(&live, &gone);
    assert_eq!(started.len(), 1);
    assert_eq!(started[0].kind, EventKind::Started);
    assert_eq!(exited.len(), 1);
    assert_eq!(exited[0].kind, EventKind::Exited);
    assert_eq!(started[0].id, exited[0].id);
}
