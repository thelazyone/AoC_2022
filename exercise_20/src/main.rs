// Exercise 20: swapping elements of a vector in a cyclical manner

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Other useful includes:
use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;


// utility

// The number element has a value and a flag that marks whether it moved or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MovableNumber {
    value : i64,
    shift_order : usize,
}

// Implementing a linked list, which is easy to swap elements with.
// I'm sure there is one already out there, but it's worth experimenting.

type ListLink<T> = Option<Rc<RefCell<LoopedLinkedListNode<T>>>>;

#[derive(Debug, Clone)]
struct LoopedLinkedListNode<T> {
    value: T,
    next_element: ListLink<T>,
}   

#[derive(Debug, Clone)]
pub struct LoopedLinkedList<T> {
    first_element: ListLink<T>,
    last_element: ListLink<T>,
    total_size : usize,
}

impl<T> LoopedLinkedList<T> where T: Copy + Debug {

    fn new()->LoopedLinkedList<T> {
        LoopedLinkedList{first_element : None, last_element : None, total_size : 0}
    }

    fn get_element_at_index(&self, position : usize) -> Option<T> {
        
        if self.total_size == 0 {
            return None;
        }

        let list_index = position % self.total_size;

        let mut current = self.first_element.clone();
        for _ in 0..list_index {
            current = current.unwrap().borrow().next_element.clone();
        }

        if let Some(node) = current {
            let out_element = node.borrow().value;
            Some(node.borrow().value)
        } else {
            None
        }
    }

    fn add_in_position(&mut self, object: T, position: i64) {

        let new_node = Rc::new(RefCell::new(LoopedLinkedListNode {
            value: object,
            next_element: None,
        }));

        // If it's the first element, handling it differently.
        if self.total_size == 0 {
            let mut node_ref = (*new_node.borrow_mut()).clone();
            node_ref.next_element = None;
            self.first_element = Some(new_node.clone());
            self.last_element = Some(new_node.clone());
            self.total_size = 1;

            return;
        }

        // Adding in front or back is easy. Apologies for the code duplication.
        let position = position % (self.total_size + 1) as i64;
        if position == 0 {
            // Adding the next element as the first in the current list.
            new_node.borrow_mut().next_element = Some(self.first_element.clone().unwrap());
            self.first_element = Some(new_node.clone());
        } 
        else if position == -1 || position == self.total_size as i64 - 1 {
            // Updating the "next" of the last element
            self.last_element.clone().unwrap().borrow_mut().next_element = Some(new_node.clone());
            self.last_element = Some(new_node.clone());
        }
        else {
            // Getting to position -1 to set the next element of current:
            let mut current = self.first_element.clone();
            for _ in 0..position - 1  {
                current = current.unwrap().borrow().next_element.clone();
            }
            current.as_ref().unwrap().borrow_mut().next_element = Some(new_node.clone());

            // Moving 1 forward and setting that as next element of the new node
            current = current.unwrap().borrow().next_element.clone();
            new_node.borrow_mut().next_element = Some(current.clone().unwrap());
        }

        self.total_size += 1;
    }

    fn _print_list(&self) {
        let mut current = self.first_element.clone();
        for list_idx in 0..self.total_size  {
            println!("List element {} : {:?}", list_idx, current.as_ref().unwrap().borrow().value);
            current = current.unwrap().borrow().next_element.clone();
        }
    }

}


// Given a dimension and a movement direction, the result can 
fn get_position_circular_module(signed_movement_direction : i64, list_length : usize) -> usize {

    // unwrapping the movement by calculating the module. Module doesn't work well with negative numbers.
    let modulus = list_length as i64 - 1;
    //let unwrapped_movement = (signed_movement_direction) + modulus * (1 + signed_movement_direction.abs() / modulus) ;
    let unwrapped_movement = ((signed_movement_direction % modulus) + modulus) % modulus;

    unwrapped_movement as usize 
}

// Functions specific for the current problem:
fn apply_shift (list : & mut LoopedLinkedList<MovableNumber>, shifts_number : usize) {
    
    // Applying the shift multiple times if necessary.
    for _ in 0..shifts_number {

        // For each element:
        // 1 - retrieving the "next" in line for the next shift
        // 2 - applying the shift
        for element_idx in 0..list.total_size {
            shift_first_available_by(list, element_idx);
            //list.print_list();
        }
    }

}

