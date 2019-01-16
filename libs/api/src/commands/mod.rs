use std::path::{Path, PathBuf};

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

mod result;
pub use self::result::*;

pub type PathList = Vec<PathBuf>;


pub trait ICommand {
    fn execute(&self, ctx: &context::Context) -> Result<()>;
}
