#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;
//extern crate toml;
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

pub fn execute<A, V>(process: Process<A, V>)
    where A: Decodable,
          V: Encodable,
{
    let result = process.execute();
    result.handle(|r| {
        match r {
            Ok(_) => {},
            Err(e) => { handle_error(e) },
        }
    });
}

fn handle_error(e: Error) -> ! {
    println!("{}", e);
    ::std::process::exit(1);
}


// Cargo defines this environment variable at compile time from the crate's manifest.
fn version() -> String {
    format!("jarvis {}", env!("CARGO_PKG_VERSION"))
}
