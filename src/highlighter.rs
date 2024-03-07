use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};
use tokio::io::split;

pub struct RCONHighlighter {
    commands: Vec<String>,
    command_style: Style,
    hint_style: Style,
    neutral_style: Style,
    nomatch_style: Style,
}

impl Highlighter for RCONHighlighter {
    fn highlight(&self, line: &str, cursor: usize) -> reedline::StyledText {
        let mut styled_text = StyledText::new();

        let words: Vec<&str> = line.split_inclusive(" ").collect();
        for (i, word) in words.iter().enumerate() {
            if self.commands.contains(&word.trim().to_string()) && i == 0 {
                styled_text.push((self.command_style, word.to_string()));
            } else {
                styled_text.push((self.neutral_style, word.to_string()));
            }
        }

        styled_text
    }
}

impl RCONHighlighter {
    pub fn new(commands: Vec<String>) -> RCONHighlighter {
        RCONHighlighter {
            commands,
            command_style: Style::new().fg(Color::LightYellow),
            hint_style: Style::new().fg(Color::LightGray),
            neutral_style: Style::new().fg(Color::DarkGray),
            nomatch_style: Style::new().fg(Color::Red),
        }
    }
}
