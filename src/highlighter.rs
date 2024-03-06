use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

pub struct RCONHighlighter {
    commands: Vec<String>,
    command_style: Style,
    neutral_style: Style,
    nomatch_style: Style,
}

impl Highlighter for RCONHighlighter {
    fn highlight(&self, line: &str, cursor: usize) -> reedline::StyledText {
        let mut styled_text = StyledText::new();

        if self.commands.clone().iter().any(|x| line.contains(x)) {
            let matches: Vec<&str> = self
                .commands
                .iter()
                .filter(|c| line.contains(*c))
                .map(std::ops::Deref::deref)
                .collect();
            let longest_match = matches.iter().fold("".to_string(), |acc, &item| {
                if item.len() > acc.len() {
                    item.to_string()
                } else {
                    acc
                }
            });
            let buffer_split: Vec<&str> = line.splitn(2, &longest_match).collect();

            styled_text.push((self.neutral_style, buffer_split[0].to_string()));
            styled_text.push((self.command_style, longest_match));
            styled_text.push((self.neutral_style, buffer_split[1].to_string()));
        } else if self.commands.is_empty() {
            styled_text.push((self.neutral_style, line.to_string()));
        } else {
            styled_text.push((self.nomatch_style, line.to_string()));
        }

        styled_text
    }
}

impl RCONHighlighter {
    pub fn new(commands: Vec<String>) -> RCONHighlighter {
        RCONHighlighter {
            commands,
            command_style: Style::new().fg(Color::LightYellow),
            neutral_style: Style::new().fg(Color::DarkGray),
            nomatch_style: Style::new().fg(Color::DarkGray),
        }
    }
}
