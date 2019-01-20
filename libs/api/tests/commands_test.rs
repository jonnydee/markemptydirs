#![allow(non_snake_case)]

extern crate api;

mod common;
use common::*;

use api::commands::*;
use api::fs::*;
use api::notification::{LogLevel, MessageLength};

mod test_Config {
    use super::*;

    #[test]
    pub fn default_root_dirs() {
        assert_eq!(vec![Path::new(".").to_owned()], Config::default_root_dirs());
    }

    #[test]
    pub fn new() {
        let expected = Config {
            exclude_dirs: vec![Path::new(".git").to_owned()],
            executable_file: PathBuf::new(),
            log_level: LogLevel::Error,
            message_length: MessageLength::Long,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        };

        assert_eq!(expected, Config::new());
    }
}

mod test_Clean {
    use super::*;

    #[test]
    pub fn new() {
        let expected = Clean {
            delete_hook: String::new(),
            dry_run: false,
            root_dirs: Config::default_root_dirs(),
        };

        assert_eq!(expected, Clean::new());
    }

    #[test]
    pub fn execute() {
        let ctx = TestContext::new(|_| DirDescriptorList::new());
        let sut = Clean::new();
        
        sut.execute(&ctx).unwrap();

        assert_eq!(0, ctx.get_markers_created().len());
        assert_eq!(0, ctx.get_markers_deleted().len());
    }
}
