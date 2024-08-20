mod buffer;
mod editor;
mod status_bar;
mod terminal;

use editor::Editor;

fn main() {
    let filename = "untitled.txt";
    let initial_text = "holas\r\nndo cruel\r\nerro";

    let mut editor = Editor::new(filename, initial_text);
    editor.run();
    editor.exit();
}
