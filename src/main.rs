use exitfailure::ExitFailure;
use std::io::{stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tree::{self, Counts};

fn main() -> Result<(), ExitFailure> {
    let opt = tree::args::Opt::from_args();

    let root: PathBuf = match opt.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };
    let mut counts: Counts = Default::default();

    let stdout = stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "{}", root.to_str().unwrap())?;

    tree::walk_tree(&mut handle, root.to_str().unwrap(), "", opt.all_files, opt.only_dirs, &mut counts)?;

    writeln!(
        handle,
        "\n{} directories, {} files",
        counts.dirs, counts.files
    )?;

    Ok(())
}
