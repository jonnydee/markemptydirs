use std::path::{Path, PathBuf};

use context;

mod default;

mod result;
pub use self::result::*;

mod clean;
pub use self::clean::*;

mod list;
pub use self::list::*;

mod purge;
pub use self::purge::*;

mod update;
pub use self::update::*;

pub type PathList = Vec<PathBuf>;


pub trait ICommand {
    fn execute(&self, ctx: &context::Context) -> Result<()>;
}
