use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType},
};
use std::io::stdout;

const COLOR_MODE_INSERT: Color = Color::Rgb {
    r: 97,
    g: 175,
    b: 239,
};
const COLOR_MODE_NORMAL: Color = Color::Rgb {
    r: 152,
    g: 195,
    b: 119,
};

const LIGHT_GRAY_BACKGROUND: Color = Color::Rgb {
    r: 59,
    g: 63,
    b: 76,
};

const GRAY_BACKGROUND: Color = Color::Rgb {
    r: 49,
    g: 53,
    b: 63,
};

/// The 'StatusBar' struct represents the status bar in the lower part of the editor
pub struct StatusBar {
    filename: String,
    mode: String,
}

impl StatusBar {
    /// Creates a new 'StatusBar' instance.
    ///
    /// # Arguments
    ///
    /// * 'filename' - The name of the file being edited
    /// #TODO: Change filename to path from root directory back
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            mode: "INSERT".to_string(),
        }
    }

    /// Updates the status message shown in the status bar.
    ///
    /// # Arguments
    ///
    /// * 'status' - The new status message to display
    pub fn update(&mut self, mode: &str) {
        self.mode = mode.to_string();
    }

    /// Renders the status bar on the bottom of the screen,
    /// This involves printing Styled mode and filename
    pub fn render(&self) {
        let (cols, rows) = size().unwrap();
        let arrow = "";
        let status_bar_content = format!(" {} {}  File  {}", self.mode, arrow, self.filename);

        let status_bar_length = status_bar_content.len() - 2;
        let padding = if status_bar_length < cols as usize {
            " ".repeat(cols as usize - status_bar_length)
        } else {
            format!("{}", status_bar_length)
        };

        let status_bar = format!(
            "{}{}{}{}{}",
            format!(" {} ", self.mode)
                .with(GRAY_BACKGROUND)
                .on(if self.mode == "INSERT" {
                    COLOR_MODE_INSERT
                } else {
                    COLOR_MODE_NORMAL
                })
                .bold(),
            arrow
                .with(if self.mode == "INSERT" {
                    COLOR_MODE_INSERT
                } else {
                    COLOR_MODE_NORMAL
                })
                .on(LIGHT_GRAY_BACKGROUND),
            " File ".with(Color::White).on(LIGHT_GRAY_BACKGROUND),
            arrow.with(LIGHT_GRAY_BACKGROUND).on(GRAY_BACKGROUND),
            format!(" {} {}", self.filename, padding)
                .with(Color::White)
                .on(GRAY_BACKGROUND),
        )
        .on(Color::Grey);

        execute!(
            stdout(),
            MoveTo(0, rows - 1),
            Clear(ClearType::CurrentLine),
            PrintStyledContent(status_bar),
        )
        .unwrap();
    }
}
