use std::fs::{remove_file, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

mod database;

fn save_data(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let temp_path = PathBuf::from(format!("{}.temp", path.display()));
    let temp_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)?;

    let mut writer = BufWriter::new(temp_file);
    writer.write_all(data)?;
    writer.flush()?;
    writer.get_ref().sync_data()?;

    std::fs::rename(&temp_path, path)?;
    if temp_path.exists() {
        remove_file(&temp_path)?;
    }

    Ok(())
}

fn main() {
    let file_path = Path::new("test");
    let content = b"hello world";
    save_data(file_path, content).unwrap();
}
