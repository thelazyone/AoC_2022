// Exercise 3: get the common element between two halves of a string.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// For hashSet
use std::collections::HashSet;

// Converting the character in a priority. 
// According to the AoC:
// - Lowercase item types a through z have priorities 1 through 26.
// - Uppercase item types A through Z have priorities 27 through 52.
fn get_character_score(input: &u8) -> u8 {
    // Assuming the character is either upper case or lower case letter,
    // I just need to shift from the ascii value.
    match input {
        65..=90 => return input - 65 + 26 + 1,
        97..=122 => return input - 97 + 1,
        _ => panic!("wrong input character!"),
    }
}


// Searching the common element between two slices of u8
fn search_common_between_char_slices(input1 : &[u8], input2 : &[u8]) -> u8 {
    for left_element in input1 {
        for right_element in input2 {
            if left_element == right_element {
                return right_element.to_owned();
            }
        }
    }
    0
}


// Searching the common element between two slices of u8
fn search_common_elements_between_char_slices(input1 : &[u8], input2 : &[u8]) -> Option<Vec::<u8>> {
    let mut common_elements = Vec::<u8>::new();
    for left_element in input1 {
        for right_element in input2 {
            if left_element == right_element {
                common_elements.push(right_element.to_owned());
                break;
            }
        }
    }

    // Removing duplicates, if any:
    let common_elements = common_elements
    .into_iter()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect::<Vec<_>>();

    Some(common_elements)
}



// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 3!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // Reading in two vectors, then using the "zip" functionality to work along them
    let mut inventory_vec = Vec::<String>::new();
    let mut inventory_split_vec = Vec::<(String, String)>::new();
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {

            // Reading the line length:
            let size = line.len();
            if size % 2 != 0 {
                panic!("wrong number of arguments")
            }
            let size = size/2;

            inventory_split_vec.push((line[..size].to_owned(), line[size..].to_owned()));
            inventory_vec.push(line.to_owned());
        }
    }
    
    // for each, searching the common element
    let mut total_sum : i32 = 0;
    for element in &inventory_split_vec {
        // Searching on both lines - complexity is n^2 but the characters are only 27 tops.
        total_sum += get_character_score(
            &search_common_between_char_slices(element.0.as_bytes(), element.1.as_bytes())) as i32;
    }

    // And returning the output.
    println!("Result of Part 1 is {}", total_sum);

    // For Part 2 I must find the common item between any truple of lines.
    // I am now implementing a brutal approach since (once again) the maximum number of searches goes with 27^3
    if inventory_vec.len() % 3 != 0 {
        panic!("Inventory size not multiple of 3!");
    }
    let mut total_sum : i32 = 0;
    for inventory_index in (0..inventory_vec.len()).step_by(3) {
        // Comparing 1 and 2:
        let common_1_2 = 
        search_common_elements_between_char_slices(
            inventory_vec[inventory_index].as_bytes(), 
            inventory_vec[inventory_index + 1].as_bytes()).unwrap();
        let common_1_2 = common_1_2.as_slice();
        //println!("1_2: Found {} common elements: {}", common_1_2.len(), String::from_utf8(common_1_2.to_vec()).unwrap());

        let common_1_2_3 =
        search_common_elements_between_char_slices(
            common_1_2,
            inventory_vec[inventory_index + 2].as_bytes()).unwrap();
        let common_1_2_3 = common_1_2_3.as_slice();
        //println!("X_3: Found {} common elements: {}", common_1_2_3.len(), String::from_utf8(common_1_2_3.to_vec()).unwrap());

        if common_1_2_3.len() != 1 {
            panic!("Not a single element has been found between the three!");
        }
        
        total_sum += get_character_score(&common_1_2_3[0]) as i32;
    }

    // Returning the common element sum:
    println!("For Part 2 the sum of priorities is {}.", total_sum);

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_scores() {
        assert_eq!(get_character_score(&('a' as u8)), 1);
        assert_eq!(get_character_score(&('z' as u8)), 26);
        assert_eq!(get_character_score(&('A' as u8)), 27);
        assert_eq!(get_character_score(&('Z' as u8)), 52);
    }

    #[test]
    fn test_find_common_characters() {
        assert_eq!('a' as u8, search_common_between_char_slices("abcd".as_bytes(), "TKfaLK".as_bytes()));
    }
}