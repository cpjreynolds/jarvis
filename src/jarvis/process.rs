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

pub struct Process<A, V>
    where A: Decodable,
          V: Encodable,
{
    exec: fn(A) -> Result<V, Error>,
    argv: Option<Vec<String>>, // Optional, manually specified argv. Will use env::args otherwise.
    usage: String,
    options_first: bool,
}

impl<A, V> Process<A, V>
    where A: Decodable,
          V: Encodable,
{
    pub fn new(exec: fn(A) -> Result<V, Error>, usage: &str) -> Process<A, V> {
        Process {
            exec: exec,
            argv: None,
            usage: String::from(usage),
            options_first: false,
        }
    }

    // Sets the argv to use to construct the process' arguments.
    pub fn argv<I, S>(mut self, argv: I) -> Process<A, V>
        where I: IntoIterator<Item=S>,
              S: ToString
    {
        self.argv = Some(argv.into_iter()
                             .map(|s| s.to_string())
                             .collect());
        self
    }

    pub fn options_first(mut self, optf: bool) -> Process<A, V> {
        self.options_first = optf;
        self
    }

    pub fn execute(self) -> ProcessResult<V>
    {
        let args: A = if let Some(ref argv) = self.argv {
            decode_args(&self.usage, argv, self.options_first)
        } else {
            let ref argv: Vec<String> = env::args().collect();
            decode_args(&self.usage, argv, self.options_first)
        };
        ProcessResult::new((self.exec)(args))
    }
}

pub struct ProcessResult<V>
    where V: Encodable,
{
    result: Result<V, Error>,
}

impl<V> ProcessResult<V>
    where V: Encodable,
{
    fn new(r: Result<V, Error>) -> ProcessResult<V> {
        ProcessResult {
            result: r,
        }
    }

    pub fn handle<H>(self, h: H)
        where H: FnOnce(Result<V, Error>),
    {
        h(self.result);
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
