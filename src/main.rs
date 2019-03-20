use exitfailure::ExitFailure;
use std::io::stdout;
use std::path::PathBuf;
use structopt::StructOpt;
use tree;

fn main() -> Result<(), ExitFailure> {
    let opt = tree::args::Opt::from_args();

    let path: PathBuf = match opt.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };

    let stdout = stdout();
    let handle = stdout.lock();

    tree::write_tree(path, handle)?;

    Ok(())
}
