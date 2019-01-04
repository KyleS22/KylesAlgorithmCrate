use base::cursor_position::CursorPosition;
use custom_errors::after_the_end_error::AfterTheEndError;

/// An iterator for linearly moving through a collection of items.
/// Uses a cursor to keep track of the current position, and has functions
/// to move around in the sequence
pub trait LinearIterator: CursorPosition {
    
    /// Is the current position before the start of the structure?
    fn before() -> bool;

    /// Is the current position after the end of the structure?
    fn after() -> bool;

    fn go_forth() -> Result<(), AfterTheEndError>;
    // TODO THERE IS MORE

}