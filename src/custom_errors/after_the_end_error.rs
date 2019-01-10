use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when a cursor is after the end of the structure
pub struct AfterTheEndError;

impl fmt::Display for AfterTheEndError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "after the end error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for AfterTheEndError {
    fn description(&self) -> &str {
        "after the end error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}