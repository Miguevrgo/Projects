mod buffer;
mod editor;
mod status_bar;
mod terminal;

use editor::Editor;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let file_path = std::path::Path::new(filename);

        if !std::path::Path::is_file(file_path) {
            std::fs::File::create(filename)?;
        }
        let mut editor = Editor::new(filename);
        editor.run();
        editor.exit();
    }

    Ok(())
}
