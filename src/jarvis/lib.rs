#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate toml;
extern crate docopt;

pub mod process;
pub mod util;

use rustc_serialize::{
    Decodable,
    Encodable,
};

use process::{
    Process,
};

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

pub fn execute<F, A, V>(process: Process<F, A, V>)
    where F: Fn(A) -> Result<V, Error>,
          A: Decodable,
          V: Encodable,
{
    let r = process.execute();
    r.handle();
}

// Cargo defines this environment variable at compile time from the crate's manifest.
fn version() -> String {
    format!("jarvis {}", env!("CARGO_PKG_VERSION"))
}
