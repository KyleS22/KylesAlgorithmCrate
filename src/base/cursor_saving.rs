use base::cursor_position::CursorPosition;

/// Interface for getting the current position of a cursor
/// and go to a specified cursor position
pub trait CursorSaving<T> {
    
    /// Get the current position in the structure
    fn current_position(&self) -> CursorPosition<T>;

    /// Go to the given position
    ///
    /// # Arguments
    /// * `pos` - The position to go to
    fn go_position(&mut self, pos: CursorPosition<T>);
}
