// File Name: arrayed_list.rs
//
// Authors: Kyle Seidenthal 
//
// Date: 01-03-2019
//
// Description: Implementation of an Arrayed List data structure 

use base::cursor_position::CursorPosition;
use base::basic_dict::BasicDict;
use base::searchable::Searchable;
use base::membership::Membership;
use base::dispenser::Dispenser;
use base::container::Container;
use base::cursor::Cursor;
use base::cursor_saving::CursorSaving;
use base::linear_iterator::LinearIterator;

use custom_errors::after_the_end_error::AfterTheEndError;
use custom_errors::container_empty_error::ContainerEmptyError;
use custom_errors::container_full_error::ContainerFullError;
use custom_errors::invalid_argument_error::InvalidArgumentError;
use custom_errors::item_not_found_error::ItemNotFoundError;
use custom_errors::no_current_item_error::NoCurrentItemError;

use lists::arrayed_list_iterator::ArrayedListIterator;
use lists::simple_list::SimpleList;

use dictionary::dict::Dict;

use std::fmt;
use std::error::Error;
 
trait ConstPositions{
    const AFTER_POS: i32 = -1;
    const BEFORE_POS: i32 = -2;
}

/// A struct to represent an arrayed list structure
#[derive(Clone, Debug)]
pub struct ArrayedList<T: 'static>{
    list_elements: Vec<T>,
    head: i32,
    tail: i32,
    capacity: usize,
    num_el: u32,
    position: i32,
    continue_search: bool,
}

impl <T> ConstPositions for ArrayedList<T>{

}

impl <T> ArrayedList<T>
    where T: Copy + Clone, {
    
         
	/// Create an empty list
	///
	/// # Arguments
	/// * `capacity` - The maximum number of elements that can go into the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// 
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
	/// ```
    pub fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);

        ArrayedList {list_elements: vec, head: 0, tail: 0, capacity: capacity, position: ArrayedList::<T>::BEFORE_POS, num_el: 0,  continue_search: false}
    }
   
     
	/// Return the capacity of the list
	///
	/// # Arguments
	/// * `&self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// // Create a list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // The capacity is 3
    /// if list.capacity() == 3{
    ///     println!("The capacity is 3");
    /// }
	/// ```
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    
	/// Return an iterator for the list
	///
	/// # Arguments
	/// * `&self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Get the iterator for this list
    /// let iter = list.iterator();
	/// ```
    pub fn iterator(&self) -> ArrayedListIterator<T>{
        return ArrayedListIterator::new(self.list_elements.clone(), self.head, self.tail, self.num_el);
    }

    
	/// Get an item from the list at a specific index
	///
	/// # Arguments
	/// * `&self` - A mutable reference to the list
	/// * `index` - The index to get an item from
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    ///
    /// list.insert_first(2);
    ///
    /// list.insert_first(1);
    ///
    /// // This will return the first item, 1
    /// list.get_item_at_index(0);
    ///
    /// // This will return the second item, 2
    /// list.get_item_at_index(1);
	/// ```
    pub fn get_item_at_index(&self, index: u32) -> Result<T, InvalidArgumentError>{
        
        // Make sure the index is valid
        if index < 0 || index > (self.tail - 1) as u32 {
            return Err(InvalidArgumentError);
        }
        
        return Ok(self.list_elements[index as usize]);
    
    }

}

