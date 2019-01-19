mod command_parsing;

mod config_parsing;

use api::application;
use api::commands;
use clap::ArgMatches;

pub trait ConfigParser {
    fn parse(matches: &ArgMatches) -> Option<commands::Config>;
}

pub trait CommandParser {
    fn parse(matches: &ArgMatches) -> Option<commands::Execution>;
}

pub fn create_session() -> Option<application::Session> {
    let yml = load_yaml!("argv.yml");
    let app = clap::App::from_yaml(yml);
    let matches = app.get_matches();

    if cfg!(debug_assertions) {
        dbg!(&matches);
    }

    if let Some(cfg) = commands::Config::parse(&matches) {
        if let Some(exec) = commands::Command::parse(&matches) {
            return Some(application::Session::new(cfg, exec));
        }
    }

    None
}
