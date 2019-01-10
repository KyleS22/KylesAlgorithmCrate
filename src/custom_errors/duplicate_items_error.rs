use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when there are duplicate items
pub struct DuplicateItemsError;

impl fmt::Display for DuplicateItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "duplicate items error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for DuplicateItemsError {
    fn description(&self) -> &str {
        "duplicate items error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}