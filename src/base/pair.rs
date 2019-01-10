use std::fmt;   


#[derive(Copy, Clone, Debug)]
/// Represents a pair of values
pub struct Pair<T, U> {
    first_item: T,
    second_item: U
}

impl<T, U> Pair<T, U> {

    /// Create a new pair with the given items
    ///
    /// # Arguments
    /// * `first_item` - The first item in the pair
    /// * `second_item` - The second item in the pair
    ///
    /// # Examples
    /// ```
    /// # use kyles_algorithm_crate::base::pair::Pair;
    /// // Create a new pair with an integer and a string
    /// let pair = Pair::new(5, "Hello");
    /// ```
    pub fn new(first_item: T, second_item: U) -> Self {
        Pair{first_item, second_item}
    }

}

impl <T: fmt::Debug, U: fmt::Debug> fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.first_item, self.second_item)

    }
}

#[cfg(test)]
mod pair_tests{
    #[test]
    fn test_new(){
        use base::pair::Pair;

        let pair = Pair::new(1, "Hello");

        assert_eq!(pair.first_item, 1);
        assert_eq!(pair.second_item, "Hello");
    }

    #[test]
    fn test_to_string(){
        use base::pair::Pair;

        let pair = Pair::new(1, "Hello");
        
        assert_eq!(pair.to_string(), "(1, \"Hello\")");
    }

    #[test]
    fn test_clone(){
        use base::pair::Pair;

        let pair = Pair::new(1, "Hello");

        let clone_pair = pair.clone();

        assert_eq!(clone_pair.first_item, 1);
        assert_eq!(clone_pair.second_item, "Hello");
        
    }
}