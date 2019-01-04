use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when there is no current item at the cursor
pub struct NoCurrentItemError;

impl fmt::Display for NoCurrentItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no current item error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for NoCurrentItemError {
    fn description(&self) -> &str {
        "no current item error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}