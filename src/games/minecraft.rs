use regex::Regex;
use std::slice::Iter;

use crossterm::style::{Attribute, ContentStyle, Stylize};

use crate::games::Response;
pub struct Minecraft;

impl Minecraft {
    //I should probably find a way to load this from a file or something.
    const COMMANDS: [&'static str; 82] = [
        "advancement",
        "attribute",
        "ban",
        "ban-ip",
        "banlist",
        "bossbar",
        "clear",
        "clone",
        "damage",
        "data",
        "datapack",
        "debug",
        "defaultgamemode",
        "deop",
        "difficulty",
        "effect",
        "enchant",
        "execute",
        "experience",
        "fill",
        "fillbiome",
        "forceload",
        "function",
        "gamemode",
        "gamerule",
        "give",
        "help",
        "item",
        "jfr",
        "kick",
        "kill",
        "list",
        "locate",
        "loot",
        "me",
        "msg",
        "op",
        "pardon",
        "pardon-ip",
        "particle",
        "perf",
        "place",
        "playsound",
        "publish",
        "random",
        "recipe",
        "reload",
        "return",
        "ride",
        "save-all",
        "save-off",
        "save-on",
        "say",
        "schedule",
        "scoreboard",
        "seed",
        "setblock",
        "setidletimeout",
        "setworldspawn",
        "spawnpoint",
        "spectate",
        "spreadplayers",
        "stop",
        "stopsound",
        "summon",
        "tag",
        "team",
        "teammsg",
        "teleport",
        "tell",
        "tellraw",
        "tick",
        "time",
        "tm",
        "tp",
        "transfer",
        "trigger",
        "w",
        "weather",
        "whitelist",
        "worldborder",
        "xp",
    ];

    pub fn get_commands() -> Vec<String> {
        return Self::COMMANDS.map(|s| return s.to_string()).to_vec();
    }
}

#[derive(Clone)]
pub enum MinecraftResponse {
    UnknownCommand,
    PlayerNotFound,
    ListResponse,
    BanListResponse,
    UnknownItem,
    InvalidInteger,
    NoElement,
    ExpectedInteger,
    //TO BE IMPLEMENTED
    //IntegerMin (integer less than 0)
    Default,
}

