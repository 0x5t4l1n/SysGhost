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

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("sysghost").chain(args.iter().copied()))
    }

    #[test]
    fn defaults_are_one_second_all_processes_dashboard() {
        let cli = parse(&[]).unwrap();
        assert_eq!(cli.interval, 1000);
        assert_eq!(cli.pid, None);
        assert!(!cli.json);
    }

    #[test]
    fn interval_is_limited_to_200_through_60000_ms() {
        assert!(parse(&["--interval", "199"]).is_err());
        assert_eq!(parse(&["--interval", "200"]).unwrap().interval, 200);
        assert_eq!(parse(&["--interval", "60000"]).unwrap().interval, 60_000);
        assert!(parse(&["--interval", "60001"]).is_err());
    }

    #[test]
    fn pid_must_be_positive() {
        assert!(parse(&["--pid", "0"]).is_err());
        assert_eq!(parse(&["--pid", "1"]).unwrap().pid, Some(1));
        assert!(parse(&["--pid", "-1"]).is_err());
    }

    #[test]
    fn json_flag_combines_with_pid() {
        let cli = parse(&["--json", "--pid", "42"]).unwrap();
        assert!(cli.json);
        assert_eq!(cli.pid, Some(42));
    }
}
