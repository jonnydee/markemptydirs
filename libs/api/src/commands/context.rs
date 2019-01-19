use super::Error;
use crate::fs;
use application::ApplicationInfo;
use notification::stdout::Stdout;
use notification::{LogLevel, Notifier};
use pathdiff::diff_paths;
use std;
use std::path::{Path, PathBuf};

pub type PathList = Vec<PathBuf>;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub exclude_dirs: PathList,
    pub executable_file: PathBuf,
    pub log_level: LogLevel,
    pub marker_name: String,
    pub dereference_symlinks: bool,
}

impl Config {
    pub fn default_root_dirs() -> PathList {
        vec![Path::new(".").to_owned()]
    }

    pub fn new() -> Config {
        Config {
            exclude_dirs: vec![Path::new(".git").to_owned()],
            executable_file: PathBuf::new(),
            log_level: LogLevel::Error,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub appinfo: ApplicationInfo,
    pub config: Config,
    pub fsaccess: Box<fs::access::FileSystemAccess>,
    pub notifier: Box<Notifier>,
}

impl Context {
    pub fn new(appinfo: ApplicationInfo, config: Config, dry_run: bool) -> Context {
        Context {
            appinfo: appinfo,
            notifier: Context::create_notifier(config.log_level),
            fsaccess: Context::create_fs_access(dry_run),
            config: config,
        }
    }

    pub fn crawl_dirs(&self, root_dirs: &PathList) -> fs::crawling::DirDescriptorList {
        let crawler = fs::crawling::FileSystemCrawler {
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
        let ref marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Write marker to disk.
        self.fsaccess.create_file(marker_file_path, text)?;

        self.notifier.info(
            "create_marker",
            &format!("Marker created: {:?}", marker_file_path),
            None,
        );
        Ok(())
    }

    pub fn create_marker_catched(&self, dir: &PathBuf, text: &String) {
        if let Err(error) = self.create_marker(dir, text) {
            self.notifier.error(
                "create_marker",
                &format!("{:?}", dir),
                Some(Error::Io(error)),
            )
        }
    }

    pub fn delete_child_file(&self, file: &PathBuf) -> std::io::Result<()> {
        // Remove file from disk.
        self.fsaccess.remove_file(file)?;

        self.notifier.info(
            "delete_child_file",
            &format!("Child file deleted: {:?}", file),
            None,
        );
        Ok(())
    }

    pub fn delete_child_file_catched(&self, file: &PathBuf) {
        if let Err(error) = self.delete_child_file(file) {
            self.notifier.error(
                "delete_child_file",
                &format!("{:?}", file),
                Some(Error::Io(error)),
            );
        }
    }

    pub fn delete_child_dir(&self, dir: &PathBuf) -> std::io::Result<()> {
        // Remove dir from disk.
        self.fsaccess.remove_dir_all(dir)?;

        self.notifier.info(
            "delete_child_dir",
            &format!("Child dir deleted: {:?}", dir),
            None,
        );
        Ok(())
    }

    pub fn delete_child_dir_catched(&self, dir: &PathBuf) {
        if let Err(error) = self.delete_child_dir(dir) {
            self.notifier.error(
                "delete_child_dir",
                &format!("{:?}", dir),
                Some(Error::Io(error)),
            );
        }
    }

    pub fn delete_marker(&self, dir: &PathBuf) -> std::io::Result<()> {
        let ref marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Remove marker from disk.
        self.fsaccess.remove_file(marker_file_path)?;

        self.notifier.info(
            "delete_marker",
            &format!("Marker deleted: {:?}", marker_file_path),
            None,
        );
        Ok(())
    }

    pub fn delete_marker_catched(&self, dir: &PathBuf) {
        if let Err(error) = self.delete_marker(dir) {
            self.notifier.error(
                "delete_marker",
                &format!("{:?}", dir),
                Some(Error::Io(error)),
            );
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
        let ref cur_dir = std::env::current_dir()?;
        match Context::get_relative_dir(dir, cur_dir) {
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

    fn create_fs_access(dry_run: bool) -> Box<fs::access::FileSystemAccess> {
        if dry_run {
            Box::new(fs::access::DryRunFileSystemAccess {})
        } else {
            Box::new(fs::access::RealFileSystemAccess {})
        }
    }

    fn create_notifier(log_level: LogLevel) -> Box<Notifier> {
        Box::new(Stdout::new(log_level))
    }
}
