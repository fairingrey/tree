#[macro_use]
extern crate lazy_static;

use exitfailure::ExitFailure;
use regex::Regex;
use std::io::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

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
    prefix: &str,
    counts: &mut Counts,
) -> Result<(), ExitFailure> {
    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e));

    for entry in walker {
        let entry = entry?;
        let filename = entry.file_name().to_str().unwrap();

        if entry.file_type().is_dir() {
            counts.dirs += 1;
        } else if entry.file_type().is_file() {
            counts.files += 1;
        }
        writeln!(handle, "{}", entry.file_name().to_str().unwrap())?;
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
