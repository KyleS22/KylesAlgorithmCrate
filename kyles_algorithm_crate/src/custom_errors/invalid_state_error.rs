use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when a structure is in an invalid state
pub struct InvalidStateError;

impl fmt::Display for InvalidStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid state error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for InvalidStateError {
    fn description(&self) -> &str {
        "invalid state error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}