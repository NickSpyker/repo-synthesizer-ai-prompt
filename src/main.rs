mod config;
mod formatter;
mod parser;
mod validator;

use crate::config::Config;
use parser::Args;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config: Config = Config::load_or_create()?;
    let args: Args = Args::parse(&config)?;

    if let Some(output_file_name) = &args.output_file {
        config.add_output_file_to_ignored_files(output_file_name);
    }

    let mut output_file: Option<File> = args.output_file();
    let root: PathBuf = args.directory()?;

    let entries: Vec<DirEntry> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry: &DirEntry| validator::is_valid_folder(entry, &config))
        .filter(|entry: &DirEntry| validator::is_valid_file(entry, &config))
        .filter(|entry: &DirEntry| validator::is_valid_extension(entry, &args))
        .collect();

    let mut is_first: bool = true;
    for entry in entries.into_iter() {
        let relative_path: &Path = match entry.path().strip_prefix(&root) {
            Ok(path) => path,
            Err(_) => return Err("Failed to get relative path".into()),
        };

        if let Ok(content) = fs::read_to_string(entry.path()) {
            let content: String = formatter::format(is_first, relative_path, content.trim());
            match &mut output_file {
                Some(file) => writeln!(file, "{content}")?,
                None => println!("{content}"),
            }
            is_first = false;
        }
    }

    Ok(())
}
