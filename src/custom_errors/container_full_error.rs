use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when a container is full and elements are inserted
pub struct ContainerFullError;

impl fmt::Display for ContainerFullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "container full error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ContainerFullError {
    fn description(&self) -> &str {
        "container full error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}