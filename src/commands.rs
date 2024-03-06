///Defines a command
pub struct Command {}

///Defines a subcommand with all of its arguments
pub struct SubCommand {
    args: Vec<Box<dyn SubCommandArg>>,
}

pub trait SubCommandArg {
    fn get_regex(&self) -> &'static str;
    fn get_pretty(&self) -> &'static str;
}
