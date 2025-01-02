use crate::config::Config;
use clap::{error::ErrorKind, Parser};
use std::{
    collections::HashSet,
    env,
    ffi::OsStr,
    fs::{self, File},
    path::PathBuf,
    process,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Specifies the directory to analyze. If not provided, uses the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// Path to the output file. If not provided, prints to stdout
    #[arg(short, long)]
    pub output_file: Option<String>,

    /// List of file extensions to process. Only files with these extensions will be included
    #[arg(short, long)]
    extensions: Option<Vec<String>>,

    /// List of file extensions to ignore. Files with these extensions will be excluded
    #[arg(short, long)]
    ignore: Option<Vec<String>>,

    /// Opens the default config file in the system's file explorer
    #[arg(short, long)]
    config: bool,
}

impl Args {
    pub fn parse(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut args: Self = <Args as clap::Parser>::parse();

        if args.config {
            Config::open_in_file_explorer()?;
            process::exit(0);
        }

        if let Some(ignore) = &mut args.ignore {
            ignore.extend_from_slice(config.ignored_extensions.as_slice());
        } else {
            args.ignore = Some(config.ignored_extensions.clone());
        }

        if let (Some(target), Some(ignore)) = (&args.extensions, &args.ignore) {
            let no_common_elements: bool = target
                .iter()
                .collect::<HashSet<_>>()
                .is_disjoint(&ignore.iter().collect());

            if !no_common_elements {
                clap::Error::raw(
                    ErrorKind::ArgumentConflict,
                    "The extensions and ignore list must not overlap",
                )
                .exit();
            }
        }

        Ok(args)
    }

    pub fn directory(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        self.directory.clone().map_or_else(
            || env::current_dir().map_err(|_| "Failed to get current directory".into()),
            Ok,
        )
    }

    pub fn output_file(&self) -> Option<File> {
        let open_or_create = |file: String| {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file)
                .ok()
        };

        self.output_file.clone().and_then(open_or_create)
    }

    pub fn is_valid_extension(&self, extension: &OsStr) -> bool {
        if let Some(valid_extensions) = &self.extensions {
            valid_extensions
                .iter()
                .any(|valid_ext: &String| extension == valid_ext.as_str())
        } else if let Some(invalid_extensions) = &self.ignore {
            !invalid_extensions
                .iter()
                .any(|invalid_ext: &String| extension == invalid_ext.as_str())
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::CommandFactory;

    #[test]
    fn test_args_parsing() {
        Args::command().debug_assert()
    }
}
