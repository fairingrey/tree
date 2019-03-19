use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let path: PathBuf = match args.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };

    let walker = WalkDir::new(path);
    for entry in walker {
        println!("{}", entry?.path().display());
    }
    Ok(())
}
