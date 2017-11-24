use log::LogLevel;
use std::fmt;
use std::path::PathBuf;


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


#[derive(Debug)]
pub enum Command {
    None,
    Help,
    Update,
    Clean,
    Overview,
    Purge,
    Version,
}

#[derive(Debug)]
pub struct Config {
    pub application_info: ApplicationInfo,
    pub command: Command,
    pub create_hook_command: String,
    pub delete_hook_command: String,
    pub dry_run: bool,
    pub exclude_dirs: PathList,
    pub executable_file: String,
    pub help_text: String,
    pub log_level: LogLevel,
    pub marker_name: String,
    pub marker_text: String,
    pub dereference_symlinks: bool,
    pub root_dirs: PathList,
    pub substitute_variables: bool,
}

impl Config {
    pub fn new(application_info: ApplicationInfo) -> Config {
        Config {
            application_info: application_info,
            command: Command::None,
            create_hook_command: "".to_string(),
            delete_hook_command: "".to_string(),
            dry_run: false,
            exclude_dirs: vec![],
            executable_file: "".to_string(),
            help_text: "".to_string(),
            log_level: LogLevel::Error,
            marker_name: ".emptydir".to_string(),
            marker_text: "".to_string(),
            dereference_symlinks: false,
            root_dirs: vec![],
            substitute_variables: true,
        }
    }
}
