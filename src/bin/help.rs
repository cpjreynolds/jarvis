use jarvis::util::{
    Error,
};

#[derive(Debug, RustcDecodable)]
pub struct Args {
    cmd_command: String,
}

pub const USAGE: &'static str = "
Get help with a jarvis command.

Usage:
    jarvis help <command>
    jarvis help [options]

Options:
    -h, --help      Print this message
";

pub fn execute(_: Args) -> Result<(), Error> {
    Err(Error::new("Help command should not be executed"))
}
