extern crate rustc_serialize;
extern crate jarvis;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

mod bar;
mod backlight;
mod keylight;
mod sound;
mod help;

use jarvis::util::{
    Result,
    Error,
};
use jarvis::process::Process;

pub const USAGE: &'static str = "
Just A Rather Very Intelligent System

Usage:
    jarvis <command> [<args>...]
    jarvis [options]

Options:
    -h, --help      Display this message
    -v, --version   Print version info and exit
    --list          List commands

Commands:
    bar             Create a new bar
    backlight       Control screen backlight
    keylight        Control keyboard backlight

See 'jarvis help <command>' for more information on a specific command.
";

macro_rules! each_subcommand {
    ($mac:ident) => (
        $mac!(backlight);
        $mac!(bar);
        $mac!(sound);
        $mac!(keylight);
        $mac!(help);
    );
}


#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_command: String,
    arg_args: Vec<String>,
    flag_list: bool,
}

fn main() {
    env_logger::init().unwrap();
    let process = Process::new(execute, USAGE).options_first(true);
    jarvis::execute(process);
}

pub fn execute(args: Args) -> Result<()> {
    if args.flag_list {
        println!("Commands:");
        macro_rules! print_command {
            ($cmd:ident) => (
                println!("    {}", stringify!($cmd));
            );
        }
        each_subcommand!(print_command);
        return Ok(())
    }

    let argv = match &args.arg_command[..] {
        "" | "help" if args.arg_args.is_empty() => {
            let ref argv = ["jarvis", "-h"];
            let process = Process::new(execute, USAGE).argv(argv);
            jarvis::execute(process);
            return Ok(())
        },
        "help" if args.arg_args[0] == "-h" ||
                  args.arg_args[0] == "--help" => {
            vec!["jarvis".to_string(), "help".to_string(), "-h".to_string()]
        },
        "help" => {
            vec!["jarvis".to_string(), args.arg_args[0].clone(), "-h".to_string()]
        },
        _ => {
            env::args().collect()
        },
    };

    macro_rules! cmd {
        ($name:ident) => (
            if argv[1] == stringify!($name) {
                let process = Process::new($name::execute, $name::USAGE).argv(argv);
                jarvis::execute(process);
                return Ok(())
            }
        );
    }
    each_subcommand!(cmd);
    Err(Error::new("No such subcommand"))
}
