// Exercise 20: swapping elements of a vector in a cyclical manner

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt::Debug;

// utility

// The number element has a value and a flag that marks whether it moved or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MovableNumber {
    value : u32,
    has_moved : bool,
}

// Implementing a linked list, which is easy to swap elements with.
// I'm sure there is one already out there, but it's worth experimenting.
#[derive(Debug)]
struct LoopedLinkedListNode<T> {
    value: T,
    prev_element: Option<Box<LoopedLinkedListNode<T>>>,
    next_element: Option<Box<LoopedLinkedListNode<T>>>,
}   

pub struct LoopedLinkedList<T> {
    element_zero: Option<Box<LoopedLinkedListNode<T>>>,
    total_size : usize,
}

impl<T> LoopedLinkedList<T> where T: Copy + Debug {

    fn new()->LoopedLinkedList<T> {
        LoopedLinkedList{element_zero : None, total_size : 0}
    }

    fn get_element_at_index(&self, position : usize) -> Option<T> {
        
        if self.total_size == 0 {
            return None;
        }

        let list_index = position % self.total_size;

        let mut current = &self.element_zero;
        for _ in 0..list_index {
            if let Some(node) = current {
                current = &node.next_element;
            }
        }

        if let Some(node) = current {
            Some(node.value)
        } else {
            None
        }
    }

    fn add_in_position(&self, object : T, position : i32) {
        // Finding the position based on what is closest to the starting point.
        let loop_size_int = self.total_size as i32;
        let actual_position = (position + loop_size_int/2) % loop_size_int - loop_size_int/2;

        let new_node = Box::new(LoopedLinkedListNode {
            value: object,
            prev_element: None,
            next_element: None,
        });

        // If positive iterating in the front:
        let mut current = &mut self.element_zero;
        if actual_position >= 0 {
            for _ in 0..actual_position - 1 {
                if let Some(node) = current {
                    current = &mut node.next_element;
                }
            }
        }
        else {
            for _ in 0..actual_position.abs() - 1 {
                if let Some(node) = current {
                    current = &mut node.prev_element;
                }
            }
        }
        if let Some(node) = current {
                
            // Links on the new node
            new_node.next_element = node.next_element;
            new_node.prev_element = Some(node);

            // Updating links of the old nodes.
            if let Some(next_node) = node.next_element {
                next_node.prev_element = Some(new_node);
            }
            node.next_element = Some(new_node);
        }
    }

    fn print_list(&self) {
        let mut current = &mut self.element_zero;
        for _ in 0..self.total_size - 1 {
            if let Some(node) = current {
                current = &mut node.next_element;
                if let Some(next_node) = current {
                    println!("{:?}", next_node.value);
                }
            }
        }
    }

}

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
    let mut numbers_vec : Vec<MovableNumber> = lines_vec.iter()
        .map(|elem| MovableNumber{value: elem.parse().unwrap(), has_moved: false}).collect();
    let mut loopedList = LoopedLinkedList::<MovableNumber>::new();
    for element in numbers_vec {

    }


    result_part_1 = 0;
    result_part_2 = 0;
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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 21);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}