#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate toml;
extern crate docopt;

pub mod util;
pub mod process;

use std::env;

use rustc_serialize::Decodable;
use docopt::Docopt;

use util::{
    Error,
};

// High level control routines.
//
// The general control structure of the executable draws heavy inspiration from the Cargo project,
// which has a great method for executing subcommands in their own environment and propagating
// errors up within an environment, to be handled gracefully without panicking.
//
// Each executable command has an `execute()` function, which serves as its main entry point. The
// execute function is passed an environment, configuration, and command line arguments. The
// function should return a `Result` 

// Wraps the function in an execution environment and calls it.
pub fn execute_main<T, V>(exec: fn(T) -> Result<V, Error>,
                          usage: &str,
                          options_first: bool)
    where T: Decodable
{
    process::<_, V>(|argv| {
        call_main(exec, usage, argv, options_first)
    });
}

// Calls the function from within the current execution environment.
pub fn call_main<T, V>(exec: fn(T) -> Result<V, Error>,
                       usage: &str,
                       argv: &[String],
                       options_first: bool) -> Result<V, Error>
    where T: Decodable
{
    let args: T = parse_argv(usage, argv, options_first);
    exec(args)
}

// Creates and wraps a function in an execution environment.
// Errors within the environment will propagate up to this level, where they are handled without
// having to panic.
fn process<F, V>(mut exec: F)
    where F: FnMut(&[String]) -> Result<V, Error>
{
    let result = (|| {
        let argv = collect_argv();
        exec(&argv)
    })();
    handle_process(result);
}

pub fn handle_process<V>(result: Result<V, Error>) {
    match result {
        Err(e) => { handle_error(e) },
        Ok(_) => {},
    }
}

fn handle_error(err: Error) {
    panic!("{:?}", err);
}

// Given a usage string and a slice of argument Strings, parse the arguments into the given type.
// The third argument determines whether "options first" behavior should be used; when options
// first behavior is enabled, all arguments after the first positional are unconditionally
// considered position arguments as well.
fn parse_argv<T>(usage: &str, argv: &[String], options_first: bool) -> T
    where T: Decodable
{
    let docopt = Docopt::new(usage).unwrap()
                                   .argv(argv.iter().map(|s| &s[..]))
                                   .help(true)
                                   .version(Some(version()))
                                   .options_first(options_first);
    // If Err is returned it's either an unanticipated error, or a version or help option.
    // In either case we should exit.
    docopt.decode().unwrap_or_else(|e| e.exit())
}

// Collects the process' arguments and returns them as a string vector.
fn collect_argv() -> Vec<String> {
    env::args().collect()
}

// Cargo defines this environment variable at compile time from the crate's manifest.
fn version() -> String {
    format!("jarvis {}", env!("CARGO_PKG_VERSION"))
}
