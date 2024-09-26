use std::fs::File;
use std::io::Write;

fn save_data(mut file: &File, data: &str) -> std::io::Result<()> {
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn main() {
    let file = File::create("test.txt").unwrap();
    let content = String::from("Hello world");

    save_data(&file, &content).unwrap();
}