impl Response<MinecraftResponse> for MinecraftResponse {
    //Returns the most identifying part of the response. Might need to get a little more complicated with it, for example the list command identifier is very
    //short. Not sure if that's a problem.
    fn get_id_string(response: &MinecraftResponse) -> &'static str {
        return match response {
            MinecraftResponse::UnknownCommand => {
                "Unknown or incomplete command, see below for error"
            }
            MinecraftResponse::PlayerNotFound => "No player was found",
            //Handles both the list and banlist case, as their syntax is very similar
            MinecraftResponse::ListResponse => {
                r"There are (\d{1,4}) of a max of (\d{1,4}) players online"
            }
            MinecraftResponse::BanListResponse => r"There are (\d{1,4}) ban\(s\)",
            MinecraftResponse::UnknownItem => "Unknown item '",
            MinecraftResponse::InvalidInteger => "Invalid integer '",
            MinecraftResponse::NoElement => "Can't find element '",
            MinecraftResponse::ExpectedInteger => "Expected integer",
            MinecraftResponse::Default => "",
        };
    }

    //Return an iterator of all responses (besides default, which means we don't format it)
    //The repetition is a bit ugly, but it works.
    fn iterator() -> Iter<'static, MinecraftResponse> {
        [
            MinecraftResponse::UnknownCommand,
            MinecraftResponse::PlayerNotFound,
            MinecraftResponse::ListResponse,
            MinecraftResponse::BanListResponse,
            MinecraftResponse::UnknownItem,
            MinecraftResponse::InvalidInteger,
            MinecraftResponse::NoElement,
            MinecraftResponse::ExpectedInteger,
        ]
        .iter()
    }

    //Iterate over possible response values and identify if the response matches any of them
    fn from_response_str(response: &str) -> MinecraftResponse {
        for res in MinecraftResponse::iterator() {
            let id_str = MinecraftResponse::get_id_string(res);
            let regex = Regex::new(id_str).unwrap();
            if regex.is_match(response) {
                return res.clone();
            }
        }

        return MinecraftResponse::Default;
    }

    //Huge match statement which contains the formatting for all the responses we want to modify formatting for.
    fn get_output(response: &str) -> Vec<(String, ContentStyle)> {
        let res_type = Self::from_response_str(&response);
        let id_str = MinecraftResponse::get_id_string(&res_type);
        match res_type {
            MinecraftResponse::UnknownCommand => {
                let mut response_lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_at(id_str.len());
                response_lines.push((sections.0.to_string(), ContentStyle::new().red().bold()));
                response_lines.push((
                    sections.1.to_string(),
                    ContentStyle::new()
                        .red()
                        .attribute(Attribute::NoBold)
                        .attribute(Attribute::NoUnderline),
                ));

                return response_lines;
            }
            MinecraftResponse::ListResponse => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_once(":").unwrap();
                //List or banlist with player case

                lines.push((sections.0.to_string(), ContentStyle::new().bold()));

                if sections.1.trim().len() > 0 {
                    lines.push((
                        sections.1.trim().to_string(),
                        ContentStyle::new().attribute(Attribute::Reset),
                    ));
                }

                return lines;
            }
            //Nicely parsing the banlist response will probably require some complicated regex (due to the possibility of it containing IP addresses)
            MinecraftResponse::BanListResponse => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_once(":").unwrap();
                //List or banlist with player case

                lines.push((sections.0.to_string(), ContentStyle::new().bold()));

                if sections.1.trim().len() > 0 {
                    lines.push((
                        sections.1.trim().to_string(),
                        ContentStyle::new().attribute(Attribute::Reset),
                    ));
                }

                return lines;
            }
            MinecraftResponse::PlayerNotFound => {
                return vec![(response.to_string(), ContentStyle::new().red())]
            }
            MinecraftResponse::UnknownItem => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_inclusive("'");
                for (i, section) in sections.enumerate() {
                    if i == 0 {
                        lines.push((section.to_string(), ContentStyle::new().red().bold()))
                    } else if i == 1 {
                        lines[0].0.push_str(section)
                    } else {
                        lines.push((
                            section.to_string(),
                            ContentStyle::new()
                                .red()
                                .attribute(Attribute::NoBold)
                                .attribute(Attribute::NoUnderline),
                        ));
                    }
                }

                return lines;
            }
            MinecraftResponse::InvalidInteger => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                //this command is tricky to parse. There's no real clear delimiter string besides the command truncation ellipsis
                let sections = response.split_inclusive("'");
                for (i, section) in sections.enumerate() {
                    if i == 0 {
                        lines.push((section.to_string(), ContentStyle::new().red().bold()))
                    } else if i == 1 {
                        lines[0].0.push_str(section)
                    } else {
                        lines.push((
                            section.to_string(),
                            ContentStyle::new()
                                .red()
                                .attribute(Attribute::NoBold)
                                .attribute(Attribute::NoUnderline),
                        ));
                    }
                }

                return lines;
            }
            MinecraftResponse::NoElement => {
                return vec![(response.to_string(), ContentStyle::new().red())]
            }
            MinecraftResponse::ExpectedInteger => {
                let mut lines = Vec::<(String, ContentStyle)>::new();

                let sections = response.split_at(id_str.len());
                lines.push((sections.0.to_string(), ContentStyle::new().red().bold()));
                lines.push((sections.1.to_string(), ContentStyle::new().red()));

                return lines;
            }
            MinecraftResponse::Default => {
                return vec![(response.to_string(), ContentStyle::new().white())]
            }
        }
    }
}
