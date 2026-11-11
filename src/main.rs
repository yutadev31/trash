use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut failed = false;

    for path in &cli.paths {
        if let Err(err) = trash_path(path) {
            eprintln!("trash: {}: {err}", path.display());
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn trash_path(path: &Path) -> Result<(), trash_rs::Error> {
    trash_rs::delete(path)
}
