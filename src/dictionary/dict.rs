use base::basic_dict::BasicDict;
use base::searchable::Searchable;
use base::dispenser::Dispenser;
use base::linear_iterator::LinearIterator;
use base::cursor_saving::CursorSaving;

/// Represents a dictionary with search, and basic insert and delete functions
pub trait Dict<T>: BasicDict<T> + Searchable<T> + Dispenser<T> + LinearIterator + CursorSaving {

} 
