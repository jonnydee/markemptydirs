mod command_parsing;

mod config_parsing;

use api::commands;
use clap::ArgMatches;

pub trait ConfigParser {
    fn parse(matches: &ArgMatches, dry_run: bool) -> Option<commands::Config>;
}

pub enum CommandExecution {
    DryRun(Box<commands::Command>),
    Run(Box<commands::Command>),
}

pub trait CommandParser {
    fn parse(matches: &ArgMatches) -> Option<CommandExecution>;
}

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