impl<T> SimpleList<T> for ArrayedList<T>
    where T: Clone + Copy
{
    
	/// Insert the given item into the list at the first position
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// * `x` - The item to insert into the first position of the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    /// 
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// // Insert the item 1 into the list at the beginning
    /// list.insert_first(1).expect("The list is full");
    ///
    /// // Insert the item 2 into the list before the item 1
    /// list.insert_first(2).expect("The list is full");
    ///
    /// // Insert a third element
    /// list.insert_first(3).expect("The list is full");
    ///
    /// // Inserting another item will cause an error, because the list is full
    /// list.insert_first(4).expect("The list is full");
	/// ```
    fn insert_first(&mut self, x: T) -> Result<(), ContainerFullError>{
        
        // Make sure the list is not full
        if self.is_full(){
            return Err(ContainerFullError);
        }
        
        
        self.list_elements.insert(0, x);//[self.head as usize] = x;
        
        self.num_el += 1;
        self.head = (self.head - 1) % (self.capacity as i32);

        Ok(())
    }

    
	/// Return the first item in the list
	///
	/// # Arguments
	/// * `&self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    /// 
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// // Insert some items
    /// list.insert_last(1).expect("Error inserting item");
    /// list.insert_last(2).expect("Error inserting item");
    /// list.insert_last(3).expect("Error inserting item");
    /// 
    /// if list.first_item() == 1{
    ///     println!("The first item is 1");
    /// }else{
    ///     println!("This should not happen");
    /// }
    ///
	/// ```
    fn first_item(&self) -> Result<T, ContainerEmptyError>{
        
        // Check to see if the list is empty
        if self.is_empty(){
            return Err(ContainerEmptyError)
        }
        
        return Ok(self.list_elements[self.head as usize]);
    }

    
	/// Delete the first item in the list
	///
	/// # Arguments
	/// * `&mut self` - Reference to self
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    ///
    /// // Create a new list
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Insert some items
    /// list.insert_last(1);
    /// list.insert_last(2);
    /// list.insert_last(3);
    ///
    /// // Remove 1 from the list
    /// list.delete_first();
	/// ```
    fn delete_first(&mut self) -> Result<(), ContainerEmptyError>{
       
        // Check for empty list
        if self.is_empty(){
            return Err(ContainerEmptyError)
        }
       
        // If we are deleting the current position, move the position up
        if self.position == self.head{
            self.position = (self.head + 1) % (self.capacity as i32);
        }
        
        // Move the head up
        self.head = (self.head + 1) % (self.capacity as i32);
        self.num_el -= 1;
            
        // Check to see if we should be in the before position
        if self.is_empty(){
            self.position = ArrayedList::<T>::BEFORE_POS;
        }

        return Ok(())
    }
    
    
	/// Insert the given item at the end of the list
	///
	/// # Arguments
	/// * `&mut self` - A reference to self
	/// * `x` - The item to insert at the end of the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    ///
    /// // Create a new list
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Insert some items at the end of the list
    /// list.insert_last(1);
    /// list.insert_last(2);
    /// list.insert_last(3);
    ///
    /// // The list now contains [1, 2, 3]
	/// ```
    fn insert_last(&mut self, x: T) -> Result<(), ContainerFullError>{
        
        // Make sure the list is not full
        if self.is_full(){
            return Err(ContainerFullError)
        }

        self.tail = (self.tail + 1) % (self.capacity as i32);

        self.list_elements[self.tail as usize] = x;
        self.num_el += 1;

        return Ok(())
    }
    
    
	/// Return the last item in the list
	///
	/// # Arguments
	/// * `&self` - Reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    ///
    /// // Create a new list
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Insert some items
    /// list.insert_first(1);
    /// list.insert_first(2);
    ///
    /// // Get the last item, which is 2
    /// list.last_item();
    ///
	/// ```
    fn last_item(&self) -> Result<T, ContainerEmptyError>{
        
        // Make sure the list is not empty
        if self.is_empty(){
            return Err(ContainerEmptyError)
        }
        
        return Ok(self.list_elements[self.tail as usize])

    }
    
    

    
	/// Delete the last item in the list
	///
	/// # Arguments
	/// * `&mut self` - A reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    ///
    /// // Create a new list
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // insert some items
    /// list.insert_first(1);
    /// list.insert_first(2);
    /// list.insert_first(3);
    ///
    /// // Delete the last item, which is 3 in this case
    /// list.delete_last();
    ///
	/// ```
    fn delete_last(&mut self) -> Result<(), ContainerEmptyError>{
        if self.is_empty() {
            return Err(ContainerEmptyError)
        }

        // If the cursor is on this item, we need to move it
        if self.position == self.tail {
            self.position = (self.tail - 1) % (self.capacity as i32);
        }

        // Update the tail
        self.tail = (self.tail - 1) % (self.capacity as i32);

        self.num_el -= 1;

        if self.is_empty(){
            self.position = ArrayedList::<T>::BEFORE_POS;
        }

        return Ok(())

    }

}



    
impl<T> BasicDict<T> for ArrayedList<T>
    where T: Clone + Copy
{
    
    
	/// Get the given item from the list
	///
	/// # Arguments
	/// * `&self` - A reference to the list
	/// * `y` - The item to obtain
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::simple_list::SimpleList;
    ///
    /// // Create a new list
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Add some items
    /// list.insert_first(1);
    /// list.insert_first(2);
    /// list.insert_first(3);
    ///
    /// // Obtain 2
    /// let two = list.obtain(2);
	/// ```
    fn obtain(&mut self, y: T) -> Result<T, ItemNotFoundError>{
        
        // Save the cursor state
        let save_pos;
        match self.current_position(){
            CursorPosition::ArrayedList(pos) => save_pos = pos,
            _ => return Err(ItemNotFoundError)
        }
        


        // Search for y
        self.search(y);

        // See if there is an item at the cursor
        if !self.item_exists() {
            return Err(ItemNotFoundError);
        }

        let result = self.item().unwrap();

        // Restore cursor
        self.go_position(CursorPosition::ArrayedList(save_pos));

        return Ok(result)

    }
    
    
	/// Insert the given item at the current position in the list
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// * `x` - The item to insert
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::basic_dict::BasicDict;
    ///
    /// // Create an empty list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    ///
    /// // Insert 1 at the current position in the list
    /// list.insert(1);
    ///
    /// // 2 will be inserted after 1
    /// list.insert(2);
	/// ```
    fn insert(&mut self, x: T) -> Result<(), Box<Error>>{
        self.insert_item(x)
    }

    
	/// Delete the given item
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// * `x` - The item to delete
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// // Create a new list with capacity 3
    /// let &mut list = ArrayedList::new(3);
    ///
    /// list.insert(1);
    ///
    /// // Delete 1 from the list
    /// list.delete(1);
	/// ```
    fn delete(&mut self, x: T) -> Result<(), Box<Error>>{
        
        // Make sure the list is not empty
        if self.is_empty() {
            return Err(Box::new(ContainerEmptyError));
        }
        
        // Save the cursor
        let save_pos;
        
        match self.current_position(){
            CursorPosition::ArrayedList(pos) => save_pos = pos,
            _ => return Err(Box::new(InvalidArgumentError)),
        }
        


        let save_item;

        // Save the item at the cursor if it exists
        if save_pos.item_exists() {
            save_item = Some(save_pos.item());   
        } else {
            save_item = None;
        }

        // Find the item to delete
        self.search(x);
        
        // If there is no item, then it does not exist
        if !self.item_exists() {
            return Err(Box::new(ItemNotFoundError));
        }
        

        // If we are deleting the item at the cursor, just remove it
        if self.position == save_pos.position {
            self.delete_item();
        
        // Otherwise, we need to restore the cursor
        } else {

            match save_item {
                None => self.position = save_pos.position,
                Some(item) => {
                    let save_continue_setting = self.continue_search;
                    self.restart_searches();

                
                    self.search(item.unwrap());

                    self.continue_search = save_continue_setting;
   
                }
            }
        }

        return Ok(())

    }
 
}

