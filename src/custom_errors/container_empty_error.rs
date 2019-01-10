use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// An error that occurs when a container is empty and an operation is attempted
/// on its elements
pub struct ContainerEmptyError;

impl fmt::Display for ContainerEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "container empty error!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ContainerEmptyError {
    fn description(&self) -> &str {
        "container empty error!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}