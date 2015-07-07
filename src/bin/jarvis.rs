extern crate rustc_serialize;
extern crate jarvis;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate env_logger;

mod bar;
mod backlight;
mod keylight;

use jarvis::util::Error;
use jarvis::process::Process;

pub const USAGE: &'static str = "
Just A Rather Very Intelligent System

Usage:
    jarvis <command> [<args>...]
    jarvis [options]

Options:
    -h, --help      Display this message
    -v, --version   Print version info and exit

Commands:
    bar             Create a new bar
    backlight       Control screen backlight
    keylight        Control keyboard backlight

See 'jarvis help <command>' for more information on a specific command.
";


#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_command: String,
    arg_args: Vec<String>,
}

fn main() {
    env_logger::init().unwrap();
    let process = Process::new(execute, USAGE).options_first(true);
    jarvis::execute(process);
}

pub fn execute(args: Args) -> Result<(), Error> {
    match &args.arg_command[..] {
        "bar" => {
            let process = Process::new(bar::execute, bar::USAGE);
            jarvis::execute(process);
        },
        "" | "help" if args.arg_args.is_empty() => {
            let args = &["jarvis", "-h"];
            let process = Process::new(execute, USAGE).argv(args);
            jarvis::execute(process);
            return Ok(())
        },
        _ => {
            println!("No such subcommand");
        },
    }
    Ok(())
}
