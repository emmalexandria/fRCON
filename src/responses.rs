use std::slice::Iter;

use crossterm::style::{Attribute, ContentStyle, Stylize};

#[derive(Clone)]
pub enum Response {
    UnknownCommand,
    PlayerNotFound,
    ListResponse,
    Default,
}

impl Response {
    fn get_id_string(response: &Response) -> &'static str {
        return match response {
            Response::UnknownCommand => "Unknown or incomplete command, see below for error",
            Response::PlayerNotFound => "No player was found",
            Response::ListResponse => "There are",
            Response::Default => "",
        };
    }

    fn iterator() -> Iter<'static, Response> {
        [
            Response::UnknownCommand,
            Response::PlayerNotFound,
            Response::ListResponse,
        ]
        .iter()
    }

    pub fn get_from_response_str(response: &str) -> Response {
        for res in Response::iterator() {
            let id_str = Response::get_id_string(res);
            if response.len() >= id_str.len() {
                if &response[0..id_str.len()] == id_str {
                    return res.clone();
                }
            }
        }

        return Response::Default;
    }

    pub fn get_output(&self, response: String) -> Vec<(String, ContentStyle)> {
        let id_str = Response::get_id_string(&self);
        match self {
            Response::UnknownCommand => {
                let mut response_lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_at(id_str.len());
                response_lines.push((sections.0.to_string(), ContentStyle::new().red().bold()));
                response_lines.push((sections.1.to_string(), ContentStyle::new().red()));

                return response_lines;
            }
            Response::ListResponse => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_once(":").unwrap();
                lines.push((sections.0.to_string(), ContentStyle::new().bold()));
                lines.push((
                    sections.1.trim().to_string(),
                    ContentStyle::new()
                        .attribute(Attribute::NoBold)
                        //once again, Attribute::NoBold seems to add a random underline
                        .attribute(Attribute::NoUnderline),
                ));

                return lines;
            }
            Response::PlayerNotFound => return vec![(response, ContentStyle::new().red())],
            _ => return vec![(response, ContentStyle::new().white())],
        }
    }
}