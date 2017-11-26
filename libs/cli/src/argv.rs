use clap::App;
use api::context;
use api::commands;
use std::path::Path;


pub fn parse_config_and_command() -> Option<(context::Config, Box<commands::ICommand>)> {
    let yml = load_yaml!("argv.yml");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    println!("{:?}", matches);

    let cfg = context::Config::new();

    // let cmd = commands::CleanCommand {};

    // let cmd = commands::UpdateCommand {};

    let cmd = commands::List {
        filter: vec![],
        root_dirs: vec![Path::new(".").to_owned()],
    };

    Some((cfg, Box::new(cmd)))
}
