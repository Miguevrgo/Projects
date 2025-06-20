use std::path::PathBuf;

mod config;
mod watcher;

fn main() {
    let path = std::path::Path::new("/home/miguevr/Oxsync");
    let config = config::Config::new(path.to_path_buf());

    println!("Watching folder: {:?}", path.to_str())
}
