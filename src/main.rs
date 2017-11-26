extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

use cli::argv;


fn main() {
    std_logger::init();

    if let Some((ctx, cmd)) = argv::parse_context_and_command() {
        println!("{:?}", ctx);
        cmd.execute(&ctx).unwrap();
    }
}
