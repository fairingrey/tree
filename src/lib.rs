use exitfailure::ExitFailure;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub mod args;

pub fn write_tree(path: PathBuf, mut handle: impl Write) -> Result<(), ExitFailure> {
    let walker = WalkDir::new(path);
    for entry in walker {
        println!("{}", entry?.file_name().to_str().unwrap());
    }
    Ok(())
}
