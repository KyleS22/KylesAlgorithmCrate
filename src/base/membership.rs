use base::container::Container;

/// A container with a membership test and equality tests
pub trait Membership<T>: Container{

    /// Returns true if the container contains the element y
    ///
    /// # Arguments
    /// * `x` - The element to check for membership
    fn has(&self, x: T) -> bool;

    /// Determines if two elements of type T are equal
    ///
    /// # Arguments
    /// * `x` - The first element to compare
    /// * `y` - The second element to compare
    fn membership_equals(&self, x: T, y: T) -> bool;
}
