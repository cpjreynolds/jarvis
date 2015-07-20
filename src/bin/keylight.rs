use jarvis::util::{
    Result,
    Error,
};

pub const USAGE: &'static str = "
Keyboard light control

Usage:
    jarvis keylight [options]

Options:
    -t, --test      Test command
    -h, --help      Print this message
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_test: bool,
}

pub fn execute(args: Args) -> Result<()> {
    println!("{:?}", args);
    println!("nothing here yet");
    Ok(())
}
