use custom_errors::before_the_start_error::BeforeTheStartError;
use base::linear_iterator::LinearIterator;

/// An iterator that can move forward and backward
pub trait BilinearIterator: LinearIterator {
    
    /// Go to the last item of the interface
    fn go_last();

    /// Move back one item in the structure
    /// Throws a BeforeTheStartError if the cursor is before the start of the structure
    fn go_back() -> Result<(), BeforeTheStartError>;
}