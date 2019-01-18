extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

use api::commands;
use cli::argv;


fn main() {
    std_logger::init();

    if let Some((cfg, cmd)) = argv::parse_config_and_command() {
        let ctx = commands::Context::new(cfg);

        cmd.execute(&ctx).unwrap();

        if cfg!(debug_assertions) {
            dbg!(ctx);
        }
    }
}
