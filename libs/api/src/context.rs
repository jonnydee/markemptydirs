use fscrawling;
use log::LogLevel;
use pathdiff::diff_paths;
use std;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};


pub type PathList = Vec<PathBuf>;


#[derive(Debug)]
pub struct VersionInfo {
    pub major: u16,
    pub minor: u16,
    pub bugfix: u16,
    pub suffix: String,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if self.bugfix > 0 {
            write!(f, ".{}", self.bugfix)?;
        }
        if !self.suffix.is_empty() {
            write!(f, "-{}", &self.suffix)?;
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct ApplicationInfo {
    pub copyright: String,
    pub disclaimer: String,
    pub license: String,
    pub name: String,
    pub site: String,
    pub vendor_email: String,
    pub vendor_name: String,
    pub version_info: VersionInfo,
}


#[derive(PartialEq, Debug)]
pub struct Config {
    pub exclude_dirs: PathList,
    pub executable_file: String,
    pub log_level: LogLevel,
    pub marker_name: String,
    pub dereference_symlinks: bool,
}

impl Config {
    pub fn new() -> Config {
        // pub fn new(application_info: ApplicationInfo) -> Config {
        Config {
            // application_info: application_info,
            exclude_dirs: vec![Path::new(".git").to_owned()],
            executable_file: "".to_string(),
            log_level: LogLevel::Error,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        }
    }
}


#[derive(Debug)]
pub struct Context {
    // pub application_info: config::ApplicationInfo,
    pub config: Config,
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


// #[derive(PartialEq, Debug)]
// pub enum CommandListFilter {
//     Clashing,
//     Correct,
//     Missing,
// }

// #[derive(PartialEq, Debug)]
// pub enum Command {
//     Clean {
//         delete_hook: String,
//         dry_run: bool,
//         root_dirs: PathList,
//     },
//     List {
//         filter: Vec<CommandListFilter>,
//         root_dirs: PathList,
//     },
//     Purge { dry_run: bool, root_dirs: PathList },
//     Update {
//         create_hook: String,
//         delete_hook: String,
//         dry_run: bool,
//         marker_text: String,
//         root_dirs: PathList,
//         substitute_variables: bool,
//     },
// }
