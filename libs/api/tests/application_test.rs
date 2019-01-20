#![allow(non_snake_case)]

extern crate api;

use api::application::*;


mod test_VersionInfo {
    use super::*;

    #[test]
    fn new() {
        assert_eq!("1976.10.3", VersionInfo::new(1976, 10, 3).to_string());
    }

    #[test]
    fn new_with_suffix() {
        assert_eq!("1976.10.3-stable", VersionInfo::new_with_suffix(1976, 10, 3, "stable").to_string());
    }
}
