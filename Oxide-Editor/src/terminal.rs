use std::io::{self, stdout};

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
    pub fn init() -> Result<(), io::Error> {
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        execute!(stdout(), SetTitle("Oxide"))?;
        enable_raw_mode()?;
        Ok(())
    }

    /// Restores the terminal to its original state
    pub fn restore() -> Result<(), io::Error> {
        execute!(stdout(), Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Clears the terminal screen and moves the cursor to (0,0)
    pub fn clear_screen() -> Result<(), io::Error> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }

    /// Renders the given text to the terminal without any formatting
    /// or cursor movement
    ///
    /// # Arguments
    ///
    /// * `text`- The text to render to the terminal with type
    ///
    pub fn render_text(text: &str) -> Result<(), io::Error> {
        execute!(stdout(), Print(text))?;
        Ok(())
    }

    pub fn read_key() -> Result<Option<KeyCode>, io::Error> {
        if let Event::Key(event) = read()? {
            Ok(Some(event.code))
        } else {
            Ok(None)
        }
    }

    /// Prints a command box at the top of the terminal with the given text
    /// and returns the x and y position of the cursor after the box
    /// has been printed.
    ///
    /// # Arguments
    ///
    /// * `text`- The text to render in the command box
    ///
    pub fn print_command_box(text: &str) -> Result<(u16, u16), io::Error> {
        let term_size = crossterm::terminal::size()?;
        let length = term_size.0 as usize / 2;
        let content_down = "─".repeat(length);
        let content = "─".repeat(length.saturating_sub(16) / 2);
        let content = format!("{}| Command Mode |{}", content, content);
        let content_middle = format!("{:<width$}", text, width = length);

        let x_position = (term_size.0 / 2).saturating_sub(length as u16 / 2);

        execute!(
            stdout(),
            MoveTo(x_position, 1),
            PrintStyledContent(format!("╭{}╮", content).with(Color::Cyan)),
            MoveTo(x_position, 2),
            PrintStyledContent(format!("│{}│", content_middle).with(Color::Cyan)),
            MoveTo(x_position, 3),
            PrintStyledContent(format!("╰{}╯", content_down).with(Color::Cyan)),
            MoveTo(x_position + 1 + text.len() as u16, 2),
        )?;

        Ok((x_position + 1, 2))
    }

    pub fn size() -> Result<(u16, u16), io::Error> {
        Ok(crossterm::terminal::size()?)
    }

    pub fn show_cursor() -> Result<(), io::Error> {
        execute!(stdout(), Show)?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
}