use exitfailure::ExitFailure;
use std::io::{stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tree::{self, Counts};

fn main() -> Result<(), ExitFailure> {
    let args = tree::args::Opt::from_args();

    let root: PathBuf = match &args.path {
        Some(path) => path.to_path_buf(),
        None => PathBuf::from("."),
    };
    let mut counts: Counts = Default::default();

    let stdout = stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "{}", root.to_str().unwrap())?;

    tree::walk_tree(
        &mut handle,
        &args,
        root.to_str().unwrap(),
        "",
        1,
        &mut counts,
    )?;

    writeln!(
        handle,
        "\n{} directories, {} files",
        counts.dirs, counts.files
    )?;

    Ok(())
}