impl<T> Searchable<T> for ArrayedList<T>
    where T: Clone + Copy
{   
    
	/// Search for the given item
	///
	/// # Arguments
	/// * `&self` - Reference to self
	/// * `x` - The item to search for in the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// let &mut list = ArrayedList::new(3);
    ///
    /// list.insert(1);
    /// list.insert(2);
    /// list.insert(3);
    /// 
    /// // The cursor of the list will now be on 1
    /// list.search(1);
	/// ```
    fn search(&mut self, x: T){
        
        if !self.continue_search {
            self.go_first();
        } else if !self.after() {
            self.go_forth();
        }

        while !self.after() && !self.membership_equals(x, self.item().unwrap()){
            self.go_forth();
        }
    }
    
    
	/// Set the search mechanism for the list to restart from the beginning of the list each time
	///
	/// # Arguments
	/// * `&mut self` - Reference to self
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// // Now the list will start all searches from the beginning of the list
    /// list.restart_searches();
	/// ```
    fn restart_searches(&mut self){
        self.continue_search = false;
    }

	/// Set the search mechanism for the list to start from the current cursor position each time
	///
	/// # Arguments
	/// * `&mut self` - Reference to self
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// // Now the list will start all searches from the current position
    /// list.resume_searches();
	/// ``` 
    fn resume_searches(&mut self){
        self.continue_search = true;
    }

}

