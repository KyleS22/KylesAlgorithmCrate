use base::cursor_position::CursorPosition;
use custom_errors::no_current_item_error::NoCurrentItemError;

/// Represents a cursor that points to an item
pub trait Cursor<T>: CursorPosition + Clone {
    
    /// Returns the current item, or a NoCurrentItemError if there is no item
    fn item() -> Result<T, NoCurrentItemError>;

    /// Returns true if there is an item, false otherwise
    fn item_exists() -> bool;
}