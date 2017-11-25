extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

use api::commands;
use api::commands::*;
use api::config;
use cli::argv;


fn main() {
    std_logger::init();

    let config = argv::parse_config();
    println!("{:?}", config);

    let ctx = commands::Context { config: config::Config::new() };

    // let cmd = commands::CleanCommand {};
    // cmd.execute(&ctx).unwrap();

    // let cmd = commands::UpdateCommand {};
    // cmd.execute(&ctx).unwrap();

    let cmd = commands::OverviewCommand {};
    cmd.execute(&ctx).unwrap();
}
