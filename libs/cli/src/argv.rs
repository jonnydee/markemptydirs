use api::commands;
use clap::App;
use clap::ArgMatches;
use api::LogLevel;
use std::path::PathBuf;


pub fn parse_config_and_command() -> Option<(commands::Config, Box<commands::ICommand>)> {
    let yml = load_yaml!("argv.yml");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    println!("{:?}", matches);

    let cfg = parse_config(&matches);

    let cmd = parse_command(&matches);
 
    Some((cfg, cmd))
}

fn parse_config(matches : &ArgMatches) -> commands::Config {
    let mut cfg = commands::Config::new();

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

    cfg
}

fn parse_command(matches : &ArgMatches) -> Box<commands::ICommand> {
    match matches.subcommand() {
        ("clean", Some(ref matches)) => parse_command_clean(matches),
        ("list", Some(ref matches)) => parse_command_list(matches),
        ("purge", Some(ref matches)) => parse_command_purge(matches),
        ("update", Some(ref matches)) => parse_command_update(matches),
        _ => panic!(),
    }
}

fn parse_command_clean(matches : &ArgMatches) -> Box<commands::Clean> {
    let mut cmd = commands::Clean::new();

    if let Some(delete_hook) = matches.value_of("delete-hook") {
        cmd.delete_hook = delete_hook.to_owned();
    }

    cmd.dry_run = matches.is_present("dry-run");

    if let Some(root_dirs) = matches.values_of("root-dirs") {
        cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
    }

    println!("COMMAND: {:?}", &cmd);
    Box::new(cmd)
}

fn parse_command_list(matches : &ArgMatches) -> Box<commands::List> {
    let mut cmd = commands::List::new();

    if let Some(filter) = matches.values_of("filter") {
        cmd.filter = filter.into_iter().map(|list_filter| match list_filter {
            "clashing" => commands::ListFilter::Clashing,
            "correct" => commands::ListFilter::Correct,
            "missing" => commands::ListFilter::Missing,
            _ => panic!(),
        }).collect();
    }
    
    if let Some(root_dirs) = matches.values_of("root-dirs") {
        cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
    }

    println!("COMMAND: {:?}", &cmd);
    Box::new(cmd)
}

fn parse_command_purge(matches : &ArgMatches) -> Box<commands::Purge> {
    let mut cmd = commands::Purge::new();

    cmd.dry_run = matches.is_present("dry-run");

    if let Some(root_dirs) = matches.values_of("root-dirs") {
        cmd.root_dirs = root_dirs.into_iter().map(PathBuf::from).collect();
    }

    println!("COMMAND: {:?}", &cmd);
    Box::new(cmd)
}

fn parse_command_update(matches : &ArgMatches) -> Box<commands::Update> {
    let mut cmd = commands::Update::new();

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

    println!("COMMAND: {:?}", &cmd);
    Box::new(cmd)
}
