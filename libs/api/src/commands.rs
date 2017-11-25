use config;
use fscrawling;
use pathdiff::diff_paths;
use rayon::prelude::*;
use std;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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
        Other(err: Box<std::error::Error>) {
            cause(&**err)
            description(err.description())
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Context {
    pub config: config::Config,
    // base_dir: PathBuf,
    // current_dir: PathBuf,
}

impl Context {
    pub fn crawl_dirs(&self, root_dirs: &PathList) -> fscrawling::DirDescriptorList {
        let crawler = fscrawling::FileSystemCrawler {
            exclude_dirs: self.config.exclude_dirs.clone(),
            dereference_symlinks: self.config.dereference_symlinks,
            marker_name: self.config.marker_name.clone(),
        };

        crawler
            .crawl_dirs(root_dirs.clone())
            .into_iter()
            .map(|(_, descr)| descr)
            .collect()
    }

    pub fn create_marker(&self, dir: &PathBuf, text: &String) -> std::io::Result<()> {
        let marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Write marker to disk.
        {
            let mut file = File::create(&marker_file_path)?;
            file.write_all(text.as_bytes())?;
        }

        info!(target: "create_marker", "Marker created: {:?}", &marker_file_path);
        Ok(())
    }

    pub fn create_marker_catched(&self, dir: &PathBuf, text: &String) {
        if let Err(error) = self.create_marker(dir, text) {
            error!(target: "create_marker", "{}: {:?}", error, &dir);
        }
    }

    pub fn delete_marker(&self, dir: &PathBuf) -> std::io::Result<()> {
        let marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Remove marker from disk.
        fs::remove_file(&marker_file_path)?;

        info!(target: "delete_marker", "Marker deleted: {:?}", &marker_file_path);
        Ok(())
    }

    pub fn delete_marker_catched(&self, dir: &PathBuf) {
        if let Err(error) = self.delete_marker(&dir) {
            error!(target: "delete_marker", "{}: {:?}", error, &dir);
        }
    }

    pub fn get_absolute_dir(dir: &PathBuf) -> std::io::Result<PathBuf> {
        if dir.is_absolute() {
            return Ok(dir.clone());
        }
        let mut abs_dir = std::env::current_dir()?;
        abs_dir.push(dir);
        Ok(abs_dir)
    }

    pub fn get_relative_dir(dir: &PathBuf, base_dir: &PathBuf) -> Option<PathBuf> {
        diff_paths(dir, base_dir)
    }

    pub fn get_relative_dir_to_current_dir(dir: &PathBuf) -> std::io::Result<Option<PathBuf>> {
        let cur_dir = std::env::current_dir()?;
        match Context::get_relative_dir(dir, &cur_dir) {
            Some(dir) => {
                let rel_dir = Path::new(".");
                if dir.iter().next().is_some() {
                    Ok(Some(rel_dir.join(dir)))
                } else {
                    Ok(Some(rel_dir.to_owned()))
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_root_dir<'a>(
        &self,
        dir: &PathBuf,
        root_dirs: &'a PathList,
    ) -> std::io::Result<Option<&'a PathBuf>> {
        let dir = Context::get_absolute_dir(dir)?;
        Ok(root_dirs.iter().find(|root_dir| dir.starts_with(root_dir)))
    }
}

pub trait ICommand {
    fn execute(&self, ctx: &Context) -> Result<()>;
}


pub struct CleanCommand {
    pub delete_hook: String,
    pub dry_run: bool,
    pub root_dirs: PathList,
}
impl ICommand for CleanCommand {
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


#[derive(PartialEq, Debug)]
pub enum ListFilter {
    Clashing,
    Correct,
    Missing,
}

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

impl ICommand for List {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let mut statistics_list: Vec<_> = ctx.crawl_dirs(&self.root_dirs)
            .into_par_iter()
            .map(|descr| {
                ListStatistics {
                    marker_found: descr.has_marker(),
                    marker_required: !descr.has_children(),
                    child_count: descr.get_child_count(),
                    dir_count: descr.get_sub_directory_count(),
                    dir: match Context::get_relative_dir_to_current_dir(&descr.dir) {
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


pub struct Purge {
    pub dry_run: bool,
    pub root_dirs: PathList,
}


pub struct Update {
    pub create_hook: String,
    pub delete_hook: String,
    pub dry_run: bool,
    pub marker_text: String,
    pub root_dirs: PathList,
    pub substitute_variables: bool,
}
impl ICommand for Update {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let descr_list = ctx.crawl_dirs(&self.root_dirs);

        // Delete markers.
        descr_list.par_iter().for_each(|descr| {
            if descr.has_marker() && descr.has_children() {
                ctx.delete_marker_catched(&descr.dir);
            }
        });

        // Create markers.
        descr_list.par_iter().for_each(|descr| {
            if !descr.has_marker() && !descr.has_children() {
                ctx.create_marker_catched(&descr.dir, &self.marker_text);
            }
        });

        Ok(())
    }
}
