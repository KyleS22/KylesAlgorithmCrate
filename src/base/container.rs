/// Represents a structure that contains things
pub trait Container: Clone{

    /// Returns true if the container is empty, false otherwise
    fn is_empty(&self) -> bool;

    /// Returns true if the container is full, false otherwise
    fn is_full(&self) -> bool;

    /// Remove all items from the structure
    fn clear(&mut self);
}
