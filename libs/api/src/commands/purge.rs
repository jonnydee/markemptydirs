use super::*;

#[derive(PartialEq, Debug)]
pub struct Purge {
    pub dry_run: bool,
    pub root_dirs: PathList,
}

impl Purge {
    pub fn new() -> Purge {
        Purge {
            dry_run: false,
            root_dirs: Config::default_root_dirs(),
        }
    }
}

impl Command for Purge {
    fn execute(&self, _ctx: &Context) -> Result<()> {
        // TODO Implement Purge command.
        Err(Error::Message("To be implementd"))
    }
}
