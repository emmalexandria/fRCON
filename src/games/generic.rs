use crossterm::style::{Attribute, ContentStyle, Stylize};

pub struct Generic;

impl Generic {
    pub fn get_commands() -> Vec<String> {
        return vec![];
    }
}

pub fn get_output(response: &str) -> Vec<(String, ContentStyle)> {
    return vec![(
        response.to_string(),
        ContentStyle::new().attribute(Attribute::Reset),
    )];
}
