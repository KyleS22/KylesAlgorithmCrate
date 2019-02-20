use base::cursor_position::CursorPosition;
use custom_errors::after_the_end_error::AfterTheEndError;
use custom_errors::container_empty_error::ContainerEmptyError;

/// An iterator for linearly moving through a collection of items.
/// Uses a cursor to keep track of the current position, and has functions
/// to move around in the sequence
pub trait LinearIterator: CursorPosition {
    
    /// Is the current position before the start of the structure?
    fn before(&self) -> bool;

    /// Is the current position after the end of the structure?
    fn after(&self) -> bool;

    /// Advance one item in the data structure.
    /// Returns an AfterTheEndError if the cursor goes after the end of the structure
    fn go_forth(&mut self) -> Result<(), AfterTheEndError>;
    
    /// Sets the cursor to the first element in the structure
    /// Returns a ContainerEmptyError if the structure is empty
    fn go_first(&mut self) -> Result<(), ContainerEmptyError>;

    /// Move the cursor to the position before the first element of the structure
    fn go_before(&mut self);

    /// Move the cursor to the position after the last element of the structure
    fn go_after(&mut self);
}
