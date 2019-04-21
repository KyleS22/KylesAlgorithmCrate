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
use std::fmt::Error;
 
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

impl <T> ArrayedList<T>
    where T: Copy + Clone, {
    
     
    pub fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        ArrayedList {list_elements: vec, head: 0, tail: 0, capacity: capacity, position: 0, num_el: 0,  continue_search: false}
    }
    
    pub fn capacity(&self) -> usize {
        return 0;
    }

    pub fn iterator(&self) -> ArrayedListIterator<T>{
        return ArrayedListIterator::new(self.list_elements.clone(), self.head, self.tail, self.num_el);
    }

    pub fn get_item_at_index(&self, index: u32) -> Result<T, InvalidArgumentError>{
        Err(InvalidArgumentError)
    }

}

impl<T> SimpleList<T> for ArrayedList<T>
    where T: Clone + Copy
{
    fn insert_first(&mut self, x: T) -> Result<(), ContainerFullError>{
        Err(ContainerFullError)
    }

    fn first_item(&self) -> Result<T, ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

    fn delete_first(&mut self) -> Result<(), ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

    fn insert_last(&mut self, x: T) -> Result<(), ContainerFullError>{
        Err(ContainerFullError)
    }

    fn last_item(&self) -> Result<T, ContainerEmptyError>{
        Err(ContainerEmptyError)

    }

    fn delete_last(&mut self) -> Result<(), ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

}



    
impl<T> BasicDict<T> for ArrayedList<T>
    where T: Clone
{
    fn obtain(&self, y: T) -> Result<T, ItemNotFoundError>{
        Err(ItemNotFoundError)
    }

    // inserts at the current position 
    fn insert(&mut self, x: T) -> Result<(), Error>{
        Err(Error)
    }

    fn delete(&mut self, x: T) -> Result<(), ItemNotFoundError>{
        Err(ItemNotFoundError)
    }
 
}

impl<T> Searchable<T> for ArrayedList<T>
    where T: Clone
{
    fn search(&self, x: T){

    }

    fn restart_searches(&mut self){

    }

    fn resume_searches(&mut self){

    }

}

impl<T> Membership<T> for ArrayedList<T>
    where T: Clone
{
    fn has(&self, x: T) -> bool{
        false
    }

    fn membership_equals(&self, x: T, y: T) -> bool{
        false
    }

}

impl<T> Dispenser<T> for ArrayedList<T>
    where T: Clone
{
    //fn insert(&mut self, x: T) -> Result<(), Error>{
    //    Err(Error)
    //}
    
    // delets at the current position    
    fn delete_item(&mut self) -> Result<(), NoCurrentItemError>{
        Err(NoCurrentItemError)
    }

}

impl<T> Container for ArrayedList<T>
    where T: Clone
{
    fn is_empty(&self) -> bool{
        false
    }
    
    fn is_full(&self) -> bool{
        false
    }
    
    fn clear(&mut self){

    }
}

impl<T> Cursor<T> for ArrayedList<T>
    where T: Clone
{
    
    fn item(&self) -> Result<T, NoCurrentItemError>{
        Err(NoCurrentItemError)
    }
    
    fn item_exists(&self) -> bool{
        false
    }
}


impl<T> CursorSaving for ArrayedList<T>
    where T: Clone + Copy
{
  
    fn current_position(&self) -> Box<CursorPosition>{
        let cursor_pos = ArrayedListIterator::new(self.list_elements.clone(), self.head, self.tail, self.num_el);
        Box::new(cursor_pos)
    }
    
  
    fn go_position(&mut self, pos: &CursorPosition){

    }

}

impl<T> LinearIterator for ArrayedList<T>
    where T: Clone
{

    fn before(&self) -> bool{
        return false
    }

    fn after(&self) -> bool{
        return false
    }


    fn go_forth(&mut self) -> Result<(), AfterTheEndError>{
        Err(AfterTheEndError)
    }
    
    fn go_first(&mut self) -> Result<(), ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

    fn go_before(&mut self){

    }
    
    fn go_after(&mut self){

    }

}

impl<T> Dict<T> for ArrayedList<T>
    where T: Clone + Copy{

}

impl<T> CursorPosition for ArrayedList<T>
    where T: Clone
{

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
        list.insert(1);
        list.insert(2);
        list.insert(3);
        
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

        list.insert_first(1);

        match list.first_item() {
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(2);

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(3);

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
        list.insert_first(3);
        list.insert_first(2);
        list.insert_first(1);

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
        list.delete_first();

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
        list.insert_last(1);

        match list.last_item(){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(2);

        match list.last_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }

        list.insert_last(3);

        match list.last_item() {
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        
        // Last item should not change after deleting first
        list.delete_first();

        match list.last_item(){
            Ok(3) => assert!(true),
            _ => assert!(false)
        }

        list.insert_first(4);

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
        
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);

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
        list.insert(1);
        
        match list.obtain(1){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        list.insert(2);
        
        match list.obtain(1){
            Ok(1) => assert!(true),
            _ => assert!(false)
        }

        match list.obtain(2){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }
    
        list.insert(3);

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

        list.go_first();

        match list.insert(2){
            Ok(()) => assert!(true),
            _ => assert!(false)
        }

        match list.first_item(){
            Ok(2) => assert!(true),
            _ => assert!(false)
        }


        list.go_first();

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
        
    
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);
        

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
    // restart_searches
    // resume_searches
    //
    // has
    // membership_equals
    //
    // delete_item
    //
    // is_empty
    // is_full
    // clear
    //
    // item
    // item_exists
    //
    // current_position
    // go_position
    //
    // before
    // after
    // go_forth
    // go_first
    // go_before
    // go_after
}
