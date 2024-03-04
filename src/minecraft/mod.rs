use crate::game_mapper::GameMap;

use self::responses::MinecraftResponse;

pub mod responses;

const MINECRAFT_COMMANDS: Vec<String> = vec![
    "give".to_string(),
    "locate".to_string(),
    "kill".to_string(),
    "summon".to_string(),
];

pub struct Minecraft {}

impl GameMap for Minecraft {
    const COMMANDS: Vec<String> = MINECRAFT_COMMANDS;

    type ResponseType = MinecraftResponse;

    fn get_response(command: &str) -> Box<dyn crate::game_mapper::Response<MinecraftResponse>> {
        todo!()
    }

    fn get_commands() -> Vec<String> {
        return Self::COMMANDS;
    }
}
