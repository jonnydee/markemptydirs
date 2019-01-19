use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

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
    match get_relative_dir(dir, cur_dir) {
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
