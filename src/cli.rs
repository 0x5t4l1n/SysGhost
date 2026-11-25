use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Refresh interval in milliseconds
    #[arg(long, default_value_t = 1000, value_parser = clap::value_parser!(u64).range(200..=60_000))]
    pub interval: u64,

    /// Show only this process ID
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    pub pid: Option<u32>,

    /// Write one JSON snapshot to stdout
    #[arg(long)]
    pub json: bool,
}
