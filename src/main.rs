extern crate api;
extern crate clap;
extern crate cli;
extern crate std_logger;

fn main() {
    std_logger::init();

    let appinfo = create_appinfo();
    let session = cli::argv::create_session(appinfo).unwrap();

    if cfg!(debug_assertions) {
        dbg!(&session);
    }

    session.run().unwrap();
}

fn create_appinfo() -> api::application::ApplicationInfo {
    api::application::ApplicationInfo {
        copyright_year: 2018,
        license: "Simplified BSD License",
        name: "markemptydirs",
        site: "https://github.com/jonnydee/markemptydirs-rs",
        vendor_email: "jonny.dee@posteo.net",
        vendor_name: "Johann Duscher",
        version_info: api::application::VersionInfo {
            major: 0,
            minor: 1,
            bugfix: 0,
            suffix: "beta1",
        },
    }
}
