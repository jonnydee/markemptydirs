extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

use api::context;
use cli::argv;


fn main() {
    std_logger::init();

    if let Some((cfg, cmd)) = argv::parse_config_and_command() {
        let ctx = context::Context { config: cfg };

        cmd.execute(&ctx).unwrap();

        println!("{:?}", ctx);
    }
}
