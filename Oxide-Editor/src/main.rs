mod buffer;
mod editor;
mod status_bar;
mod terminal;

use editor::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: Oxide <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let file_path = std::path::Path::new(filename);

    if !std::path::Path::is_file(file_path) {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }
    let mut editor = Editor::new(filename);
    editor.run();
    editor.exit();
}
