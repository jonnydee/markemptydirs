use super::{Error, Path, PathBuf, PathList};
use crate::fs;
use crate::fs::{DirDescriptorList, FileSystemAccess, FileSystemCrawler};
use application::ApplicationInfo;
use notification::{LogLevel, MessageLength, Notifier};
use std;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub exclude_dirs: PathList,
    pub executable_file: PathBuf,
    pub log_level: LogLevel,
    pub message_length: MessageLength,
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
            message_length: MessageLength::Long,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        }
    }
}

pub trait Context: Sync + Debug {
    fn get_appinfo(&self) -> &ApplicationInfo;

    fn get_config(&self) -> &Config;

    fn get_notifier(&self) -> &Notifier;

    fn get_root_dir<'a>(
        &self,
        dir: &PathBuf,
        root_dirs: &'a PathList,
    ) -> std::io::Result<Option<&'a PathBuf>>;

    fn crawl_dirs(&self, root_dirs: &PathList) -> DirDescriptorList;

    fn create_marker(&self, dir: &PathBuf, text: &String);

    fn delete_child_file(&self, file: &PathBuf);

    fn delete_child_dir(&self, dir: &PathBuf);

    fn delete_marker(&self, dir: &PathBuf);
}

#[derive(Debug)]
pub struct DefaultContext {
    appinfo: ApplicationInfo,
    config: Config,
    fsaccess: Box<FileSystemAccess>,
    notifier: Box<Notifier>,
}

impl DefaultContext {
    pub fn new(
        appinfo: ApplicationInfo,
        config: Config,
        dry_run: bool,
        notifier_factory: impl FnOnce(LogLevel, MessageLength) -> Box<Notifier>,
        fsaccess_factory: impl FnOnce(bool) -> Box<FileSystemAccess>,
    ) -> DefaultContext {
        DefaultContext {
            appinfo: appinfo,
            notifier: notifier_factory(config.log_level, config.message_length),
            fsaccess: fsaccess_factory(dry_run),
            config: config,
        }
    }

    fn create_marker_impl(&self, dir: &PathBuf, text: &String) -> std::io::Result<()> {
        let ref marker_file_path = self.get_marker_file_path(dir)?;

        // Write marker to disk.
        self.fsaccess.create_file(marker_file_path, text)?;

        self.notifier.info(
            "create_marker",
            "Marker created",
            &fs::to_native(marker_file_path),
            None,
        );
        Ok(())
    }

    fn delete_child_file_impl(&self, file: &PathBuf) -> std::io::Result<()> {
        // Remove file from disk.
        self.fsaccess.remove_file(file)?;

        self.notifier.info(
            "delete_child_file",
            "Child file deleted",
            &fs::to_native(file),
            None,
        );
        Ok(())
    }

    fn delete_child_dir_impl(&self, dir: &PathBuf) -> std::io::Result<()> {
        // Remove dir from disk.
        self.fsaccess.remove_dir_all(dir)?;

        self.notifier.info(
            "delete_child_dir",
            "Child dir deleted",
            &fs::to_native(dir),
            None,
        );
        Ok(())
    }

    fn delete_marker_impl(&self, dir: &PathBuf) -> std::io::Result<()> {
        let ref marker_file_path = self.get_marker_file_path(dir)?;

        // Remove marker from disk.
        self.fsaccess.remove_file(marker_file_path)?;

        self.notifier.info(
            "delete_marker",
            "Marker deleted",
            &fs::to_native(marker_file_path),
            None,
        );
        Ok(())
    }

    fn get_marker_file_path(&self, dir: &PathBuf) -> std::io::Result<PathBuf> {
        let mut dir = fs::get_absolute_dir(dir)?;
        dir.push(&self.config.marker_name);
        Ok(dir)
    }
}

impl Context for DefaultContext {
    fn get_appinfo(&self) -> &ApplicationInfo {
        &self.appinfo
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_notifier(&self) -> &Notifier {
        &*self.notifier
    }

    fn crawl_dirs(&self, root_dirs: &PathList) -> DirDescriptorList {
        let crawler = FileSystemCrawler {
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

    fn create_marker(&self, dir: &PathBuf, text: &String) {
        if let Err(error) = self.create_marker_impl(dir, text) {
            self.notifier.error(
                "create_marker",
                "Marker creation failed",
                &fs::to_native(dir),
                Some(Error::Io(error)),
            )
        }
    }

    fn delete_child_file(&self, file: &PathBuf) {
        if let Err(error) = self.delete_child_file_impl(file) {
            self.notifier.error(
                "delete_child_file",
                "Child file deletion failed",
                &fs::to_native(file),
                Some(Error::Io(error)),
            );
        }
    }

    fn delete_child_dir(&self, dir: &PathBuf) {
        if let Err(error) = self.delete_child_dir_impl(dir) {
            self.notifier.error(
                "delete_child_dir",
                "Child deletion failed",
                &fs::to_native(dir),
                Some(Error::Io(error)),
            );
        }
    }

    fn delete_marker(&self, dir: &PathBuf) {
        if let Err(error) = self.delete_marker_impl(dir) {
            self.notifier.error(
                "delete_marker",
                "Marker deletion failed",
                &fs::to_native(dir),
                Some(Error::Io(error)),
            );
        }
    }

    fn get_root_dir<'a>(
        &self,
        dir: &PathBuf,
        root_dirs: &'a PathList,
    ) -> std::io::Result<Option<&'a PathBuf>> {
        let dir = fs::get_absolute_dir(dir)?;
        Ok(root_dirs.iter().find(|root_dir| dir.starts_with(root_dir)))
    }
}
