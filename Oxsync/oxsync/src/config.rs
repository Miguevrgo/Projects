use std::path::{Path, PathBuf};

pub struct Config {
    pub directory_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let curr_dir = std::env::current_exe().unwrap();
        Config {
            directory_path: curr_dir,
        }
    }
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        Self {
            directory_path: path,
        }
    }
}
