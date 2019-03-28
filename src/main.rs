use exitfailure::ExitFailure;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tree::{self, Counts};

fn main() -> Result<(), ExitFailure> {
    let args = tree::args::Opt::from_args();

    let root: PathBuf = match &args.path {
        Some(path) => path.to_path_buf(),
        None => PathBuf::from("."),
    };
    let mut counts: Counts = Default::default();

    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut handle = stdout.lock();

    handle.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    write!(handle, "{}", root.to_str().unwrap())?;

    tree::walk_tree(
        &mut handle,
        &args,
        root.to_str().unwrap(),
        "",
        1,
        &mut counts,
    )?;

    if !args.no_report {
        writeln!(
            handle,
            "\n{} directories, {} files",
            counts.dirs, counts.files
        )?;
    }

    Ok(())
}
