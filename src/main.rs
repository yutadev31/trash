use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Parser, builder::TypedValueParser};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(required = true, value_parser = clap::builder::OsStringValueParser::new().try_map(parse_path))]
    pub paths: Vec<PathBuf>,
}

fn parse_path(value: std::ffi::OsString) -> Result<PathBuf, String> {
    let last_component = value
        .as_encoded_bytes()
        .rsplit(|byte| *byte == b'/')
        .find(|component| !component.is_empty());

    match last_component {
        None | Some(b"." | b"..") => {
            Err("refusing to trash '.', '..', or the root directory".into())
        }
        Some(_) => Ok(PathBuf::from(value)),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_special_paths_before_deletion() {
        for path in [
            "", ".", "..", "./", "../", "dir/.", "dir/..", "dir/..//", "/", "///",
        ] {
            assert!(
                Cli::try_parse_from(["trash", "valid-file", path]).is_err(),
                "{path:?}"
            );
        }
    }

    #[test]
    fn accepts_regular_paths() {
        for path in [
            "file",
            "./file",
            "../file",
            "/tmp/file",
            "dir/",
            ".hidden",
            "...",
        ] {
            let cli = Cli::try_parse_from(["trash", path]).unwrap();

            assert_eq!(cli.paths, [PathBuf::from(path)]);
        }
    }
}
