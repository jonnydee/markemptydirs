use config;
use fscrawling;
use rayon::prelude::*;
use std;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;


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
    pub fn crawl_dirs(&self) -> fscrawling::DirDescriptorMap {
        let crawler = fscrawling::FileSystemCrawler {
            exclude_dirs: self.config.exclude_dirs.clone(),
            dereference_symlinks: self.config.dereference_symlinks,
            marker_name: self.config.marker_name.clone(),
        };

        crawler.crawl_dirs(self.config.root_dirs.clone())
    }

    pub fn create_marker(&self, dir: &PathBuf) -> std::io::Result<()> {
        let marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Write marker to disk.
        {
            let mut file = File::create(&marker_file_path)?;
            file.write_all(self.config.marker_text.as_bytes())?;
        }

        info!(target: "create_marker", "Marker created: {:?}", &marker_file_path);
        Ok(())
    }

    pub fn create_marker_catched(&self, dir: &PathBuf) {
        if let Err(error) = self.create_marker(&dir) {
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

    pub fn get_root_dir(&self, dir: &PathBuf) -> std::io::Result<Option<&PathBuf>> {
        let dir = Context::get_absolute_dir(dir)?;
        Ok(self.config.root_dirs.iter().find(|root_dir| {
            dir.starts_with(root_dir)
        }))
    }
}

pub trait ICommand {
    fn execute(&self, ctx: &Context) -> Result<()>;
}


pub struct UpdateCommand {}
impl ICommand for UpdateCommand {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let descr_list: Vec<_> = ctx.crawl_dirs().into_iter().collect();

        // Delete markers.
        descr_list.par_iter().for_each(
            |&(ref dir, ref descr)| if descr.get_marker_direntry().is_some() &&
                descr.has_children()
            {
                ctx.delete_marker_catched(&dir);
            },
        );

        // Create markers.
        descr_list.par_iter().for_each(
            |&(ref dir, ref descr)| if descr.get_marker_direntry().is_none() &&
                !descr.has_children()
            {
                ctx.create_marker_catched(&dir);
            },
        );

        Ok(())
    }
}

pub struct CleanCommand {}
impl ICommand for CleanCommand {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let descr_list: Vec<_> = ctx.crawl_dirs().into_iter().collect();

        // Delete all markers.
        descr_list.par_iter().for_each(|&(ref dir, ref descr)| {
            if descr.get_marker_direntry().is_some() {
                ctx.delete_marker_catched(&dir);
            }
        });

        Ok(())
    }
}
