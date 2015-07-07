use jarvis::util::Error;

pub const USAGE: &'static str = "
Status bar control

Usage:
    jarvis bar [options]

Options:
    -t, --test      Test command
    -h, --help      Print this message
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_test: bool,
}

pub fn execute(args: Args) -> Result<(), Error> {
    println!("{:?}", args);
    println!("Nothing to see here yet.");
    Ok(())
}
