use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::{Position, Size, Terminal};
mod terminal;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor::default()
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        Terminal::print("Welcome to Oxide")?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('x') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Bye\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            print!("~");
            if row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}
