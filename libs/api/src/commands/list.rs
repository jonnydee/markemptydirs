use super::*;

use rayon::prelude::*;


#[derive(PartialEq, Debug)]
pub enum ListFilter {
    Clashing,
    Correct,
    Missing,
}

#[derive(PartialEq, Debug)]
pub struct List {
    pub filter: Vec<ListFilter>,
    pub root_dirs: PathList,
}

#[derive(Debug)]
struct ListStatistics {
    pub dir: PathBuf,
    pub marker_found: bool,
    pub marker_required: bool,
    pub child_count: usize,
    pub dir_count: usize,
}

impl List {
    pub fn new() -> List {
        List {
            filter: vec![],
            root_dirs: default::root_dirs(),
        }
    }
}

impl ICommand for List {
    fn execute(&self, ctx: &Context) -> Result<()> {
        let mut statistics_list: Vec<_> = ctx.crawl_dirs(&self.root_dirs)
            .into_par_iter()
            .map(|descr| {
                ListStatistics {
                    marker_found: descr.has_marker(),
                    marker_required: !descr.has_children(),
                    child_count: descr.get_child_count(),
                    dir_count: descr.get_sub_directory_count(),
                    dir: match Context::get_relative_dir_to_current_dir(&descr.dir) {
                        Ok(Some(dir)) => dir,
                        _ => descr.dir,
                    },
                }
            })
            .collect();

        statistics_list
            .as_mut_slice()
            .par_sort_unstable_by_key(|stat| stat.dir.clone());

        for stat in statistics_list {
            println!("{:?}", stat);
        }

        Ok(())
    }
}
