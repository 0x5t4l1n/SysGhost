#![forbid(unsafe_code)]

mod app;
mod cli;
mod ui;

use std::{
    io::{self, IsTerminal, Write},
    sync::{Arc, atomic::AtomicBool},
    thread,
    time::Duration,
};

use anyhow::{Context, Result, ensure};
use clap::Parser;
use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGTERM},
    flag,
};
use sysghost::monitor::Monitor;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    if args.json {
        let mut monitor = Monitor::new()?;
        monitor.sample(args.pid)?;
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let snapshot = monitor.sample(args.pid)?;
        let mut stdout = io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &snapshot)?;
        writeln!(stdout)?;
        return Ok(());
    }

    ensure!(
        io::stdin().is_terminal() && io::stdout().is_terminal(),
        "the dashboard needs an interactive terminal. Use --json to export a snapshot"
    );

    let stop = Arc::new(AtomicBool::new(false));
    for signal in [SIGINT, SIGTERM, SIGHUP] {
        flag::register(signal, Arc::clone(&stop)).context("registering shutdown handler")?;
    }
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(error) => {
            let _ = ratatui::try_restore();
            return Err(error).context("initializing dashboard");
        }
    };
    let result = app::run(
        &mut terminal,
        Duration::from_millis(args.interval),
        args.pid,
        stop,
    );
    let restored = ratatui::try_restore().context("restoring terminal");
    result.and(restored)
}
