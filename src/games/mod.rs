mod generic;
mod minecraft;

use std::str::FromStr;

use std::slice::Iter;

use crossterm::style::ContentStyle;

use minecraft::{Minecraft, MinecraftResponse};

use self::generic::Generic;

///Game selection enum. Used in GameMapper and for command line arguments.
#[derive(Clone, PartialEq)]
pub enum Game {
    MINECRAFT,
    GENERIC,
}

///Required for argh
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::MINECRAFT => write!(f, "Minecraft"),
            Game::GENERIC => write!(f, "generic"),
        }
    }
}

///Required for argh
#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameError;

impl std::fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid game")
    }
}

///Required for argh
impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "minecraft" => return Ok(Game::MINECRAFT),
            "generic" => return Ok(Game::GENERIC),
            _ => Err(ParseGameError),
        }
    }
}

///A trait for implementing new response types. This is not a *necessary* trait to implement on new response types, but it
/// contains everything used to implement the current iterator based technique
pub trait Response<T> {
    fn get_id_string(response: &T) -> &'static str;
    fn iterator() -> Iter<'static, T>;
    fn from_response_str(response: &str) -> T;
    fn get_output(response: &str) -> Vec<(String, ContentStyle)>;
}

///Returns function references for getting the command list and getting formatted responses based on the currently selected game
pub struct GameMapper;

impl GameMapper {
    pub fn get_command_fn(game: &Game) -> &'static dyn Fn() -> Vec<String> {
        match game {
            Game::MINECRAFT => return &Minecraft::get_commands,
            Game::GENERIC => return &Generic::get_commands,
        }
    }

    pub fn get_response_fn(game: &Game) -> &'static dyn Fn(&str) -> Vec<(String, ContentStyle)> {
        match game {
            Game::MINECRAFT => return &MinecraftResponse::get_output,
            Game::GENERIC => return &generic::get_output,
        }
    }
}
