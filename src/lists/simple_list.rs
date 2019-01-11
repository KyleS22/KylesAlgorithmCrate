use dictionary::dict::Dict;
use custom_errors::container_full_error::ContainerFullError;
use custom_errors::container_empty_error::ContainerEmptyError;

/// A simple list with insert, search, delete functions
pub trait SimpleList<T>: Dict<T> {
    
    /// Insert x as the first element in the list
    /// Returns a result with Ok on success, ContainerFullError if the
    /// container is full
    ///
    /// # Arguments
    /// * `x` - The item to insert into the list
    fn insert_first(x: T) -> Result<(), ContainerFullError>;

    /// Return the first item of the list in a Result.
    /// Returns ContainerEmptyError if the list is empty
    fn first_item() -> Result<T, ContainerEmptyError>;

    /// Delete the first item in the list, returns a result with
    /// Ok on success, ContainerEmptyError if the list is empty
    fn delete_first() -> Result<(), ContainerEmptyError>;

    /// Insert x as the last element in the list
    /// Returns result with Ok on success, ContainerFullError if 
    /// the list is full
    fn insert_last(x: T) -> Result<(), ContainerFullError>;

    /// Return the last item in the list in a result
    /// Returns ContainerEmptyError if the list is empty
    fn last_item() -> Result<T, ContainerEmptyError>;

    /// Delete the last item in the list.  Returns a result with Ok
    /// on success, ContainerEmptyError if the list is empty
    fn delete_last() -> Result<(), ContainerEmptyError>;

    
} 