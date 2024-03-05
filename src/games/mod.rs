mod generic;
mod minecraft;

use std::str::FromStr;

use std::slice::Iter;

use crossterm::style::ContentStyle;

use minecraft::{Minecraft, MinecraftResponse};

use self::generic::Generic;

#[derive(Clone)]
pub enum Game {
    MINECRAFT,
    GENERIC,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::MINECRAFT => write!(f, "Minecraft"),
            Game::GENERIC => write!(f, "generic"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
pub struct ResponseFromStrError;

impl std::fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid game")
    }
}

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

//This trait mainly exists to make writing new implementations easier
pub trait Response<T> {
    fn get_id_string(response: &T) -> &'static str;
    ///Return an iterator of all values of response besides default
    fn iterator() -> Iter<'static, T>;
    // These two are externally used.
    fn from_response_str(response: &str) -> T;
    fn get_output(response: &str) -> Vec<(String, ContentStyle)>;
}

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
