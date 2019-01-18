use std::fs::File;
use std::io::Result;
use std::path::Path;
use std::io::prelude::*;


pub trait FileSystemAccess: Sync + std::fmt::Debug {

    fn create_file(&self, path: &Path, text: &str) -> Result<()>;

    fn remove_file(&self, path: &Path) -> Result<()>;

    fn remove_dir_all(&self, path: &Path) -> Result<()>;

}


#[derive(Debug)]
pub struct RealFileSystemAccess {}

impl FileSystemAccess for RealFileSystemAccess {

    fn create_file(&self, path: &Path, text: &str) -> Result<()> {
        let mut file = File::create(&path)?;
        file.write_all(text.as_bytes())
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        std::fs::remove_file(&path)
    }

    fn remove_dir_all(&self, _path: &Path) -> Result<()> {
        // TODO std::fs::remove_dir_all(&path)
        panic!("To be implemented!");
    }
    
}


#[derive(Debug)]
pub struct DryRunFileSystemAccess {}

impl FileSystemAccess for DryRunFileSystemAccess {

    fn create_file(&self, _path: &Path, _text: &str) -> Result<()> {
        Ok(())
    }

    fn remove_file(&self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn remove_dir_all(&self, _path: &Path) -> Result<()> {
        Ok(())
    }
    
}
