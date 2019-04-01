use base::membership::Membership;
use base::cursor::Cursor;

/// Represents a structure that can be searched that has a cursor that 
/// can be moved around
pub trait Searchable<T>: Membership<T> + Cursor<T>{

    /// Move the current position to the first or next occurane of 
    /// `x`, if it exists.
    ///
    /// # Arguments
    /// * `x` - The item to search for
    fn search(&self, x: T);

    /// Set searches to always start from the beginning of the structure
    fn restart_searches(&mut self);

    /// Set searches to resume from the current item
    fn resume_searches(&mut self);

}
