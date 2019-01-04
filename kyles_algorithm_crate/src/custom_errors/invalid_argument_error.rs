use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when an invalid argument is given
pub struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid argument error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for InvalidArgumentError {
    fn description(&self) -> &str {
        "invalid argument error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}