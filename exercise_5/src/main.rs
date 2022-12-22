// Exercise 5: Move crates in a certain order!

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;


// Parsing the syntax: move X from Y to Z
fn parse_instruction (input : &str) -> Option<(u32, u32, u32)> {
    let regex_string = 
        r"move (?P<val1>\d+) from (?P<val2>\d+) to (?P<val3>\d+)";
    let regex = Regex::new(regex_string).unwrap();
    match regex.captures(input) {
        Some(caps) => {
            let internal_parse = |key| {
                caps.name(key).unwrap().as_str().parse::<u32>().unwrap()
            };
            let val1 = internal_parse("val1");
            let val2 = internal_parse("val2");
            let val3 = internal_parse("val3");
            Some((val1, val2, val3))
        }
        None => None,
    }
}


// Applying the movement (for Part 1)
fn apply_single_movement (from : usize, to: usize, layout: &mut Vec<Vec<char>>) {

    // This is the way I wanted to do it, but the borrowing rules are blocking me. Oh well.
    // if let Some(val) = layout[from].last() { 
    //     layout[from].pop();
    //     layout[to].push(*val)
    // }

    // Get the last element of column
    let val = layout[from].last().unwrap().clone();
    layout[from].pop();
    layout[to].push(val)
}


fn apply_movements (amount: u32, from : usize, to: usize, layout: &mut Vec<Vec<char>>) {
    for _ in 0..amount {
        apply_single_movement(from, to, layout);
    }
}


// For Part 2.
fn apply_movements_together (amount: u32, from : usize, to: usize, layout: &mut Vec<Vec<char>>) {

    // retrieving the last N elements
    let layout_size = layout[from].len();
    let elements = layout[from].as_slice()[layout[from].len()-(amount as usize)..].to_vec();

    // popping the last elements
    layout[from].truncate(layout_size - amount as usize);

    // pushing the elements in the other pile:
    layout[to].extend(elements);
}


// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 4!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // First reading the inputs 

    // Using a vec of vec, since the stacks are always moved from one end.
    // I gave a peek to the input and decided to make it sized FOR the specific input.
    let mut layout_lines_vec = Vec::<String>::new();
    let mut crates_layout = Vec::<Vec<char>>::new();

    // The order is stored in a tuple: amount of elements, from where, to where.
    let mut crates_instructions = Vec::<(u32, u32, u32)>::new();

    // Since there are two sections in the file I'm implementing a basic state machine.
    let mut section_num = 0;

    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            match section_num {
                0 => {
                // The empty line marks the start of the second part of the parsing.
                if line.is_empty() {
                    section_num = 1;
                    continue;
                }
                layout_lines_vec.push(line.to_owned());
                },

                1 => {
                    // Parsing with a regex.
                    crates_instructions.push(parse_instruction(&line.to_owned()).unwrap());
                },
                _ => panic!("There should be only two sections!"),
            }
        }
    }

    // Interpreting the layout.
    // Reading the vector from the bottom - the first line is a counter of the elements.
    let mut layout_iter = layout_lines_vec.into_iter().rev();
    let first_line = layout_iter.next().unwrap();
    let temp_vec: Vec<&str> = first_line.split("   ").collect();
    let stacks_number = temp_vec.len();
    println!("there are {} stacks of crates", stacks_number);
    crates_layout.resize(stacks_number, Vec::<char>::new());
    while let Some(item) = layout_iter.next() {
        for stack_idx in 0..stacks_number {
            // Extracting the character if it exists:
            let selected_character = item.chars().nth((1 + 4 * (stack_idx as u32)).try_into().unwrap()).unwrap();
            if selected_character != ' ' {
                crates_layout[stack_idx].push(selected_character);
            } 
        }
    }

    // Copying the layout for the processing
    let mut crates_layout_part_1 = crates_layout.clone();

    // Iterating over the instructions:
    for element in &crates_instructions {
        //println!("Applying movements: {} from {} to {}", element.0, element.1, element.2);
        apply_movements(
            element.0, //amount
            TryInto::<usize>::try_into(element.1 - 1).unwrap(), // from 
            TryInto::<usize>::try_into(element.2 - 1).unwrap(), // to
            &mut crates_layout_part_1
        )
    }

    // Extracting the last element from each stack:
    let mut part_1_result : String = "".to_owned();
    for stack_idx in 0..stacks_number {
        part_1_result.push(crates_layout_part_1[stack_idx].last().unwrap().to_owned());
    }
    println!("\nPart 1 Result is {}.", part_1_result);

    // For Part 2 the crane is capable of moving MULTIPLE crates at once.
    // Iterating over the instructions:
    let mut crates_layout_part_2 = crates_layout.clone();
    for element in &crates_instructions {
        //println!("Applying movements: {} from {} to {}", element.0, element.1, element.2);
        apply_movements_together(
            element.0, //amount
            TryInto::<usize>::try_into(element.1 - 1).unwrap(), // from 
            TryInto::<usize>::try_into(element.2 - 1).unwrap(), // to
            &mut crates_layout_part_2
        );
    }

    // Extracting the last element from each stack:
    let mut part_2_result : String = "".to_owned();
    for stack_idx in 0..stacks_number {
        part_2_result.push(crates_layout_part_2[stack_idx].last().unwrap().to_owned());
    }
    println!("\nPart 2 Result is {}.", part_2_result);

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        assert_eq!(parse_instruction("move 3 from 2 to 5").unwrap(), (3,2,5));
    }

    // I should have added more tests, but setting up the "layout" of the crates 
    // would have been too time-consuming.
}