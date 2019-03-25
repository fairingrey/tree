#[macro_use]
extern crate lazy_static;

use exitfailure::ExitFailure;
use regex::Regex;
use std::fs::{self, DirEntry};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub mod args;

lazy_static! {
    pub static ref RE_HIDDEN_FILENAME: Regex = Regex::new(r"^\..+$").unwrap();
}

#[derive(Debug, Default)]
pub struct Counts {
    pub dirs: isize,
    pub files: isize,
}

// some code taken from https://github.com/kddeisz/tree/blob/master/tree.rs
pub fn walk_tree<P: AsRef<Path> + ToString>(
    handle: &mut impl Write,
    root: P,
    prefix: &str,
    all_files: bool,
    only_dirs: bool,
    current_depth: usize,
    max_depth: Option<usize>,
    counts: &mut Counts,
) -> Result<(), ExitFailure> {
    if let Some(max_depth) = max_depth {
        if current_depth > max_depth {
            return Ok(());
        }
    }

    let mut paths = fs::read_dir(&root)?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if all_files || !is_hidden(&entry) {
                if !only_dirs || entry.file_type().unwrap().is_dir() {
                    Some(entry.path())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    paths.sort_by(|a, b| {
        let a = a.file_name().unwrap().to_str().unwrap();
        let b = b.file_name().unwrap().to_str().unwrap();
        a.cmp(b)
    });

    let mut index = paths.len();

    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        index -= 1;

        if path.is_dir() {
            counts.dirs += 1;
        } else if path.is_file() {
            counts.files += 1;
        }

        if index == 0 {
            writeln!(handle, "{}└── {}", prefix, name)?;
            if path.is_dir() {
                walk_tree(
                    handle,
                    &format!("{}/{}", &root.to_string(), name),
                    &format!("{}    ", prefix),
                    all_files,
                    only_dirs,
                    current_depth + 1,
                    max_depth,
                    counts,
                )?;
            }
        } else {
            writeln!(handle, "{}├── {}", prefix, name)?;
            if path.is_dir() {
                walk_tree(
                    handle,
                    &format!("{}/{}", &root.to_string(), name),
                    &format!("{}│   ", prefix),
                    all_files,
                    only_dirs,
                    current_depth + 1,
                    max_depth,
                    counts,
                )?;
            }
        }
    }

    Ok(())
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| RE_HIDDEN_FILENAME.is_match(s))
        .unwrap_or(false)
}
