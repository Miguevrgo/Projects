use core::fmt::Display;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, queue};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub _width: u16,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::set_name()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), crossterm::cursor::MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Size {
            height,
            _width: width,
        })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn print(str: impl Display) -> Result<(), Error> {
        queue!(stdout(), crossterm::style::Print(str))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn set_name() -> Result<(), Error> {
        execute!(stdout(), crossterm::terminal::SetTitle("Oxide"))?;
        Ok(())
    }
}
