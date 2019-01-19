use super::*;

use rayon::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Clean {
    pub delete_hook: String,
    pub dry_run: bool,
    pub root_dirs: PathList,
}

impl Clean {
    pub fn new() -> Clean {
        Clean {
            delete_hook: String::new(),
            dry_run: false,
            root_dirs: Config::default_root_dirs(),
        }
    }
}

impl Command for Clean {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let descr_list = ctx.crawl_dirs(&self.root_dirs);

        // Delete all markers.
        descr_list.par_iter().for_each(|descr| {
            if descr.has_marker() {
                ctx.delete_marker_catched(&descr.dir);
            }
        });

        Ok(())
    }
}
