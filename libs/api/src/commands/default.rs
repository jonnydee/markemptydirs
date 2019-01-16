use super::{Path, PathList};


pub fn root_dirs() -> PathList {
    vec![Path::new(".").to_owned()]
}
