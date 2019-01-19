use super::ConfigParser;
use api::commands;
use api::notification::LogLevel;
use clap::ArgMatches;
use std::path::PathBuf;

impl ConfigParser for commands::Config {
    fn parse(matches: &ArgMatches, dry_run: bool) -> Option<commands::Config> {
        let mut cfg = commands::Config::new();

        cfg.dry_run = dry_run;

        if let Some(exclude_dirs) = matches.values_of("exclude-dirs") {
            cfg.exclude_dirs = exclude_dirs.into_iter().map(PathBuf::from).collect();
        }

        if let Some(marker_name) = matches.value_of("marker-name") {
            cfg.marker_name = marker_name.to_owned();
        }

        cfg.log_level = match matches.occurrences_of("verbose") {
            0 => LogLevel::Error,
            1 => LogLevel::Warn,
            2 => LogLevel::Info,
            3 => LogLevel::Debug,
            _ => LogLevel::Trace,
        };

        cfg.dereference_symlinks = matches.is_present("dereference-symlinks");

        cfg.executable_file = std::env::current_exe().unwrap();

        Some(cfg)
    }
}
