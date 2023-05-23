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
    value : i32,
    has_moved : bool,
}

// Implementing a linked list, which is easy to swap elements with.
// I'm sure there is one already out there, but it's worth experimenting.

type ListLink<T> = Option<Rc<RefCell<LoopedLinkedListNode<T>>>>;

#[derive(Debug, Clone)]
struct LoopedLinkedListNode<T> {
    value: T,
    next_element: ListLink<T>,
}   

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
            Some(node.borrow().value)
        } else {
            None
        }
    }

    fn add_in_position(&mut self, object: T, position: i32) {


        let new_node = Rc::new(RefCell::new(LoopedLinkedListNode {
            value: object,
            next_element: None,
        }));

        // If it's the first element, handling it differently.
        if self.total_size == 0 {
            let mut node_ref = (*new_node.borrow_mut()).clone();
            node_ref.next_element = None;
            println!("NREF: {:?}", node_ref );
            self.first_element = Some(new_node.clone());
            self.last_element = Some(new_node.clone());
            self.total_size = 1;
            println!("debug: {:?}", self.first_element );
            return;
        }

        // Adding in front or back is easy. Apologies for the code duplication.
        let position = position % self.total_size as i32;

        println!("calling ADD, size is {}, position is {}", self.total_size, position);

        if position == 0 || position == -1 || position == self.total_size as i32 - 1{
            // Adding the next element as the first in the current list.
            let mut node_ref = (*new_node.borrow_mut()).clone();
            node_ref.next_element = self.first_element.clone();

            // Updating the "next" of the last element
            let last_element_ref = self.last_element.clone().unwrap();
            let mut last_element_ref = (*last_element_ref.borrow_mut()).clone();
            last_element_ref.next_element = Some(new_node.clone());

            // Updating "first element", last element doesn't need it.
            if position == 0 {
                self.first_element = Some(new_node.clone());
            }
            else {
                self.last_element = Some(new_node.clone());
            }

            println!("debug: {:?}", node_ref );
        } 
        else {
            // Do nothing for now TODO

            // // Iterating on the list elements:
            // let mut current = self.first_element.clone();
            // for _ in 0..position - 1 {
            //     current = current.unwrap().borrow().next_element.clone();
            // }

            // // Adding the new node in position.
            // let mut current_node = current.unwrap();
            // new_node.get_mut().next_element = current_node.borrow().next_element.clone();
            // current_node.get_mut().next_element = Some(new_node);
        }

        self.total_size += 1;
    }


    fn print_list(&self) {
        let mut current = self.first_element.clone();
        for list_idx in 0..self.total_size - 1 {
            println!("List element {} : {:?}", list_idx, current.as_ref().unwrap().borrow());
            current = current.unwrap().borrow().next_element.clone();
        }
    }

}



// Using an application-specific approach rather than implementing a linked list: 
// While the list is a potentially elastic approach, moving N numbers N times has a 
// complexity of at least N^2.
// Instead, we only need to know what's happening on points 1000, 2000 and 3000! 

// Primary Function
fn execute (input_path : String)  -> Option<(u32, u32)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : u32;
    let result_part_2 : u32;

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
    let numbers_vec : Vec<MovableNumber> = lines_vec.iter()
        .map(|elem| {let value : i32 = elem.parse().unwrap(); MovableNumber{value: value, has_moved: false}}).collect();
    let mut loopedList = LoopedLinkedList::<MovableNumber>::new();
    for element in numbers_vec {
        loopedList.add_in_position(element, -1);
    }
    loopedList.print_list();


    result_part_1 = 0;
    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 20!");

    let results = execute("./data/test.txt".to_string()).unwrap();
    
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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 21);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}