#[macro_use]
extern crate lazy_static;

use exitfailure::ExitFailure;
use regex::Regex;
use std::io::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir, Error as WalkError};

pub mod args;

lazy_static! {
    pub static ref RE_HIDDEN_FILENAME: Regex = Regex::new(r"^\..+$").unwrap();
}

#[derive(Debug, Default)]
pub struct Counts {
    pub dirs: isize,
    pub files: isize,
}

pub fn walk_tree<P: AsRef<Path>>(
    handle: &mut impl Write,
    path: P,
    counts: &mut Counts,
) -> Result<(), ExitFailure> {
    let walker = WalkDir::new(path)
        .sort_by(|a,b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .collect::<Result<Vec<DirEntry>, WalkError>>()?;

    for (i, entry) in walker.iter().enumerate() {
        let filename = entry.file_name().to_str().unwrap();
        let depth = entry.depth();

        if entry.file_type().is_dir() {
            counts.dirs += 1;
        } else if entry.file_type().is_file() {
            counts.files += 1;
        }

        if depth == 0 {
            writeln!(handle, "[{}] {}", &depth.to_string(), filename)?;
            continue;
        }

        for d in 1..=depth {
            let lookahead = &walker[i..walker.len()].iter().any(|entry| entry.depth() == d);
            if *lookahead {
                if d == depth {
                    write!(handle, "{}", "├── ")?;
                } else {
                    write!(handle, "{}", "│   ")?;
                }
            } else {
                write!(handle, "{}", "    ")?;
            }
        }

        writeln!(handle, "[{}] {}", &depth.to_string(), filename)?;
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