fn shift_first_available_by (list: &mut LoopedLinkedList<MovableNumber>, shift_order : usize) {
    
    // Starting from the first element
    let mut previous = None;
    let mut current = list.first_element.clone();

    // Finding the first "shiftable" element in the list: 
    let is_first_moving = current.as_ref().unwrap().borrow().value.shift_order == shift_order;

    for _ in 0..list.total_size {
        if current.as_ref().unwrap().borrow().value.shift_order == shift_order {
            break;
        }
        previous = current.clone();
        current = current.unwrap().borrow().next_element.clone();
    }

    // Retrieving the wrapped shift value. However, IF the movement is backwards the element might end up becoming
    // the new first element
    let shift_value = get_position_circular_module(current.as_ref().unwrap().borrow().value.value, list.total_size);

    // If movement is zero, we can return.
    if shift_value == 0 {
        return;
    }

    // If we're moving the first element, unless the movement was Zero the object next to it becomes first.
    if is_first_moving && shift_value != 0 {
        list.first_element = Some(current.as_ref().unwrap().borrow().next_element.clone().unwrap());
    }

    // finding the target node. If reaching next_element = None, starting from the beginning
    let mut node_cursor = current.clone();
    for _ in 0..shift_value { 

        // If None, setting the node cursor to the beginning again.
        if let None = node_cursor.clone().unwrap().borrow().next_element {
            node_cursor = list.first_element.clone();
        }
        else {
            node_cursor = node_cursor.unwrap().borrow().next_element.clone();
        }
    }

    // Linking the current node to the one after the target, and the target to the current node.
    if let Some(previous_node) = previous {
        previous_node.as_ref().borrow_mut().next_element = current.as_ref().unwrap().borrow().next_element.clone();
    }
    current.as_ref().unwrap().borrow_mut().next_element = node_cursor.as_ref().unwrap().borrow().next_element.clone();
    node_cursor.as_ref().unwrap().borrow_mut().next_element = current.clone();
}

fn find_index_with_value_0 (list: &LoopedLinkedList<MovableNumber>) -> usize {
    let mut current = list.first_element.clone();

    // Finding the first element in the list with value 0 (i'm assuming it's the only one, actually): 
    let mut zero_found = false;
    let mut found_index : usize = 0;
    for index in 0..list.total_size {
        if current.as_ref().unwrap().borrow().value.value == 0 {
            if zero_found == true {
                panic!("More than one element is zero in this set!");
            }

            zero_found = true;
            found_index = index;
        }
        current = current.unwrap().borrow().next_element.clone();
    }

    if !zero_found {
        panic!("No zero found in the list!");
    }

    found_index
}

fn create_list_from_strings_vect (input_vec : &Vec<String>, multiplication_factor : i64) -> LoopedLinkedList<MovableNumber> {
    
    let numbers_vec : Vec<MovableNumber> = input_vec.iter().enumerate()
        .map(|(i, elem)| {
            let value : i64 = elem.parse().unwrap();
            MovableNumber{value: value * multiplication_factor, shift_order: i}
        }).collect();    
    let mut looped_list = LoopedLinkedList::<MovableNumber>::new();
    for element in numbers_vec {
        looped_list.add_in_position(element, -1);
    }
    
    looped_list
}


// Using an application-specific approach rather than implementing a linked list: 
// While the list is a potentially elastic approach, moving N numbers N times has a 
// complexity of at least N^2.
// Instead, we only need to know what's happening on points 1000, 2000 and 3000! 

// Primary Function
fn execute (input_path : String)  -> Option<(u32, u64)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : u32;
    let result_part_2 : u64;

    // First reading the input string - easy.
    let mut lines_vec = Vec::<String>::new();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            lines_vec.push(line);
        }
    }
    println!("read {} lines from input", lines_vec.len());
    assert!(lines_vec.len() > 1);

    // Converting to number. The step above is unnecessary, but this uniforms the various exercises.
    println!("Executing part 1...");
    let mut looped_list = create_list_from_strings_vect(&lines_vec, 1);
    apply_shift(& mut looped_list, 1 /* shift order, see Part 2 */);

    // Retrieving the values at 1000, 2000 and 3000
    let zero_index = find_index_with_value_0(&looped_list);
    println!("zero index of the list is at {}", zero_index);
    result_part_1 = 
        (looped_list.get_element_at_index(1000 + zero_index).unwrap().value + 
        looped_list.get_element_at_index(2000 + zero_index).unwrap().value + 
        looped_list.get_element_at_index(3000 + zero_index).unwrap().value) as u32;

    // The shift to be applied now has to be done ten times, but keeping the original logic.
    // This requires modifying the "apply shift" logic a bit, with a degenerate case of n=1
    // Also, all the values must first be multiplied by 811589153.    
    println!("Executing part 2...");
    let mut looped_list = create_list_from_strings_vect(&lines_vec, 811589153);
    apply_shift(& mut looped_list, 10 /* shift order, see Part 2 */);
    

    // Retrieving the values at 1000, 2000 and 3000
    let zero_index = find_index_with_value_0(&looped_list);
    println!("zero index of the list is at {}", zero_index);
    result_part_2 = 
        (looped_list.get_element_at_index(1000 + zero_index).unwrap().value + 
        looped_list.get_element_at_index(2000 + zero_index).unwrap().value + 
        looped_list.get_element_at_index(3000 + zero_index).unwrap().value) as u64;

    Some((result_part_1, result_part_2))
}


// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 20!");

    let results = execute("./data/input.txt".to_string()).unwrap();
    
    println!("Part 1 result is {}.", results.0);
    println!("Part 2 result is {}.", results.1);

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // General Test
    #[test]
    fn global_test_part_1() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 3);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 1623178306);
    }    
}