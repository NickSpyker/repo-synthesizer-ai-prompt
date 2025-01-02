use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
};

const DEFAULT_CONFIG: &str = include_str!("default_config.json");

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ignored_folders: Vec<String>,
    pub ignored_files: Vec<String>,
    pub ignored_extensions: Vec<String>,
}

impl Config {
    fn get_config_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "repo-synthesizer-ai-prompt")
            .map(|proj_dirs: ProjectDirs| proj_dirs.config_dir().join("config.json"))
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path: PathBuf =
            Self::get_config_path().ok_or_else(|| "Failed to determine config path".to_string())?;

        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&config_path, DEFAULT_CONFIG)?;
        }

        let content: String = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn open_in_file_explorer() -> Result<(), Box<dyn std::error::Error>> {
        let config_path: PathBuf =
            Self::get_config_path().ok_or_else(|| "Failed to determine config path".to_string())?;

        let config_dir: &Path = config_path
            .parent()
            .ok_or_else(|| "Failed to get config directory".to_string())?;

        #[cfg(target_os = "windows")]
        {
            Command::new("explorer").arg(config_dir).spawn()?;
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(config_dir).spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            if Command::new("xdg-open").arg(config_dir).spawn().is_err() {
                for cmd in ["nautilus", "dolphin", "thunar", "nemo"] {
                    if Command::new(cmd).arg(config_dir).spawn().is_ok() {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_output_file_to_ignored_files(&mut self, file_name: &str) {
        self.ignored_files.push(file_name.to_string());
    }
}
