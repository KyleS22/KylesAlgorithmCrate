use base::cursor_position::CursorPosition;

/// Interface for getting the current position of a cursor
/// and go to a specified cursor position
pub trait CursorSaving {
    
    /// Get the current position in the structure
    fn current_position(&self) -> Box<CursorPosition>;

    /// Go to the given position
    ///
    /// # Arguments
    /// * `pos` - The position to go to
    fn go_position(&mut self, pos: CursorPosition);
}
