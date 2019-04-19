use custom_errors::no_current_item_error::NoCurrentItemError;
use base::container::Container;
use base::cursor::Cursor;
use std::fmt::Error;

/// A container that keeps track of the current item as inserts are made
/// Only the current item can be deleted
pub trait Dispenser<T>: Container + Cursor<T> {

    /// Insert an item into the structure
    /// Returns a DuplicateItemError or ContainerFullError
    ///
    /// # Arguments
    /// * `x` - The item to insert into the structure
    //fn insert(&mut self, x: T) -> Result<(), Error>;

    /// Delete the current item from the structure
    /// Returns a NoCurrentItemError if there is no current item
    fn delete_item(&mut self) -> Result<(), NoCurrentItemError>;
}