impl<T> Membership<T> for ArrayedList<T>
    where T: Clone + Copy
{
    
	/// Return true if the item 'x' exists in the list, false otherwise.
	///
	/// # Arguments
	/// * `&self` - Reference to self
	/// * `x` - The item to check for
	/// 
	/// # Example
	/// ```
	/// ise kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    ///
    /// let & mut list = ArrayedList::new(3);
    ///
    /// list.insert_first(4);
    /// list.insert_first(5);
    /// 
    /// // This will print 'True!'
    /// if (list.has(5)){
    ///     println!("True!");
    /// }
    /// else{
    ///     println!("False");
    /// }
    /// 
    /// // This will print 'False'
    /// if (list.has(10)){
    ///     println!("True");
    /// }else{
    ///     println!("False");
    /// }
	/// ```
    fn has(&mut self, x: T) -> bool{
        
        // Place to save the cursor
        let save_pos;

        match self.current_position(){
            CursorPosition::ArrayedList(pos) => save_pos = CursorPosition::ArrayedList(pos),
            _ => return false,
        }

        self.search(x);

        if self.item_exists(){
            return true;
        }else{
            return false;
        }

        self.go_position(save_pos);
    }

    fn membership_equals(&self, x: T, y: T) -> bool{
        false
    }

}

impl<T> Dispenser<T> for ArrayedList<T>
    where T: Clone + Copy
{
    fn insert_item(&mut self, x: T) -> Result<(), Box<Error>>{
        Err(Box::new(NoCurrentItemError))
    }
    
    // delets at the current position    
    fn delete_item(&mut self) -> Result<(), NoCurrentItemError>{
        Err(NoCurrentItemError)
    }

}

impl<T> Container for ArrayedList<T>
    where T: Clone
{
    fn is_empty(&self) -> bool{
        return self.head == self.tail && self.num_el == 0;
    }
    
    fn is_full(&self) -> bool{
        return self.head == self.tail && self.num_el == self.capacity as u32;
    }
    
    fn clear(&mut self){
        self.list_elements.clear();
        self.num_el = 0;
        self.head = 0;
        self.tail = 0;
        self.position = 0;
    }
}

impl<T> Cursor<T> for ArrayedList<T>
    where T: Clone + Copy
{
    
    fn item(&self) -> Result<T, NoCurrentItemError>{
        
        if !self.item_exists(){
            return Err(NoCurrentItemError);
        }

        return Ok(self.list_elements[self.position as usize]);
    }
    
    fn item_exists(&self) -> bool{
        if self.position != ArrayedList::<T>::BEFORE_POS && self.position != ArrayedList::<T>::AFTER_POS {
            return true;
        } else {
            return false;
        }
    }
}


impl<T> CursorSaving<T> for ArrayedList<T>
    where T: Clone + Copy
{
  
    fn current_position(&self) -> CursorPosition<T> {
        let cursor_pos = ArrayedListIterator::new(self.list_elements.clone(), self.head, self.tail, self.num_el);
        CursorPosition::ArrayedList(cursor_pos)
    }
    
  
    fn go_position(&mut self, pos: CursorPosition<T>){

    }

}


