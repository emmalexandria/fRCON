use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

use crate::games::Game;

pub struct RCONHighlighter {
    commands: Vec<String>,
    is_generic: bool,
    command_style: Style,
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
            } else if i == 0 && !self.is_generic {
                styled_text.push((self.nomatch_style, word.to_string()));
            } else {
                styled_text.push((self.neutral_style, word.to_string()));
            }
        }

        styled_text
    }
}

impl RCONHighlighter {
    pub fn new(commands: Vec<String>, game: Game) -> RCONHighlighter {
        RCONHighlighter {
            commands,
            is_generic: game == Game::GENERIC,
            command_style: Style::new().fg(Color::LightYellow),
            neutral_style: Style::new().fg(Color::LightGray),
            nomatch_style: Style::new().fg(Color::Red),
        }
    }
}
