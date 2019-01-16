use context;

use super::*;

use rayon::prelude::*;


#[derive(PartialEq, Debug)]
pub struct Update {
    pub create_hook: String,
    pub delete_hook: String,
    pub dry_run: bool,
    pub marker_text: String,
    pub root_dirs: PathList,
    pub substitute_variables: bool,
}

impl Update {
    pub fn new() -> Update {
        Update {
            create_hook: String::new(),
            delete_hook: String::new(),
            dry_run: false,
            marker_text: String::new(),
            root_dirs: default::root_dirs(),
            substitute_variables: true,
        }
    }
}

impl ICommand for Update {
    fn execute(&self, ctx: &context::Context) -> Result<()> {
        let descr_list = ctx.crawl_dirs(&self.root_dirs);

        // Delete markers.
        descr_list.par_iter().for_each(|descr| {
            if descr.has_marker() && descr.has_children() {
                ctx.delete_marker_catched(&descr.dir, self.dry_run);
            }
        });

        // Create markers.
        descr_list.par_iter().for_each(|descr| {
            if !descr.has_marker() && !descr.has_children() {
                ctx.create_marker_catched(&descr.dir, &self.marker_text, self.dry_run);
            }
        });

        Ok(())
    }
}
