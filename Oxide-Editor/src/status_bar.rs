use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType},
};
use std::io::stdout;

pub struct StatusBar {
    filename: String,
    mode: String,
}

impl StatusBar {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            mode: "INSERT".to_string(),
        }
    }

    pub fn update(&mut self, mode: &str) {
        self.mode = mode.to_string();
    }

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
            "{}{}{}{}{}{}",
            format!(" {} ", self.mode)
                .with(Color::Rgb {
                    r: 40,
                    g: 44,
                    b: 52
                })
                .on(Color::Rgb {
                    r: 152,
                    g: 195,
                    b: 119
                })
                .bold(),
            arrow
                .with(Color::Rgb {
                    r: 152,
                    g: 195,
                    b: 119
                })
                .on(Color::Rgb {
                    r: 59,
                    g: 63,
                    b: 76
                }),
            " File ".with(Color::White).on(Color::Rgb {
                r: 59,
                g: 63,
                b: 76
            }),
            arrow
                .with(Color::Rgb {
                    r: 59,
                    g: 63,
                    b: 76
                })
                .on(Color::Rgb {
                    r: 49,
                    g: 53,
                    b: 63
                }),
            format!(" {} ", self.filename)
                .with(Color::White)
                .on(Color::Rgb {
                    r: 49,
                    g: 53,
                    b: 63
                }),
            padding.on(Color::Rgb {
                r: 49,
                g: 53,
                b: 63
            })
        )
        .on(Color::Rgb {
            r: 152,
            g: 195,
            b: 119,
        });

        execute!(
            stdout(),
            MoveTo(0, rows - 1),
            Clear(ClearType::CurrentLine),
            PrintStyledContent(status_bar),
        )
        .unwrap();
    }
}
