use exitfailure::ExitFailure;
use std::io::{stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tree::{self, Counts};

fn main() -> Result<(), ExitFailure> {
    let opt = tree::args::Opt::from_args();

    let path: PathBuf = match opt.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };

    let stdout = stdout();
    let mut handle = stdout.lock();

    let mut counts: Counts = Default::default();

    tree::walk_tree(&mut handle, path, "", &mut counts)?;

    writeln!(
        handle,
        "\n{} directories, {} files",
        counts.dirs, counts.files
    )?;

    Ok(())
}