impl<T> LinearIterator for ArrayedList<T>
    where T: Clone
{
    
	/// Check to see if the current position in the structure is the before position
	///
	/// # Arguments
	/// * `&self` - Reference to self
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrated_list::ArrayedList;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// if !list.before() {
    ///     println!("We are not before the list!");
    /// }
    /// 
	/// ```
    fn before(&self) -> bool{
        return self.position == ArrayedList::<T>::BEFORE_POS;
    }
    
    
	/// Check to see if the current position in the structure is the after position
	///
	/// # Arguments
	/// * `&self` - Reference to self
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrated_list::ArrayedList;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// if !list.after() {
    ///     println!("We are not after the list!");
    /// }
    /// 
	/// ```
    fn after(&self) -> bool{
        return self.position == ArrayedList::<T>::AFTER_POS;
    }


    
	/// Move the cursor forward one position
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let &mut list = ArrayedList::new(3);
    ///
    /// list.insert_first(1).expect("Error inserting into list");
    /// list.insert_first(2).expect("Error inserting into list");
    ///
    /// // The cursor is now on element 2, which is first in the list
    ///
    /// match list.go_forth(){
    ///     Ok(()) => println!("WE ARE NOW ON ELEMENT 1, the second item in the list")
    /// }
    ///
	/// ```
    fn go_forth(&mut self) -> Result<(), AfterTheEndError>{
        
        // If we are in the before position, go to first position
        if self.position == ArrayedList::<T>::BEFORE_POS {
            self.position = self.head;    

        // We are after the end of the list, so error
        } else if self.position == ArrayedList::<T>::AFTER_POS{
           return Err(AfterTheEndError)

        // otherwise, if we are going to be after the end when we go forth
        } else if self.position == self.tail {
            self.position = ArrayedList::<T>::AFTER_POS;

        // Otherwise, increment the position
        } else{
            self.position = (self.position + 1) % (self.capacity as i32);
        }

        Ok(())
    }
   
   
     
	/// Move the cursor the the first position
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let &mut list = ArrayedList::new(3);
    ///
    /// list.insert_first(1).expect("Error inserting into list");
    /// list.insert_first(2).expect("Error inserting into list");
    ///
    /// list.go_after();
    ///
    /// match list.go_first(){
    ///     Ok(()) => println!("WE ARE NOW ON ELEMENT 2, the first item in the list")
    /// }
    ///
	/// ```
    fn go_first(&mut self) -> Result<(), ContainerEmptyError>{
        
        // Make sure the list is not empty first
        if self.is_empty(){
            return Err(ContainerEmptyError)
        }

        // set the position to be the head of the list
        self.position = self.head;

        Ok(())
    }

    
	/// Sets the cursor of the list to the before position of the list, which is an abstract
    /// location before the first element in the list.
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// list.go_before();
    ///
    /// // The cursor is now in the before position of the list
    ///
	/// ```
    fn go_before(&mut self){
        self.position = ArrayedList::<T>::BEFORE_POS;
    }
    
    /// Sets the cursor of the list to the after position of the list, which is an abstract
    /// location after the last element in the list.
	///
	/// # Arguments
	/// * `&mut self` - A mutable reference to the list
	/// 
	/// # Example
	/// ```
	/// use kyles_algorithm_crate::lists::arrayed_list::ArrayedList;
    /// use kyles_algorithm_crate::base::linear_iterator::LinearIterator;
    ///
    /// let &mut list = ArrayedList::new(3);
    /// 
    /// list.go_after();
    ///
    /// // The cursor is now in the after position of the list
    ///
	/// ```
    fn go_after(&mut self){
        self.position = ArrayedList::<T>::AFTER_POS;
    }

}

impl<T> Dict<T> for ArrayedList<T>
    where T: Clone + Copy{

}


impl<T> fmt::Display for ArrayedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.capacity)
    }
}

#[cfg(test)]
mod test_arrayed_list {
    
    #[test]
    fn test_capacity(){
        use lists::arrayed_list::ArrayedList;
        
        let list: ArrayedList<i32> = ArrayedList::new(10);  
        
        assert_eq!(list.capacity(), 10);     
    }
    

