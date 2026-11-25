use std::collections::BTreeMap;

use crate::models::{Process, ProcessId, Snapshot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    Started,
    Exited,
}

pub struct ProcessEvent {
    pub time_ms: u64,
    pub kind: EventKind,
    pub id: ProcessId,
    pub name: String,
}

fn index(snapshot: &Snapshot) -> BTreeMap<ProcessId, &Process> {
    snapshot
        .processes
        .iter()
        .map(|process| (process.id(), process))
        .collect()
}

pub fn compare(previous: &Snapshot, current: &Snapshot) -> Vec<ProcessEvent> {
    let mut events = Vec::new();
    // Record exits first when a PID has been reused between samples.
    let before = index(previous);
    let after = index(current);
    for (source, target, kind) in [
        (&before, &after, EventKind::Exited),
        (&after, &before, EventKind::Started),
    ] {
        events.extend(
            source
                .iter()
                .filter(|(id, _)| !target.contains_key(id))
                .map(|(&id, process)| ProcessEvent {
                    time_ms: current.sampled_at_unix_ms,
                    kind,
                    id,
                    name: process.name.clone(),
                }),
        );
    }
    events
}
