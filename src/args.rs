use regex::Regex;
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

    #[structopt(short = "f")]
    pub full_paths: bool,

    #[structopt(short = "L")]
    pub max_depth: Option<usize>,

    #[structopt(short = "P")]
    pub match_pattern: Option<Regex>,

    #[structopt(short = "I")]
    pub ignore_pattern: Option<Regex>,

    // TODO
    #[structopt(long = "ignore-case")]
    pub ignore_case: bool,

    #[structopt(long = "noreport")]
    pub no_report: bool,

    // TODO
    #[structopt(long = "filelimit")]
    pub file_limit: Option<usize>,
}
