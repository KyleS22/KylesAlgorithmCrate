
use lists::arrayed_list_iterator::ArrayedListIterator;

/// Interface to represent an arbitrary cursor.  
/// Allows user to ignore the type of the cursor.
pub enum CursorPosition<T> {
    ArrayedList(ArrayedListIterator<T>),
}


