use base::linear_iterator::LinearIterator;
use base::cursor_position::CursorPosition;

use custom_errors::container_empty_error::ContainerEmptyError;
use custom_errors::after_the_end_error::AfterTheEndError;
use custom_errors::no_current_item_error::NoCurrentItemError;
use custom_errors::invalid_argument_error::InvalidArgumentError;

use std::error::Error;

//const AFTER_POS: i32 = -1;
//const BEFORE_POS: i32 = -2;

trait ConstPositions{
    const AFTER_POS: i32 = -1;
    const BEFORE_POS: i32 = -2;
}

pub struct ArrayedListIterator<T> {
    list_elements: Vec<T>,
    head: i32,
    tail: i32,
    capacity: u32,
    num_el: u32,
    position: i32,
}

impl<T> ConstPositions for ArrayedListIterator<T>{

}

impl<T> ArrayedListIterator<T> 
    where T: Copy,
{
    //const AFTER_POS: i32 = -1;
    //const BEFORE_POS: i32 = -2;
    
    
	/// Create a new Arrayed List iterator
	///
	/// # Arguments
	/// * `elements` - The elements in the arrayed list to include in the iterator
	/// * `head` - The head of the list 
	/// * `tail` - The tail of the list
	/// * `num_el` - The number of elements in the list
	/// 
	/// # Example
	/// ```
	///  # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    ///  // Create a new ArrayedListIterator with the elements 1, 2, 3
    ///  // The head of the list is the first element, the tail is the last, and there are
    ///  // currently three elements in the list 
    ///  let iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    ///  
	/// ```
    pub fn new(elements: Vec<T>, head: i32, tail: i32, num_el: u32) -> Self {
        let len = elements.iter().count(); 
        ArrayedListIterator {list_elements: elements, head:head, tail:tail, capacity: len as u32, num_el: num_el, position: 0}
    }

    
	/// Set the current position of the iterator
	///
	/// # Arguments
	/// * `pos` - The index of the position to set the iterator to
	/// 
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// 
    /// // The current position of the iterator is head
    /// let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// 
    /// // Set the position to the second element, being 2 in this case
    /// iter.set_position(1);
    ///
	/// ```
    pub fn set_position(&mut self, pos: i32) -> Result<(), Box<Error>>{
        if self.num_el == 0 {
            return Err(Box::new(AfterTheEndError));
        } else if pos >= self.capacity as i32{
            return Err(Box::new(InvalidArgumentError));
        } else {
            self.position = pos;
        }
        
        Ok(())
    }
    
    
	/// Return the current item in the iterator
	///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// let iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    ///
    /// // Gets the current item in the list, 1 in this case
    /// let item = iter.item();
	/// ```
    pub fn item(&self) -> Result<T, NoCurrentItemError> {
        if self.item_exists() && self.num_el > 0 {
            return Ok(self.list_elements[self.position as usize]);
        }else {
            Err(NoCurrentItemError)
        }
    }
   
    
	/// Check if there is a current item
	/// 
	/// 
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// let iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// if iter.item_exists(){
    ///     println!("There is an item here!");
    /// }
	/// ```
    pub fn item_exists(&self) -> bool {
        
        if self.position != ArrayedListIterator::<T>::AFTER_POS && self.position != ArrayedListIterator::<T>::BEFORE_POS {
            return true;
        }else{
            return false
        }
    }


}

impl<T> LinearIterator for ArrayedListIterator<T> {
    
    
	/// See if the cursor is before the beginning of the list
	///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// 
    /// // This will be false
    /// if iter.before(){
    ///     println!("We are before the list");
    /// } 
    ///
	/// ```
    fn before(&self) -> bool {
        if self.position == ArrayedListIterator::<T>::BEFORE_POS{
            return true;
        }else {
            return false;
        }
    }
    
	/// See if the cursor is after the end of the list
	///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// 
    /// // This will be false
    /// if iter.after(){
    ///     println!("We are after the list");
    /// } 
    ///
	/// ```
    fn after(&self) -> bool {
        if self.position == ArrayedListIterator::<T>::AFTER_POS{
            return true;
        }else{
            return false;
        }
    }
    
     
	/// Move the current position forward one
	/// 
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// iter.go_forth(); // The current item will now be 2
    /// let item = iter.item();
	/// ```
    fn go_forth(&mut self) -> Result<(), AfterTheEndError> {
        
        if self.position == ArrayedListIterator::<T>::BEFORE_POS{
            self.position = self.head;
        }else if self.position == self.tail{
            self.position = ArrayedListIterator::<T>::AFTER_POS;
        }else if self.position == ArrayedListIterator::<T>::AFTER_POS{
            return Err(AfterTheEndError);
        }else {
            self.position = self.position + 1;
        }
        
        Ok(())
    }

    
	/// Move the cursor to the first item in the list
    ///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// iter.go_forth();  // Current item is 2
    /// iter.go_first();  // Current item is back to 1
	/// ```
    fn go_first(&mut self) -> Result<(), ContainerEmptyError> {
        if self.num_el <= 0 {
            return Err(ContainerEmptyError);
        }else{
            self.position = self.head;
        }
        
        Ok(())
    }

