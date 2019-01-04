use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when an item is not found in the structure
pub struct ItemNotFoundError;

impl fmt::Display for ItemNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item not found error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ItemNotFoundError {
    fn description(&self) -> &str {
        "item not found error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}