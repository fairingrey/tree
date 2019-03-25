#[macro_use]
extern crate lazy_static;

use crate::args::Opt;
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
    args: &Opt,
    root: P,
    prefix: &str,
    current_depth: usize,
    counts: &mut Counts,
) -> Result<(), ExitFailure> {
    if let Some(max_depth) = args.max_depth {
        if current_depth > max_depth {
            return Ok(());
        }
    }

    let mut paths = fs::read_dir(&root)?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if args.all_files || !is_hidden(&entry) {
                Some(entry)
            } else {
                None
            }
        })
        .filter(|entry| !args.only_dirs || entry.file_type().unwrap().is_dir())
        .filter(|entry| {
            if entry.path().is_file() {
                let path = entry.path();
                let filename = path.file_name().unwrap().to_str().unwrap();
                let mut will_yield = true;
                if let Some(match_pattern) = &args.match_pattern {
                    if !match_pattern.is_match(filename) {
                        will_yield = false;
                    }
                }
                if let Some(ignore_pattern) = &args.ignore_pattern {
                    if ignore_pattern.is_match(filename) {
                        will_yield = false;
                    }
                }
                will_yield
            } else {
                true
            }
        })
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    paths.sort_by(|a, b| {
        let a = a.file_name().unwrap().to_str().unwrap();
        let b = b.file_name().unwrap().to_str().unwrap();
        a.cmp(b)
    });

    let mut index = paths.len();

    for path in paths {
        let filename = path.file_name().unwrap().to_str().unwrap();

        let output_str = if args.full_paths {
            path.to_str().unwrap()
        } else {
            filename
        };

        index -= 1;

        if path.is_dir() {
            counts.dirs += 1;
        } else if path.is_file() {
            counts.files += 1;
        }

        if index == 0 {
            if path.is_dir() {
                writeln!(handle, "{}└── {}", prefix, output_str)?;
                walk_tree(
                    handle,
                    args,
                    &format!("{}/{}", &root.to_string(), filename),
                    &format!("{}    ", prefix),
                    current_depth + 1,
                    counts,
                )?;
            } else {
                writeln!(handle, "{}└── {}", prefix, output_str)?;
            }
        } else if path.is_dir() {
            writeln!(handle, "{}├── {}", prefix, output_str)?;
            walk_tree(
                handle,
                args,
                &format!("{}/{}", &root.to_string(), filename),
                &format!("{}│   ", prefix),
                current_depth + 1,
                counts,
            )?;
        } else {
            writeln!(handle, "{}├── {}", prefix, output_str)?;
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
