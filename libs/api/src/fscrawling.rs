use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;


pub type DirEntryList = Vec<DirEntry>;
pub type PathList = Vec<PathBuf>;

#[derive(Debug)]
pub struct DirDescriptor {
    pub dir: PathBuf,
    pub children: DirEntryList,

    marker_file_child_index: Option<usize>,
    subdir_child_indexes: Vec<usize>,
}

impl DirDescriptor {
    pub fn get_marker_direntry(&self) -> Option<&DirEntry> {
        if let Some(index) = self.marker_file_child_index {
            Some(&self.children[index])
        } else {
            None
        }
    }

    pub fn for_each_sub_direntry<F>(&self, f: F)
    where
        F: FnMut(&DirEntry) -> (),
    {
        self.subdir_child_indexes
            .iter()
            .map(|index| &self.children[*index])
            .for_each(f)
    }

    pub fn get_sub_direntry_count(&self) -> usize {
        self.subdir_child_indexes.len()
    }

    pub fn get_sub_direntries(&self) -> Vec<&DirEntry> {
        self.subdir_child_indexes
            .iter()
            .map(|index| &self.children[*index])
            .collect()
    }
}

pub type DirDescriptorMap = HashMap<PathBuf, DirDescriptor>;

#[derive(Debug)]
pub struct FileSystemCrawler {
    pub exclude_dirs: PathList,
    pub dereference_symlinks: bool,
    pub marker_name: String,
}

impl FileSystemCrawler {
    fn crawl_dir(&self, mut dir: PathBuf) -> Option<DirDescriptor> {
        dir = match dir.canonicalize() {
            Ok(dir) => dir,
            Err(error) => {
                warn!(target: "FileSystemCrawler", "{}", error);
                return None;
            }
        };

        let entries = match dir.read_dir() {
            Ok(entries) => entries,
            Err(error) => {
                warn!(target: "FileSystemCrawler", "{}", error);
                return None;
            }
        };
        let children: DirEntryList = entries
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(error) => {
                    warn!(target: "FileSystemCrawler", "{}", error);
                    None
                }
            })
            .collect();

        let mut marker_file_child_index = None;
        let mut subdir_child_indexes = vec![];
        children.iter().enumerate().for_each(|(index, entry)| {
            if self.is_crawlable_dir(&entry.path()) {
                subdir_child_indexes.push(index);
            } else if marker_file_child_index.is_none() {
                let entry_file_name = entry.file_name(); // Make temporary live long enough.
                let entry_file_name = entry_file_name.to_str().unwrap_or("");
                if self.marker_name == entry_file_name {
                    let entry_is_file = entry
                        .file_type()
                        .and_then(|ft| Ok(ft.is_file()))
                        .unwrap_or_else(|error| {
                            warn!(target: "FileSystemCrawler", "{}", &error);
                            false
                        });

                    if entry_is_file {
                        marker_file_child_index = Some(index)
                    }
                }
            }
        });

        Some(DirDescriptor {
            dir: dir,
            children: children,
            marker_file_child_index: marker_file_child_index,
            subdir_child_indexes: subdir_child_indexes,
        })
    }

    pub fn crawl_dirs(&self, mut dirs_to_visit: PathList) -> DirDescriptorMap {
        dirs_to_visit = dirs_to_visit
            .into_iter()
            .filter(|dir| self.is_crawlable_dir(&dir))
            .collect();

        let mut descr_map = DirDescriptorMap::new();

        while !dirs_to_visit.is_empty() {
            let (dirs_to_visit_next, descr_map_next): (PathList, DirDescriptorMap) = dirs_to_visit
                .into_par_iter()
                .filter_map(|dir| {
                    let dir_already_crawled = descr_map.contains_key(&dir);
                    if !dir_already_crawled {
                        self.crawl_dir(dir)
                    } else {
                        None
                    }
                })
                .fold(
                    || (PathList::new(), DirDescriptorMap::new()),
                    |(mut dirs_to_visit_group, mut descr_map_group), descr| {
                        descr.for_each_sub_direntry(|entry| dirs_to_visit_group.push(entry.path()));
                        descr_map_group.insert(descr.dir.clone(), descr);
                        (dirs_to_visit_group, descr_map_group)
                    },
                )
                .reduce(
                    || (PathList::new(), DirDescriptorMap::new()),
                    |(mut dirs_to_visit_final, mut descr_map_final),
                     (mut dirs_to_visit_group, descr_map_group)| {
                        dirs_to_visit_final.append(&mut dirs_to_visit_group);
                        descr_map_group.into_iter().for_each(|(dir, descr)| {
                            descr_map_final.insert(dir, descr);
                        });
                        (dirs_to_visit_final, descr_map_final)
                    },
                );

            descr_map_next.into_iter().for_each(|(dir, descr)| {
                descr_map.insert(dir, descr);
            });

            dirs_to_visit = dirs_to_visit_next
        }

        descr_map
    }

    fn is_crawlable_dir(&self, dir: &PathBuf) -> bool {
        if !dir.is_dir() {
            return false;
        }

        if self.exclude_dirs.iter().any(
            |pattern| dir.ends_with(pattern),
        )
        {
            return false;
        }

        self.dereference_symlinks ||
            !dir.symlink_metadata()
                .and_then(|md| Ok(md.file_type().is_symlink()))
                .unwrap_or_else(|error| {
                    warn!(target: "FileSystemCrawler", "{}", &error);
                    false
                })
    }
}
