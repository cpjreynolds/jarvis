use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    description: String,
    cause: Option<Box<StdError>>,
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            description: String::from(desc),
            cause: None,
        }
    }

    pub fn with_cause<E>(desc: &str, cause: E) -> Error
        where E: StdError + 'static
    {
        Error {
            description: String::from(desc),
            cause: Some(box cause),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.description[..]
    }

    fn cause(&self) -> Option<&StdError> {
        match self.cause {
            Some(ref e) => { Some(&**e) },
            None => { None },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
