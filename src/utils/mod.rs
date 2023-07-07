use globset::{Glob, GlobSetBuilder};
use ignore::{overrides::OverrideBuilder, WalkBuilder, WalkState::*};
use std::path::PathBuf;

use num_cpus;
use std::sync::mpsc;
use std::sync::Arc;

pub fn scan_files(root_path: &str, p: &str) -> Vec<PathBuf> {
    let mut include = GlobSetBuilder::new();
    include.add(Glob::new(p).unwrap());
    let include_set = include.build().unwrap();
    let arc = Arc::new(include_set);
    let mut configs: Vec<PathBuf> = Vec::new();
    let mut walk_builder: WalkBuilder = WalkBuilder::new(&root_path);
    walk_builder.hidden(false);

    let mut override_builder = OverrideBuilder::new(&root_path);
    override_builder.add("!**/.git").unwrap();

    if let Ok(overrides) = override_builder.build() {
        walk_builder.overrides(overrides);
    }

    let (tx, rx) = mpsc::channel();
    let parallel_walker = walk_builder.threads(num_cpus::get() * 2).build_parallel();

    parallel_walker.run(|| {
        let arc2 = arc.clone();
        let tx = tx.clone();
        Box::new(move |result| match result {
            Ok(entry) => {
                let path = entry.path();
                if entry.path().is_file() && arc2.is_match(path) {
                    tx.send(entry.path().to_path_buf()).unwrap();
                }
                Continue
            }
            Err(err) => {
                println!("Error:{}", err);
                Quit
            }
        })
    });

    drop(tx);

    // let mut total: u64 = 0;
    for path in rx {
        // let metadata = fs::metadata(&path).expect("Failed to get file metadata");
        // let file_size = metadata.len();
        // total += file_size;
        configs.push(path);
    }

    // if total > 0 {
    //     total = total / 8 / 1024
    // }
    configs
}
