use api::commands;
use api::notification::LogLevel;
use clap::ArgMatches;
use std::path::PathBuf;

// Import enum values directly into this namespace in order to make code more readable,
use self::CommandExecution::*;

pub fn parse() -> Option<(commands::Config, Box<commands::Command>)> {
    let yml = load_yaml!("argv.yml");
    let app = clap::App::from_yaml(yml);
    let matches = app.get_matches();

    parse_config_and_command(matches)
}

fn parse_config_and_command(
    matches: ArgMatches,
) -> Option<(commands::Config, Box<commands::Command>)> {
    if cfg!(debug_assertions) {
        dbg!(&matches);
    }

    let (cmd, dry_run) = match commands::Command::parse(&matches) {
        Some(DryRun(cmd)) => (cmd, true),
        Some(Run(cmd)) => (cmd, false),
        None => return None,
    };

    if let Some(cfg) = commands::Config::parse(&matches, dry_run) {
        Some((cfg, cmd))
    } else {
        None
    }
}

trait ConfigParser {
    fn parse(matches: &ArgMatches, dry_run: bool) -> Option<commands::Config>;
}

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

enum CommandExecution {
    DryRun(Box<commands::Command>),
    Run(Box<commands::Command>),
}

trait CommandParser {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution>;
}

impl CommandParser for commands::Command {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution> {
        if let Some(cmd) = commands::Clean::parse(matches) {
            return Some(cmd);
        }

        if let Some(cmd) = commands::List::parse(matches) {
            return Some(cmd);
        }

        if let Some(cmd) = commands::Purge::parse(matches) {
            return Some(cmd);
        }

        if let Some(cmd) = commands::Update::parse(matches) {
            return Some(cmd);
        }

        None
    }
}

impl CommandParser for commands::Clean {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution> {
        if let ("clean", Some(ref matches)) = matches.subcommand() {
            let mut cmd = Box::new(commands::Clean::new());

            if let Some(delete_hook) = matches.value_of("delete-hook") {
                cmd.delete_hook = delete_hook.to_owned();
            }

            cmd.dry_run = matches.is_present("dry-run");

            if let Some(root_dirs) = matches.values_of("root-dirs") {
                cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
            }

            if cfg!(debug_assertions) {
                dbg!(&cmd);
            }

            if cmd.dry_run {
                Some(DryRun(cmd))
            } else {
                Some(Run(cmd))
            }
        } else {
            None
        }
    }
}

impl CommandParser for commands::List {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution> {
        if let ("list", Some(ref matches)) = matches.subcommand() {
            let mut cmd = Box::new(commands::List::new());

            if let Some(filter) = matches.values_of("filter") {
                cmd.filter = filter
                    .into_iter()
                    .map(|list_filter| match list_filter {
                        "clashing" => commands::ListFilter::Clashing,
                        "correct" => commands::ListFilter::Correct,
                        "missing" => commands::ListFilter::Missing,
                        _ => panic!(),
                    })
                    .collect();
            }

            if let Some(root_dirs) = matches.values_of("root-dirs") {
                cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
            }

            if cfg!(debug_assertions) {
                dbg!(&cmd);
            }

            Some(Run(cmd))
        } else {
            None
        }
    }
}

impl CommandParser for commands::Purge {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution> {
        if let ("purge", Some(ref matches)) = matches.subcommand() {
            let mut cmd = Box::new(commands::Purge::new());

            cmd.dry_run = matches.is_present("dry-run");

            if let Some(root_dirs) = matches.values_of("root-dirs") {
                cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
            }

            if cfg!(debug_assertions) {
                dbg!(&cmd);
            }

            if cmd.dry_run {
                Some(DryRun(cmd))
            } else {
                Some(Run(cmd))
            }
        } else {
            None
        }
    }
}

impl CommandParser for commands::Update {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution> {
        if let ("update", Some(ref matches)) = matches.subcommand() {
            let mut cmd = Box::new(commands::Update::new());

            if let Some(create_hook) = matches.value_of("create-hook") {
                cmd.create_hook = create_hook.to_owned();
            }

            if let Some(delete_hook) = matches.value_of("delete-hook") {
                cmd.delete_hook = delete_hook.to_owned();
            }

            cmd.dry_run = matches.is_present("dry-run");

            if let Some(marker_text) = matches.value_of("marker-text") {
                cmd.marker_text = marker_text.to_owned();
            }

            if let Some(root_dirs) = matches.values_of("root-dirs") {
                cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
            }

            cmd.substitute_variables = matches.is_present("substitute-variables");

            if cfg!(debug_assertions) {
                dbg!(&cmd);
            }

            if cmd.dry_run {
                Some(DryRun(cmd))
            } else {
                Some(Run(cmd))
            }
        } else {
            None
        }
    }
}
