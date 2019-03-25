use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(short = "a")]
    pub all_files: bool,

    #[structopt(short = "d")]
    pub only_dirs: bool,
}
