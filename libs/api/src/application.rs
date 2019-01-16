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
