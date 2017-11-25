use clap::App;
use api::config::Config;


pub fn parse_config() -> Option<Config> {
    let yml = load_yaml!("argv.yml");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    println!("{:?}", matches);
    None
}