	/// Move the cursor to the before position
    ///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    /// 
    /// let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// iter.go_forth();  // Current item is 2
    /// iter.go_before(); // There is no current item, we are before the list
	/// ```
    fn go_before(&mut self){
        self.position = ArrayedListIterator::<T>::BEFORE_POS;
    }

	/// Move the cursor to the after position
    ///
	/// # Example
	/// ```
	/// # use kyles_algorithm_crate::lists::arrayed_list_iterator::ArrayedListIterator;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    /// iter.go_forth();  // Current item is 2
    /// iter.go_after();  // There is no current item, we are after the list
	/// ```
    fn go_after(&mut self) {
        self.position = ArrayedListIterator::<T>::AFTER_POS;
    }

}

impl<T> CursorPosition for ArrayedListIterator<T> {

}

#[cfg(test)]
mod arrayed_list_iterator_tests {
    
    #[test]
    fn test_set_position(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use lists::arrayed_list_iterator::ConstPositions;

        // Test empty case
        let mut empty_iter: ArrayedListIterator<i32> = ArrayedListIterator::new(vec![], 0, 0, 0);

        match empty_iter.set_position(1) {
            Err(_e) => assert!(true),
            _ => assert!(false)
        }

        // Test cases with items
        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);

        // Test position given > capacity 
        match iter.set_position(5) {
            Err(_e) => assert!(true),
            _ => assert!(false)
        }

        // Test go to before pos
        match iter.set_position(-2) {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, ArrayedListIterator::<i32>::BEFORE_POS);
        
        // Test go to after pos
        match iter.set_position(-1) {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, ArrayedListIterator::<i32>::AFTER_POS);

        // Test other positions, 0, 1, 2
        match iter.set_position(0) {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, 0);
   
         match iter.set_position(1) {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, 1);

        match iter.set_position(2) {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, 2);
    
    }

    #[test]
    fn test_item(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;

        let mut iter = ArrayedListIterator::new(vec![1,2,3], 0, 2, 3);
        
        match iter.item() {
            Ok(1) => assert!(true),
            Err(_e) => assert!(false),
            _ => assert!(false)
        }

        iter.go_before();
        match iter.item() {
            Err(_e) => assert!(true),
            _ => assert!(false)
        }

        iter.set_position(1);

        match iter.item() {
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_item_exists(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);

        assert_eq!(iter.item_exists(), true);

        iter.go_before();
        
        assert_eq!(iter.item_exists(), false);

        iter.go_after();

        assert_eq!(iter.item_exists(), false);
    }

    #[test]
    fn test_before(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
   
        assert_eq!(iter.before(), false);

        iter.go_before();

        assert_eq!(iter.before(), true);
    
    }

    #[test]
    fn test_after(){
        
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    
        assert_eq!(iter.after(), false);

        iter.go_after();

        assert_eq!(iter.after(), true);
    }

    #[test]
    fn test_go_forth(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;
        use lists::arrayed_list_iterator::ConstPositions;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
    
        match iter.go_forth(){
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, 1);

        match iter.go_forth() {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, 2);
        
        match iter.go_forth() {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, ArrayedListIterator::<i32>::AFTER_POS);
        
        match iter.go_forth() {
            Err(_e) => assert!(true),
            _ => assert!(false)
        }

    }

    fn test_go_first(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
        
        iter.go_forth();

        match iter.go_first() {
            Err(_e) => assert!(false),
            _ => assert!(true)
        }

        assert_eq!(iter.position, iter.head);
        
    }

    fn test_go_before(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;
        use lists::arrayed_list_iterator::ConstPositions;

        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);

        iter.go_before(); 

        assert_eq!(iter.position, ArrayedListIterator::<i32>::BEFORE_POS);
    }

    fn test_go_after(){
        use lists::arrayed_list_iterator::ArrayedListIterator;
        use base::linear_iterator::LinearIterator;
        use lists::arrayed_list_iterator::ConstPositions;
        
        let mut iter = ArrayedListIterator::new(vec![1, 2, 3], 0, 2, 3);
        
        iter.go_after();
        assert_eq!(iter.position, ArrayedListIterator::<i32>::AFTER_POS);

    }
}

