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
use std::io;
use std::io::Error;

 
/// A struct to represent an arrayed list structure
#[derive(Clone, Debug)]
pub struct ArrayedList<T>{
    list_elements: Vec<T>,
    head: i32,
    tail: i32,
    capacity: u32,
    num_el: u32,
    position: i32,
    continue_search: bool,
}

impl <T> ArrayedList<T>
    where T: Copy + Clone {
    
     
    pub fn new(capacity: u32) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        ArrayedList {list_elements: vec, head: 0, tail: 0, capacity: capacity, position: 0, continue_search: false}
    }
    
    pub fn capacity() -> u32 {
        return 0;
    }

    pub fn iterator() -> ArrayedListIterator<T>{
        return ArrayedListIterator::new();
    }

    pub fn get_item_at_index(index: u32) -> Result<T, InvalidArgumentError>{
        Err(InvalidArgumentError)
    }

}

impl<T> SimpleList<T> for ArrayedList<T>
    where T: Clone
{
    fn insert_first(x: T) -> Result<(), ContainerFullError>{
        Err(ContainerFullError)
    }

    fn first_item() -> Result<T, ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

    fn delete_first() -> Result<(), ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

    fn insert_last(x: T) -> Result<(), ContainerFullError>{
        Err(ContainerFullError)
    }

    fn last_item() -> Result<T, ContainerEmptyError>{
        Err(ContainerEmptyError)

    }

    fn delete_last() -> Result<(), ContainerEmptyError>{
        Err(ContainerEmptyError)
    }

}



    
impl<T> BasicDict<T> for ArrayedList<T>
    where T: Clone
{
    fn obtain(y: T) -> Result<T, ItemNotFoundError>{
        Err(ItemNotFoundError)
    }

    fn insert(x: T) -> Result<(), io::Error>{
        Err(Error)
    }

    fn delete(x: T) -> Result<(), ItemNotFoundError>{
        Err(ItemNotFoundError)
    }
 
}

impl<T> Searchable<T> for ArrayedList<T>
    where T: Clone
{
    fn search(x: T){

    }

    fn restart_searches(){

    }

    fn resume_searches(){

    }

}

impl<T> Membership<T> for ArrayedList<T>
    where T: Clone
{
    fn has(x: T) -> bool{
        false
    }

    fn membership_equals(x: T, y: T) -> bool{
        false
    }

}

impl<T> Dispenser<T> for ArrayedList<T>
    where T: Clone
{
    fn insert(x: T) -> Result<(), Error>{
        Err(Error)
    }
    
    fn delete_item() -> Result<(), NoCurrentItemError>{
        Err(NoCurrentItemError)
    }

}

impl<T> Container for ArrayedList<T>
    where T: Clone
{
    fn is_empty() -> bool{
        false
    }
    
    fn is_full() -> bool{
        false
    }
    
    fn clear(){

    }
}

impl<T> Cursor<T> for ArrayedList<T>
    where T: Clone
{
    
    fn item() -> Result<T, NoCurrentItemError>{
        Err(NoCurrentItemError)
    }
    
    fn item_exists() -> bool{
        false
    }
}


impl<T> CursorSaving for ArrayedList<T>
    where T: Clone
{
  
    fn current_position() -> Box<CursorPosition>{
        let cursor_pos = ArrayedListIterator::new();
        Box::new(cursor_pos)
    }
    
  
    fn go_position(pos: CursorPosition){

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
    where T: Clone{

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
    // TODO: Write tests
}

