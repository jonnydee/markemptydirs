extern crate api;

use api::application::*;
use api::commands::*;
use api::fs::*;
use api::notification::*;
use std::sync::{Arc, Mutex};

pub struct TestContext {
    appinfo: ApplicationInfo,
    config: Config,
    notifier: StdoutNotifier,

    crawl_dirs_fn: Box<Fn(&PathList) -> DirDescriptorList>,

    markers_created: Arc<Mutex<PathList>>,
    markers_deleted: Arc<Mutex<PathList>>,
}

impl TestContext {
    pub fn new<F: 'static>(crawl_dirs_fn: F) -> TestContext
    where
        F: Fn(&PathList) -> DirDescriptorList,
    {
        TestContext {
            appinfo: ApplicationInfo {
                copyright_year: 2018,
                license: "Simplified BSD License",
                name: "markemptydirs",
                site: "https://github.com/jonnydee/markemptydirs-rs",
                vendor_email: "jonny.dee@posteo.net",
                vendor_name: "Johann Duscher",
                version_info: VersionInfo::new_with_suffix(0, 1, 0, "beta1"),
            },
            config: Config::new(),
            notifier: StdoutNotifier {
                log_level: LogLevel::Debug,
                message_length: MessageLength::Long,
            },
            crawl_dirs_fn: Box::new(crawl_dirs_fn),
            markers_created: Arc::new(Mutex::new(Vec::new())),
            markers_deleted: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_markers_created(&self) -> PathList {
        self.markers_created.lock().unwrap().clone()
    }

    pub fn get_markers_deleted(&self) -> PathList {
        self.markers_deleted.lock().unwrap().clone()
    }
}

unsafe impl Sync for TestContext {}

impl std::fmt::Debug for TestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}), {:?})",
            self.appinfo, self.config, self.notifier,
        )
    }
}

impl Context for TestContext {
    fn get_appinfo(&self) -> &ApplicationInfo {
        &self.appinfo
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_notifier(&self) -> &Notifier {
        &self.notifier
    }

    fn get_root_dir<'a>(
        &self,
        dir: &PathBuf,
        root_dirs: &'a PathList,
    ) -> std::io::Result<Option<&'a PathBuf>> {
        let dir = get_absolute_dir(dir)?;
        Ok(root_dirs.iter().find(|root_dir| dir.starts_with(root_dir)))
    }

    fn crawl_dirs(&self, root_dirs: &PathList) -> DirDescriptorList {
        (self.crawl_dirs_fn)(root_dirs)
    }

    fn create_marker(&self, dir: &PathBuf, _text: &String) {
        self.markers_created.lock().unwrap().push(dir.to_owned())
    }

    fn delete_child_file(&self, _file: &PathBuf) {}

    fn delete_child_dir(&self, _dir: &PathBuf) {}

    fn delete_marker(&self, dir: &PathBuf) {
        self.markers_deleted.lock().unwrap().push(dir.to_owned())
    }
}