    #[test]
    fn test_get_item_at_index(){
        use lists::arrayed_list::ArrayedList;
        use base::basic_dict::BasicDict;
         
        let mut list: ArrayedList<i32> = ArrayedList::new(5);

        // Insert 3 items
        list.insert(1).expect("Error in insert");
        list.insert(2).expect("Error in insert");
        list.insert(3).expect("Error in insert");
        
        // Get the three items
        let res = list.get_item_at_index(0);
        let res2 = list.get_item_at_index(1);
        let res3 = list.get_item_at_index(2);

        match res {
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match res2 {
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match res3 {
            Ok(3) => assert!(true),
            _ => assert!(false)
        }
        
        // See what happens when we get an out of bounds index
        match list.get_item_at_index(4){
            Err(_e) => assert!(true),
            _ => assert!(false)
        }

        // NOTE: Negative indices are not possible because of the u32 type parameter

    }

    #[test]
    fn test_insert_first(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        // Insert some elements
        match list.insert_first(1) {
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.insert_first(2) {
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        // The first item should be what we just inserted
        match list.get_item_at_index(0){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
         

        match list.insert_first(3){
             Ok(()) => assert!(true),
             _ => assert!(false)
        }

        match list.get_item_at_index(0){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        // Insert if list is full
        match list.insert_first(4){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

    }

    #[test]
    fn test_first_item(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Get the first item when the list is empty
        match list.first_item(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(1).expect("Error in insert_first");

        match list.first_item() {
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(2).expect("Error in insert_first");

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(3).expect("Error in go insert_last");

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_delete_first(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Delete first on empty list
        match list.delete_first(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        // Insert up to capacity, then delete first until empty
        list.insert_first(3).expect("Error in insert_first");
        list.insert_first(2).expect("Error in insert_first");
        list.insert_first(1).expect("Error in insert_first");

        match list.delete_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }
        
        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match list.delete_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item() {
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.delete_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        // error on first item because it should not exist
        match list.first_item(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        // One more delete on empty list to be safe
        match list.delete_first(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }
    
    #[test]
    fn test_insert_last(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Insert last until full
        match list.insert_last(1){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        match list.insert_last(2){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match list.insert_last(3){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        // Insert last on full list should error
        match list.insert_last(4){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        // Delete an item and insert last again to make sure
        list.delete_first().expect("Error in delete_first");

        match list.insert_last(4){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(4) => assert!(true),
            _ => assert!(false)
        }
    }   
    
    // TODO: Tests
    // last item
    #[test]
    fn test_last_item(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Last item on empty should give error
        match list.last_item(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        // Insert a few different items and check the last
        list.insert_last(1).expect("Error in go insert_last");

        match list.last_item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(2).expect("Error in go insert_last");

        match list.last_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(3).expect("Error in go insert_last");

        match list.last_item() {
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        
        // Last item should not change after deleting first
        list.delete_first().expect("Error in delete_first");

        match list.last_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(4).expect("Error in insert_first");

        match list.last_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

    }

    // delete last
    
    #[test]
    fn test_delete_last(){
        use lists::arrayed_list::ArrayedList;
        use lists::simple_list::SimpleList;
        use base::container::Container;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Try to delete from empty
        
        match list.delete_last(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        // Fill the list and then delete
        
        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(3).expect("Error in go insert_last");

        match list.delete_last(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }


        match list.delete_last(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item() {
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.delete_last(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        assert!(list.is_empty());
        
        // Delete from empty once more
        match list.delete_last(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }


    // obtain
    
    #[test]
    fn test_obtain(){
        use lists::arrayed_list::ArrayedList;
        use base::basic_dict::BasicDict;

        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Obtain on empty should error
        match list.obtain(1){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
        
        // Insert stuff
        list.insert(1).expect("Error in insert");
        
        match list.obtain(1){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert(2).expect("Error in insert");
        
        match list.obtain(1){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.obtain(2){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    
        list.insert(3).expect("Error in insert");

        match list.obtain(1){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.obtain(2){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match list.obtain(3){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        // Try to obtain a nonexistant element

        match list.obtain(4){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }


    // insert
    #[test]
    fn test_insert(){
        use lists::arrayed_list::ArrayedList;
        use base::basic_dict::BasicDict;
        use base::linear_iterator::LinearIterator;
        use lists::simple_list::SimpleList;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);


        match list.insert(1){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        list.go_first().expect("Error in go first");

        match list.insert(2){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }


        list.go_first().expect("Error in go first");

        match list.insert(3){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }


        // Over capacity
        match list.insert(4){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        list.go_after();

        match list.insert(50){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        list.go_before();

        match list.insert(50){
            Err(_) => assert!(true),
            _ => assert!(false)
        }


    }


    // delete
    
    #[test]
    fn test_delete(){
        use lists::arrayed_list::ArrayedList;
        use base::basic_dict::BasicDict;
        use base::membership::Membership;
        use lists::simple_list::SimpleList;
       
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        // Delete from empty should not work
        match list.delete(5){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
        
    
        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(3).expect("Error in go insert_last");
        

        match list.delete(1){
            Ok(()) => assert!(true),
            _  => assert!(false)
        }

        match list.has(1){
            false => assert!(true),
            true => assert!(false)
        }

        match list.delete(3){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.has(3){
            false => assert!(true),
            true  => assert!(false)
        }

        match list.delete(2){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.has(2){
            false => assert!(true),
            true => assert!(false)
        }


    }
    

    // search

    #[test]
    fn test_search(){
    
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        
        // Search empty list
        list.search(1);
        
        match list.item(){
            Ok(_) => assert!(false),
            _ => assert!(true)
        }

        
        // Add some items and search for each

        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(1).expect("Error in go insert_last");

        list.go_before();

        list.search(2);

        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.search(1);

        match list.item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }


    }


    // restart_searches
    #[test]
    fn test_restart_searches(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        list.restart_searches();

        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(1).expect("Error in go insert_last");

        list.search(1);

        list.go_forth().expect("Error in go first");

        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.search(1);
        
        list.go_forth().expect("Error in go forth");

        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    }


    // resume_searches
    
    #[test]
    fn test_resume_searches(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        list.resume_searches();

        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(1).expect("Error in go insert_last");

        list.search(1);

        list.go_forth().expect("Error in go forth");

        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.search(1);

        list.go_forth().expect("Error in go forth");

        if list.after(){
            assert!(true);
        }else{
            assert!(false);
        }


    }


    // has
    #[test]
    fn test_has(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(3).expect("Error in go insert_last");

        match list.has(1){
            true => assert!(true),
            _ => assert!(false)
        }
        
        match list.has(2){
            true => assert!(true),
            _ => assert!(false)
        }

        match list.has(3){
            true => assert!(true),
            _ => assert!(false)
        }


        match list.has(42){
            true => assert!(false),
            _ => assert!(true)
        }
    }


    // membership_equals
    #[test]
    fn test_membership_equals(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
       
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);

        list.insert_last(1).expect("Error in go insert_last");
        list.insert_last(2).expect("Error in go insert_last");
        list.insert_last(3).expect("Error in go insert_last");

        if list.membership_equals(1, 2){
            assert!(false);
        }else{
            assert!(true)
        }

        if list.membership_equals(2, 2){
            assert!(true);
        }else{
            assert!(false)
        }

    }

    // insert_item
    #[test]
    fn test_insert_item(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
       
        match list.insert_item(1){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        
        match list.item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.insert_item(2){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }


        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.last_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.go_first().expect("Error in go first");

        match list.insert_item(3) {
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        list.go_forth().expect("Error in go forth");

        match list.item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

    }


    // delete_item
    #[test]
    fn test_delete_item(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        match list.delete_item(){
            Ok(()) => assert!(false),
            _ => assert!(true)
        }

        list.insert_item(1).expect("Error in insert");
        list.insert_item(2).expect("Error in insert");
        list.insert_item(3).expect("Error in insert"); 

        list.go_first().expect("Error in go first");

        match list.delete_item(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.go_forth().expect("Error in go forth");

        match list.delete_item(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.item() {
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        match list.delete_item(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }
    
        assert!(list.after());

        match list.delete_item(){
            Ok(()) => assert!(false),
            _ => assert!(true)
        }

    }

    
    // is_empty
    #[test]
    fn test_is_empty(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        assert!(list.is_empty());

        list.insert_first(1).expect("Error in insert_first");

        assert!(!list.is_empty());

        list.delete_item().expect("Error in delete item");

        assert!(list.is_empty());

    }

    
    // is_full
    #[test]
    fn test_is_full(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
    
        assert!(!list.is_full());

        list.insert_first(1).expect("Error in insert_first");
        list.insert_first(2).expect("Error in insert_first");
        list.insert_first(3).expect("Error in insert_first");

        assert!(list.is_full());

        list.delete_item().expect("Error in delete_item");

        assert!(!list.is_full());


    }



    // clear
    #[test]
    fn test_clear(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        list.insert_first(1).expect("Error in insert_first");
        
        list.clear();
        
        assert!(list.is_empty());
        
        list.insert_first(1).expect("Error in insert_first"); 
        list.insert_first(2).expect("Error in insert_first");
        
        list.clear();
        
        assert!(list.is_empty()); 

    }



    // item
    #[test]
    fn test_item(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        match list.item(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        list.insert_item(1).expect("Error in insert_first");

        match list.item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }
    

        list.insert_item(2).expect("Error in insert_first");    
        list.insert_item(3).expect("Error in insert_first");

        match list.item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        list.go_first().expect("Error in go first");

        match list.item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.go_forth().expect("Error in fo forth");

        match list.item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    }

    

    // item_exists
    #[test]
    fn test_item_exists(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        assert!(!list.item_exists());

        list.insert_first(1).expect("Error in insert_first");
    
        assert!(!list.item_exists());

        list.go_first();
        assert!(list.item_exists());

        list.go_before();

        assert!(!list.item_exists());


        list.go_after();

        assert!(!list.item_exists());
    }



    // current_position
    #[test]
    fn test_current_position(){
        assert!(false);
        //TODO: I'm not sure how this is supposed to work yet        
    }


    // go_position
    #[test]
    fn test_go_position(){
        assert!(false);
        // TODO: I'm not sure how this is supposed to work yet
    }


    // before
    #[test]
    fn test_before(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        list.insert_first(1).expect("Error in insert_first");

        assert!(!list.before());

        list.go_before();

        assert!(list.before());

        list.go_after();

        assert!(!list.before());
    }

    // after
     #[test]
    fn test_after(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        list.insert_first(1).expect("Error in insert_first");

        assert!(!list.after());

        list.go_after();

        assert!(list.after());

        list.go_before();

        assert!(!list.after());
    }


    // go_forth
    #[test]
    fn test_go_forth(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        match list.go_forth(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        assert!(list.after());

        match list.go_forth(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(1).expect("Error in insert_first");
        
        list.go_before();

        match list.go_forth(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        assert_eq!(list.item().unwrap(), 1);

        match list.go_forth(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        assert!(list.after());

        match list.go_forth(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // go_first
    #[test]
    fn test_go_first(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        match list.go_first(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        assert!(!list.before());

        list.insert_first(0).expect("Error in insert_first");
        
        list.go_before();

        match list.go_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        assert_eq!(list.item().unwrap(), 1);

        match list.go_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        assert_eq!(list.item().unwrap(), 1);

        list.go_after();

        match list.go_first(){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        
        assert_eq!(list.item().unwrap(), 1);

        list.delete_item().expect("Error in delete item");

        match list.go_first(){
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }


    // go_before
    #[test]
    fn test_go_before(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        list.go_after();

        list.go_before();

        assert!(list.before());

        list.insert_first(1).expect("Error in insert_first");

        list.go_before();

        assert!(list.before());
               
    }

    // go_after
    #[test]
    fn test_go_after(){
        use lists::arrayed_list::ArrayedList;
        use base::searchable::Searchable;
        use lists::simple_list::SimpleList;
        use base::cursor::Cursor; 
        use base::linear_iterator::LinearIterator;
        use base::membership::Membership;
        use base::dispenser::Dispenser;       
        use base::container::Container;
        
        let mut list: ArrayedList<i32> = ArrayedList::new(3);
        
        list.go_before();

        list.go_after();

        assert!(list.after());

        list.insert_first(1).expect("Error in insert_first");

        list.go_after();

        assert!(list.after());
               
    }


}
