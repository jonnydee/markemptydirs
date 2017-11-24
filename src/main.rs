extern crate api;
extern crate clap;
extern crate std_logger;

use api::config;
use api::commands;
use api::commands::*;
use api::fscrawling;
// use clap::{Arg, App, SubCommand};
use std::path::Path;


fn main() {
    std_logger::init();

    let ctx = commands::Context { config: config::Config::new() };
    let cmd = commands::UpdateCommand {};
    cmd.execute(&ctx).unwrap();

    // let crawler = fscrawling::FileSystemCrawler {
    //     exclude_dirs: vec![Path::new(".git").to_owned()],
    //     dereference_symlinks: false,
    //     marker_name: ".emptydir".to_string(),
    // };

    // let path_map = crawler.crawl_dirs(vec![Path::new(".").to_owned()]);
    // for (path, descr) in path_map {
    //     println!(
    //         "YES:: {:?} [has_marker={:?}, subdir_count={:?}])",
    //         path,
    //         descr.get_marker_direntry(),
    //         descr.get_sub_direntry_count(),
    //     );
    // }

    // let matches = App::new("MarkEmptyDirs")
    //     .version("1.0")
    //     .author("Jonny Dee <jonny.dee@gmx.net>")
    //     .about("Does awesome things")
    //     .arg(Arg::with_name("v").short("v").multiple(true).help(
    //         "Sets the level of verbosity",
    //     ))
    //     .get_matches();

    // for entry in WalkDir::new(".") {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }
}
