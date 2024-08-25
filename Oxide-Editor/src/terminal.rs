use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
};

pub struct Terminal;

impl Terminal {
    /// Initializes the terminal for the editor, setting its title name to Oxide
    /// and enabling raw_mode
    pub fn init() {
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
        execute!(stdout(), SetTitle("Oxide")).unwrap();
        enable_raw_mode().unwrap();
    }

    /// Restores the terminal to its original state
    pub fn restore() {
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    /// Clears the terminal screen and moves the cursor to (0,0)
    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    }

    /// Renders the given text to the terminal without any formatting
    /// or cursor movement
    ///
    /// # Arguments
    ///
    /// * `text`- The text to render to the terminal with type
    ///
    pub fn render_text(text: &str) {
        execute!(stdout(), Print(text)).unwrap();
    }

    pub fn read_key() -> Option<KeyCode> {
        if let Event::Key(event) = read().unwrap() {
            Some(event.code)
        } else {
            None
        }
    }

    pub fn print_command_box() {
        let term_size = crossterm::terminal::size().unwrap();
        let length = term_size.0 as usize / 2;
        let content_down = "─".repeat(length);
        let content = "─".repeat(length.saturating_sub(16) / 2);
        let content = format!("{}| Command Mode |{}", content, content);
        let content_middle = " ".repeat(length);

        let x_position = (term_size.0 / 2).saturating_sub(length as u16 / 2);

        execute!(
            stdout(),
            MoveTo(x_position, 1),
            PrintStyledContent(format!("╭{}╮", content).with(Color::Cyan)),
            MoveTo(x_position, 2),
            PrintStyledContent(format!("│{}│", content_middle).with(Color::Cyan)),
            MoveTo(x_position, 3),
            PrintStyledContent(format!("╰{}╯", content_down).with(Color::Cyan)),
            MoveTo(x_position + 1, 2),
        )
        .unwrap();
    }

    pub fn show_cursor() {
        execute!(stdout(), Show).unwrap();
    }

    pub fn move_cursor_to(x: u16, y: u16) {
        execute!(stdout(), MoveTo(x, y)).unwrap();
    }
}
