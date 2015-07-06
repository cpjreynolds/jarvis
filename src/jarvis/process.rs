use std::env;

use docopt::Docopt;
use rustc_serialize::{
    Decodable,
    Encodable,
};

use super::version;
use util::{
    Error,
};

pub struct Process<F, A, V>
    where F: Fn(A) -> Result<V, Error>,
          A: Decodable + Clone,
          V: Encodable
{
    exec: F,
    args: Option<A>, // exec function specific argument structure.
    argv: Option<Vec<String>>, // Optional, manually specified argv.
    usage: String,
    options_first: bool,
}

impl<F, A, V> Process<F, A, V>
    where F: Fn(A) -> Result<V, Error>,
          A: Decodable + Clone,
          V: Encodable
{
    pub fn new(exec: F, usage: &str) -> Process<F, A, V> {
        Process {
            exec: exec,
            args: None,
            argv: None,
            usage: String::from(usage),
            options_first: false,
        }
    }

    // Sets the argv to use to construct the process' arguments.
    pub fn argv<I, S>(&mut self, argv: Option<I>) -> &mut Process<F, A, V>
        where I: IntoIterator<Item=S>,
              S: ToString
    {
        self.args = None; // Invalidate any cached Args structure.
        self.argv = if let Some(argv) = argv {
            Some(argv.into_iter()
                     .map(|s| s.to_string())
                     .collect())
        } else {
            None
        };
        self
    }

    pub fn options_first(&mut self, optf: bool) -> &mut Process<F, A, V> {
        self.options_first = optf;
        self
    }

    pub fn execute(&mut self) -> Result<V, Error> {
        // If there is a cached Args structure, use it instead of remaking one.
        let args = if let Some(ref args) = self.args {
            args.clone()
        } else {
            match self.argv {
                Some(ref argv) => {
                    self.args = Some(decode_args(&self.usage, argv, self.options_first));
                    self.args.clone().unwrap()
                },
                None => {
                    let argv: Vec<String> = env::args().collect();
                    self.args = Some(decode_args(&self.usage, &argv, self.options_first));
                    self.args.clone().unwrap()
                },
            }
        };
        (self.exec)(args)
    }
}

fn decode_args<A>(usage: &str, argv: &[String], options_first: bool) -> A
    where A: Decodable
{
    let docopt = Docopt::new(usage).unwrap()
                                   .argv(argv.iter().map(|s| &s[..]))
                                   .help(true)
                                   .version(Some(version()))
                                   .options_first(options_first);
    docopt.decode().unwrap_or_else(|e| e.exit())
}
