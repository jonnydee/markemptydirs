extern crate api;
extern crate clap;
extern crate std_logger;
extern crate ui;

use api::application::{ApplicationInfo, VersionInfo};
use ui::cli::create_session;

fn main() {
    std_logger::init();

    let appinfo = create_appinfo();
    let session = create_session(appinfo).unwrap();

    if cfg!(debug_assertions) {
        dbg!(&session);
    }

    session.run().unwrap();
}

fn create_appinfo() -> ApplicationInfo {
    ApplicationInfo {
        copyright_year: 2018,
        license: "Simplified BSD License",
        name: "markemptydirs",
        site: "https://github.com/jonnydee/markemptydirs-rs",
        vendor_email: "jonny.dee@posteo.net",
        vendor_name: "Johann Duscher",
        version_info: VersionInfo::new_with_suffix(0, 1, 0, "beta1"),
    }
}
