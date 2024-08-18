use buffer::GapBuffer;

mod buffer;
mod editor;
//mod terminal;

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    let mut buffer = GapBuffer::from_string("Hello");

    // Inicializar terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, EnableMouseCapture)?;

    loop {
        // Limpiar la pantalla y mover el cursor
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Mostrar el contenido del buffer
        buffer.print_buffer();
        stdout.flush()?;

        // Leer la entrada del usuario
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(c) => buffer.insert_char(c), // Insertar carácter
                KeyCode::Backspace => buffer.backspace(),  // Eliminar carácter antes del cursor
                KeyCode::Delete => buffer.delete(),        // Eliminar carácter después del cursor
                KeyCode::Left => buffer.cursor_left(),     // Mover cursor a la izquierda
                KeyCode::Right => buffer.cursor_right(),   // Mover cursor a la derecha
                KeyCode::Esc => break,                     // Salir del bucle con Esc
                _ => {}                                    // Ignorar otras teclas
            }
        }
    }

    // Restaurar el estado de la terminal
    execute!(stdout, DisableMouseCapture, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
