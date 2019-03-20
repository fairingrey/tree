#[macro_use]
extern crate lazy_static;

use exitfailure::ExitFailure;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use regex::Regex;

pub mod args;

lazy_static! {
    pub static ref RE_HIDDEN_FILENAME: Regex = Regex::new(r"^\..*$").unwrap();
}

pub fn write_tree(path: PathBuf, mut handle: impl Write) -> Result<(), ExitFailure> {
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        println!("{}", entry?.file_name().to_str().unwrap());
    }
    Ok(())
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    dbg!(entry.file_name());
    entry.file_name()
        .to_str()
        .map(|s| RE_HIDDEN_FILENAME.is_match(s))
        .unwrap_or(false)
}
