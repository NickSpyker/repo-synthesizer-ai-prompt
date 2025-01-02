use crate::{config::Config, parser::Args};
use std::{
    ffi::OsStr,
    path::{Component, Path},
};
use walkdir::DirEntry;

pub fn is_valid_folder(entry: &DirEntry, config: &Config) -> bool {
    let path: &Path = entry.path();

    for component in path.components() {
        if let Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                if config
                    .ignored_folders
                    .iter()
                    .any(|ignored: &String| ignored == name_str)
                {
                    return false;
                }
            }
        }
    }

    true
}

pub fn is_valid_file(entry: &DirEntry, config: &Config) -> bool {
    let path: &Path = entry.path();
    if !path.is_file() {
        return true;
    }

    let file_name: &str = path
        .file_name()
        .and_then(|name: &OsStr| name.to_str())
        .unwrap_or("");

    !config
        .ignored_files
        .iter()
        .any(|ignored: &String| ignored == file_name)
}

pub fn is_valid_extension(entry: &DirEntry, args: &Args) -> bool {
    entry
        .path()
        .extension()
        .map_or(true, |name: &OsStr| args.is_valid_extension(name))
}
