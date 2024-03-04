use std::{marker::PhantomData, path::Display, str::FromStr};

use argh::FromArgValue;

use std::slice::Iter;

use crossterm::style::ContentStyle;

use crate::minecraft::responses::MinecraftResponse;

#[derive(Clone)]
pub enum Game {
    MINECRAFT,
    FACTORIO,
    RUST,
    SOURCE, //hopefully acceptable for now to stick to a generic implementation for valve source games
    GENERIC,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::MINECRAFT => write!(f, "Minecraft"),
            Game::FACTORIO => write!(f, "Factorio"),
            Game::RUST => write!(f, "Rust"),
            Game::SOURCE => write!(f, "Source"),
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
            "factorio" => return Ok(Game::FACTORIO),
            "rust" => return Ok(Game::RUST),
            "source" => return Ok(Game::SOURCE),
            _ => Err(ParseGameError),
        }
    }
}

pub trait Response<T> {
    fn get_id_string(response: &T) -> &'static str
    where
        Self: Sized;
    ///Return an iterator of all values of response besides default
    fn iterator() -> Iter<'static, T>
    where
        Self: Sized;
    // These two are externally used.
    fn from_response_str(response: &str) -> T
    where
        Self: Sized;
    fn get_output(&self, response: String) -> Vec<(String, ContentStyle)>;
}

pub trait GameMap {
    const COMMANDS: Vec<String>;
    type ResponseType;
    fn get_response(res_string: &str) -> Box<dyn Response<Self::ResponseType>>;
    fn get_commands() -> Vec<String>;
}

//maps various calls to the appropriate commands or responses of a given game
pub struct GameMapper<Game, Res> {
    game: PhantomData<Game>,
    res: PhantomData<Res>,
}

impl<Game, Res> GameMapper<Game, Res>
where
    Res: Response<Res>,
    Game: GameMap,
{
    pub fn new() -> Self {
        Self {
            game: PhantomData,
            res: PhantomData,
        }
    }

    pub fn get_response(response: &str) -> Vec<(String, ContentStyle)> {
        let parsed_response = Res::from_response_str(response);
        return parsed_response.get_output(response.to_string());
    }

    pub fn get_commands() -> Vec<String> {
        Game::get_commands()
    }
}
