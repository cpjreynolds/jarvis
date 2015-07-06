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
    jarvis::execute_main(execute, USAGE, true);
}

pub fn execute(args: Args) -> Result<(), Error> {
    match &args.arg_command[..] {
        "bar" => {
            jarvis::execute_main(bar::execute, bar::USAGE, false);
        },
        "" | "help" if args.arg_args.is_empty() => {
            let args = &[String::from("jarvis"), String::from("-h")];
            let retval = jarvis::call_main(execute, USAGE, args, false);
            jarvis::handle_process(retval);
            return Ok(())
        },
        _ => {
            println!("No such subcommand");
        },
    }
    Ok(())
}
