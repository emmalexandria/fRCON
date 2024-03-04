use std::slice::Iter;

use crossterm::style::ContentStyle;

pub trait Response {
    type GameResponse;

    //These two are functions that are not necessarily required for the trait, but they are
    // functionally necessary with how I'm currently implementing response parsing
    ///Get the identifying string of a given response
    fn get_id_string(response: &Self::GameResponse) -> &'static str;
    ///Return an iterator of all values of response besides default
    fn iterator() -> Iter<'static, Self::GameResponse>;
    // These two are externally used.
    fn get_from_response_str(response: &str) -> Self::GameResponse;
    fn get_output(&self, response: String) -> Vec<(String, ContentStyle)>;
}
