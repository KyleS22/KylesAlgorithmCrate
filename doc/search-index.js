var searchIndex = {};
searchIndex["kyles_algorithm_crate"] = {"doc":"","items":[[0,"base","kyles_algorithm_crate","",null,null],[0,"cursor_position","kyles_algorithm_crate::base","",null,null],[8,"CursorPosition","kyles_algorithm_crate::base::cursor_position","Interface to represent an arbitrary cursor.   Allows user to ignore the type of the cursor.",null,null],[0,"linear_iterator","kyles_algorithm_crate::base","",null,null],[8,"LinearIterator","kyles_algorithm_crate::base::linear_iterator","An iterator for linearly moving through a collection of items. Uses a cursor to keep track of the current position, and has functions to move around in the sequence",null,null],[10,"before","","Is the current position before the start of the structure?",0,{"o":{"n":"bool"}}],[10,"after","","Is the current position after the end of the structure?",0,{"o":{"n":"bool"}}],[10,"go_forth","","Advance one item in the data structure. Returns an AfterTheEndError if the cursor goes after the end of the structure",0,{"o":{"g":["aftertheenderror"],"n":"result"}}],[10,"go_first","","Sets the cursor to the first element in the structure Returns a ContainerEmptyError if the structure is empty",0,{"o":{"g":["containeremptyerror"],"n":"result"}}],[10,"go_before","","Move the cursor to the position before the first element of the structure",0,{}],[10,"go_after","","Move the cursor to the position after the last element of the structure",0,{}],[0,"bilinear_iterator","kyles_algorithm_crate::base","",null,null],[8,"BilinearIterator","kyles_algorithm_crate::base::bilinear_iterator","An iterator that can move forward and backward",null,null],[10,"go_last","","Go to the last item of the interface",1,{}],[10,"go_back","","Move back one item in the structure Throws a BeforeTheStartError if the cursor is before the start of the structure",1,{"o":{"g":["beforethestarterror"],"n":"result"}}],[0,"container","kyles_algorithm_crate::base","",null,null],[8,"Container","kyles_algorithm_crate::base::container","Represents a structure that contains things",null,null],[10,"is_empty","","Returns true if the container is empty, false otherwise",2,{"o":{"n":"bool"}}],[10,"is_full","","Returns true if the container is full, false otherwise",2,{"o":{"n":"bool"}}],[10,"clear","","Remove all items from the structure",2,{}],[0,"cursor","kyles_algorithm_crate::base","",null,null],[8,"Cursor","kyles_algorithm_crate::base::cursor","Represents a cursor that points to an item",null,null],[10,"item","","Returns the current item, or a NoCurrentItemError if there is no item",3,{"o":{"g":["nocurrentitemerror"],"n":"result"}}],[10,"item_exists","","Returns true if there is an item, false otherwise",3,{"o":{"n":"bool"}}],[0,"cursor_saving","kyles_algorithm_crate::base","",null,null],[8,"CursorSaving","kyles_algorithm_crate::base::cursor_saving","Interface for getting the current position of a cursor and go to a specified cursor position",null,null],[10,"current_position","","Get the current position in the structure",4,{"o":{"n":"cursorposition"}}],[10,"go_position","","Go to the given position",4,{"i":[{"n":"cursorposition"}]}],[0,"dispenser","kyles_algorithm_crate::base","",null,null],[8,"Dispenser","kyles_algorithm_crate::base::dispenser","A container that keeps track of the current item as inserts are made Only the current item can be deleted",null,null],[10,"insert","","Insert an item into the structure Returns a DuplicateItemError or ContainerFullError",5,{"i":[{"n":"t"}],"o":{"g":["error"],"n":"result"}}],[10,"delete_item","","Delete the current item from the structure Returns a NoCurrentItemError if there is no current item",5,{"o":{"g":["nocurrentitemerror"],"n":"result"}}],[0,"membership","kyles_algorithm_crate::base","",null,null],[8,"Membership","kyles_algorithm_crate::base::membership","A container with a membership test and equality tests",null,null],[10,"has","","Returns true if the container contains the element y",6,{"i":[{"n":"t"}],"o":{"n":"bool"}}],[10,"membership_equals","","Determines if two elements of type T are equal",6,{"i":[{"n":"t"},{"n":"t"}],"o":{"n":"bool"}}],[0,"ndpoint","kyles_algorithm_crate::base","",null,null],[3,"NDPoint","kyles_algorithm_crate::base::ndpoint","A representation of an N-dimensional point",null,null],[11,"new","","Create a new N-Dimensional point at the origin",7,{"i":[{"n":"i32"}],"o":{"n":"self"}}],[11,"from_coordinate","","Create a new point from the given coordinate, given as a vector of floats. The dimensionality of the point is equal to the length of the vector",7,{"i":[{"g":["f32"],"n":"vec"}],"o":{"n":"self"}}],[11,"dim","","Get the dimensionality of the point",7,{"i":[{"n":"self"}],"o":{"n":"i32"}}],[11,"set_point","","Set the coordinates of this point Returns InvalidArgumentError if the dimension of the given point is less than 1",7,{"i":[{"n":"self"},{"g":["f32"],"n":"vec"}],"o":{"g":["invalidargumenterror"],"n":"result"}}],[11,"idx","","Return the ith coodinate of the point Returns InvalidArgumentError if the given index is greater than the dimensionality of the point",7,{"i":[{"n":"self"},{"n":"i32"}],"o":{"g":["f32","invalidargumenterror"],"n":"result"}}],[11,"compare_by_dim","","Compares the ith coordinate of this point to another point Returns -1 if the ith coordinate of this point is smaller than that of 'other' Returns 0 if the ith coordinate of this point is equal to that of 'other' Returns 1 if the ith coordinate of this point is greater than that of 'other' Returns InvalidArgumentError if the other point has a different dimensionality or the given     dimension is greater than that of this point",7,{"i":[{"n":"self"},{"n":"i32"},{"n":"ndpoint"}],"o":{"g":["i8","invalidargumenterror"],"n":"result"}}],[11,"eq","","",7,{"i":[{"n":"self"},{"n":"ndpoint"}],"o":{"n":"bool"}}],[11,"partial_cmp","","Lexicographical comparison by dimension",7,{"i":[{"n":"self"},{"n":"ndpoint"}],"o":{"g":["ordering"],"n":"option"}}],[11,"fmt","","",7,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"custom_errors","kyles_algorithm_crate","",null,null],[0,"after_the_end_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"AfterTheEndError","kyles_algorithm_crate::custom_errors::after_the_end_error","An error that occurs when a cursor is after the end of the structure",null,null],[11,"fmt","","",8,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",8,{"i":[{"n":"self"}],"o":{"n":"aftertheenderror"}}],[11,"fmt","","",8,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",8,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",8,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"before_the_start_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"BeforeTheStartError","kyles_algorithm_crate::custom_errors::before_the_start_error","An error that occurs when a cursor is before the start of the structure",null,null],[11,"fmt","","",9,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",9,{"i":[{"n":"self"}],"o":{"n":"beforethestarterror"}}],[11,"fmt","","",9,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",9,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",9,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"container_empty_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"ContainerEmptyError","kyles_algorithm_crate::custom_errors::container_empty_error","An error that occurs when a container is empty and an operation is attempted on its elements",null,null],[11,"fmt","","",10,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",10,{"i":[{"n":"self"}],"o":{"n":"containeremptyerror"}}],[11,"fmt","","",10,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",10,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",10,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"container_full_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"ContainerFullError","kyles_algorithm_crate::custom_errors::container_full_error","An error that occurs when a container is full and elements are inserted",null,null],[11,"fmt","","",11,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",11,{"i":[{"n":"self"}],"o":{"n":"containerfullerror"}}],[11,"fmt","","",11,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",11,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",11,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"duplicate_items_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"DuplicateItemsError","kyles_algorithm_crate::custom_errors::duplicate_items_error","An error that occurs when there are duplicate items",null,null],[11,"fmt","","",12,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",12,{"i":[{"n":"self"}],"o":{"n":"duplicateitemserror"}}],[11,"fmt","","",12,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",12,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",12,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"invalid_argument_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"InvalidArgumentError","kyles_algorithm_crate::custom_errors::invalid_argument_error","An error that occurs when an invalid argument is given",null,null],[11,"fmt","","",13,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",13,{"i":[{"n":"self"}],"o":{"n":"invalidargumenterror"}}],[11,"fmt","","",13,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",13,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",13,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"invalid_state_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"InvalidStateError","kyles_algorithm_crate::custom_errors::invalid_state_error","An error that occurs when a structure is in an invalid state",null,null],[11,"fmt","","",14,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",14,{"i":[{"n":"self"}],"o":{"n":"invalidstateerror"}}],[11,"fmt","","",14,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",14,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",14,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"item_not_found_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"ItemNotFoundError","kyles_algorithm_crate::custom_errors::item_not_found_error","An error that occurs when an item is not found in the structure",null,null],[11,"fmt","","",15,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",15,{"i":[{"n":"self"}],"o":{"n":"itemnotfounderror"}}],[11,"fmt","","",15,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",15,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",15,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}],[0,"no_current_item_error","kyles_algorithm_crate::custom_errors","",null,null],[3,"NoCurrentItemError","kyles_algorithm_crate::custom_errors::no_current_item_error","An error that occurs when there is no current item at the cursor",null,null],[11,"fmt","","",16,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",16,{"i":[{"n":"self"}],"o":{"n":"nocurrentitemerror"}}],[11,"fmt","","",16,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"description","","",16,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"cause","","",16,{"i":[{"n":"self"}],"o":{"g":["error"],"n":"option"}}]],"paths":[[8,"LinearIterator"],[8,"BilinearIterator"],[8,"Container"],[8,"Cursor"],[8,"CursorSaving"],[8,"Dispenser"],[8,"Membership"],[3,"NDPoint"],[3,"AfterTheEndError"],[3,"BeforeTheStartError"],[3,"ContainerEmptyError"],[3,"ContainerFullError"],[3,"DuplicateItemsError"],[3,"InvalidArgumentError"],[3,"InvalidStateError"],[3,"ItemNotFoundError"],[3,"NoCurrentItemError"]]};
initSearch(searchIndex);
