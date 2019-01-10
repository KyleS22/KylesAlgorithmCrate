use base::membership::Membership;
use custom_errors::item_not_found_error::ItemNotFoundError;
use std::io;

/// Represents a basic dictionary
pub trait BasicDict<T>: Membership<T> {

    /// Retrieve an item from the dictionary with membership_equals
    /// Returns a result with the item, or an ItemNotFoundError
    ///
    /// # Arguments
    /// * `y` - The item to retrieve
    fn obtain(y: T) -> Result<T, ItemNotFoundError>;

    /// Insert an item into the dictionary
    /// Returns ContainerFullError if the container is full, or DuplicateItemsError if 
    /// there is a duplicate 
    ///
    /// # Arguments
    /// * `x` - The item to insert
    fn insert(x: T) -> Result<(), io::Error>;

    /// Delete the specified item
    /// Returns ItemNotFoundError if the item is not in the dict
    ///
    /// # Arguments
    /// * `x` - The item to delete
    fn delete(x: T) -> Result<(), ItemNotFoundError>;

}