mod buffer;
mod editor;
mod status_bar;
mod style;
mod syntax;
mod terminal;

use editor::Editor;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} [filename]", args[0]);
        std::process::exit(1);
    }

    let filename = if args.len() == 2 {
        args[1].clone()
    } else {
        String::from("untitled.txt")
    };

    let file_path = Path::new(&filename);

    if !file_path.exists() {
        fs::File::create(&filename)?;
    }

    let editor = Editor::new(&filename);
    let mut unwrapped_editor = editor.unwrap();
    unwrapped_editor.run()?;
    unwrapped_editor.exit()?;

    Ok(())
}
