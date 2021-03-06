pub use fs::{Path, PathBuf, PathList};

use Error;

mod clean;
pub use self::clean::*;

mod context;
pub use self::context::*;

mod list;
pub use self::list::*;

mod purge;
pub use self::purge::*;

mod update;
pub use self::update::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Command: std::fmt::Debug {
    fn execute(&self, ctx: &context::Context) -> Result<()>;
}

#[derive(Debug)]
pub enum Execution {
    DryRun(Box<Command>),
    Run(Box<Command>),
}
