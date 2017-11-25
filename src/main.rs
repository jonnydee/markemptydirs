extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

use api::commands;
use api::commands::*;
use api::config;
use cli::argv;
use std::path::Path;


fn main() {
    std_logger::init();

    let config = argv::parse_config();
    println!("{:?}", config);

    let ctx = commands::Context {
        config: config::Config::new(),
    };

    // let cmd = commands::CleanCommand {};
    // cmd.execute(&ctx).unwrap();

    // let cmd = commands::UpdateCommand {};
    // cmd.execute(&ctx).unwrap();

    let cmd = commands::List {
        filter: vec![],
        root_dirs: vec![Path::new(".").to_owned()],
    };
    cmd.execute(&ctx).unwrap();
}
