use std::path::{Path, PathBuf};
use Error;

mod clean;
pub use self::clean::*;

mod context;
pub use self::context::*;

mod default;

mod list;
pub use self::list::*;

mod purge;
pub use self::purge::*;

mod update;
pub use self::update::*;

pub type PathList = Vec<PathBuf>;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ICommand {
    fn execute(&self, ctx: &context::Context) -> Result<()>;
}