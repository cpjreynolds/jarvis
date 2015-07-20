use std::error::Error as StdError;
use std::fmt;
use std::result;
use std::io;
use std::num;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    desc: String,
    detail: Vec<String>,
    cause: Option<Box<StdError>>,
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            desc: String::from(desc),
            detail: Vec::new(),
            cause: None,
        }
    }

    pub fn with_cause<E>(desc: &str, cause: E) -> Error
        where E: StdError + 'static
    {
        let mut err = Error::new(desc);
        err.add_detail(cause.description());
        err.cause = Some(box cause);
        err
    }

    pub fn add_detail(&mut self, detail: &str) {
        self.detail.push(String::from(detail));
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&StdError> {
        match self.cause {
            Some(ref e) => { Some(&**e) },
            None => { None },
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::with_cause("internal I/O error", err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::with_cause("integer parse error", err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}
