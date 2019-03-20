#[macro_use]
extern crate lazy_static;

use exitfailure::ExitFailure;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use regex::Regex;

pub mod args;

lazy_static! {
    pub static ref RE_HIDDEN_FILENAME: Regex = Regex::new(r"^\..+$").unwrap();
}

#[derive(Debug, Default)]
pub struct Counts {
    dirs: usize,
    files: isize,
}

pub fn write_tree(path: PathBuf, mut handle: impl Write) -> Result<(), ExitFailure> {
    let walker = WalkDir::new(path).into_iter();
    let mut counts: Counts = Default::default();
    for entry in walker.filter_entry(|e| !is_hidden(e)).peekable() {
        let entry = entry?;

        if entry.file_type().is_dir() {
            counts.dirs += 1;
        } else if entry.file_type().is_file() {
            counts.files += 1;
        }

        write!(handle, "{}\n", entry.file_name().to_str().unwrap())?;
    }
    write!(handle, "\n{} directories, {} files\n", counts.dirs, counts.files)?;
    Ok(())
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| RE_HIDDEN_FILENAME.is_match(s))
        .unwrap_or(false)
}
