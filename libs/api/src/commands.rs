use context;
use rayon::prelude::*;
use std;
use std::path::{Path, PathBuf};
// use text_table::Table;


pub type PathList = Vec<PathBuf>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            cause(err)
            description(err.description())
        }
        Utf8(err: std::str::Utf8Error) {
            description("utf8 error")
        }
        Message(str: &'static str) {
            description("error message")
        }
        Other(err: Box<std::error::Error>) {
            cause(&**err)
            description(err.description())
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ICommand {
    fn execute(&self, ctx: &context::Context) -> Result<()>;
}


fn default_root_dirs() -> PathList {
    vec![Path::new(".").to_owned()]
}


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
            root_dirs: default_root_dirs(),
        }
    }
}

impl ICommand for Clean {
    fn execute(&self, ctx: &context::Context) -> Result<()> {
        let descr_list = ctx.crawl_dirs(&self.root_dirs);

        // Delete all markers.
        descr_list.par_iter().for_each(|descr| {
            if descr.has_marker() {
                ctx.delete_marker_catched(&descr.dir, self.dry_run);
            }
        });

        Ok(())
    }
}


#[derive(PartialEq, Debug)]
pub enum ListFilter {
    Clashing,
    Correct,
    Missing,
}

#[derive(PartialEq, Debug)]
pub struct List {
    pub filter: Vec<ListFilter>,
    pub root_dirs: PathList,
}

#[derive(Debug)]
struct ListStatistics {
    pub dir: PathBuf,
    pub marker_found: bool,
    pub marker_required: bool,
    pub child_count: usize,
    pub dir_count: usize,
}

impl List {
    pub fn new() -> List {
        List {
            filter: vec![],
            root_dirs: default_root_dirs(),
        }
    }
}

impl ICommand for List {
    fn execute(&self, ctx: &context::Context) -> Result<()> {
        let mut statistics_list: Vec<_> = ctx.crawl_dirs(&self.root_dirs)
            .into_par_iter()
            .map(|descr| {
                ListStatistics {
                    marker_found: descr.has_marker(),
                    marker_required: !descr.has_children(),
                    child_count: descr.get_child_count(),
                    dir_count: descr.get_sub_directory_count(),
                    dir: match context::Context::get_relative_dir_to_current_dir(&descr.dir) {
                        Ok(Some(dir)) => dir,
                        _ => descr.dir,
                    },
                }
            })
            .collect();

        statistics_list
            .as_mut_slice()
            .par_sort_unstable_by_key(|stat| stat.dir.clone());

        for stat in statistics_list {
            println!("{:?}", stat);
        }
        // let mut table = Table::new();
        // table.row(("Path", "M. found?", "M. needed!", "Entries", "Dirs"))
        //     .sep();
        // for stat in statistics_list {
        //     table.row((
        //         format!("{}", stat),
        //         print!("{}", if stat.marker_found { "yes" } else { "no" }),
        //         print!("{}", if stat.marker_required { "yes" } else { "no" }),
        //         stat.child_count,
        //         stat.dir_count);

        // println!("{}", table.write_to_string());

        Ok(())
    }
}


#[derive(PartialEq, Debug)]
pub struct Purge {
    pub dry_run: bool,
    pub root_dirs: PathList,
}

impl Purge {
    pub fn new() -> Purge {
        Purge {
            dry_run: false,
            root_dirs: default_root_dirs(),
        }
    }
}

impl ICommand for Purge {
    fn execute(&self, ctx: &context::Context) -> Result<()> {
        Err(Error::Message(""))
    }
}


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
            root_dirs: default_root_dirs(),
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
