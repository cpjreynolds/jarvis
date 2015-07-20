use rustc_serialize::{
    Decodable,
};

use jarvis::ops::sound;
use jarvis::util::{
    Result,
    Error,
};

pub const USAGE: &'static str = "
Sound control

Usage:
    jarvis sound [options] <command> [<value>]
    jarvis sound -h | --help

Commands:
    get         Get sound level
    set         Set sound level (default: 50)
    inc         Increase sound level (default: 10)
    dec         Decrease sound level (default: 10)

Options:
    -h, --help  Print this message
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_command: Command,
    arg_value: Option<u8>,
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Get,
    Set,
    Inc,
    Dec,
}

pub fn execute(args: Args) -> Result<()> {
    match args.arg_command {
        Command::Get => {
            let val = try!(sound::get());
            println!("sound level: {}", val);
            Ok(())
        },
        Command::Set => {
            let val = args.arg_value.unwrap_or(50);
            let oldval = try!(sound::get());
            let newval = try!(sound::set(val));
            println!("sound level: {} => {}", oldval, newval);
            Ok(())
        },
        _ => {
            unimplemented!()
        },
    }
}
