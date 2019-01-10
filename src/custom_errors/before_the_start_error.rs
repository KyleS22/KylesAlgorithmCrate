use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when a cursor is before the start of the structure
pub struct BeforeTheStartError;

impl fmt::Display for BeforeTheStartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "before the start error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for BeforeTheStartError {
    fn description(&self) -> &str {
        "before the start error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}