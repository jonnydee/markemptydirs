extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

fn main() -> api::commands::Result<()> {
    std_logger::init();

    let session = cli::argv::create_session().unwrap();

    if cfg!(debug_assertions) {
        dbg!(&session);
    }

    session.run()
}
