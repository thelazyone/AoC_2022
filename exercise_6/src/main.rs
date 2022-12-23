// Exercise 6: check if four characters in a row are unique

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn check_no_duplicates_in_slice(input_slice : &[u8]) -> bool {
    for i in 0..input_slice.len()-1 {
        // Check if the rest of the slice contains any element that is identical
        if input_slice[i+1..].contains(&input_slice[i]) {
            return false;
        }
    }
    true
}


// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 6!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // First reading the input string - easy.
    let mut input_line : String = "".to_string();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            input_line = line;
        }
    }
    println!("read {} characters from input", input_line.len());

    // For Part 1, i must check every group of four elements. 
    // Options are 
    // Two iterators for start-end (verbose but fast)
    // a rotating buffer (implies a copy, but no allocation)
    // using a deque (both copy and allocations)
    // I'm gonna use the first, with the slices.
    let input_line = input_line.as_bytes();
    let window_size = 4;
    let mut moving_window = input_line.windows(window_size);

    // Iterating as long as the "next" works well
    // Note that this might give weird results at the *end* of the string, 
    // I don't know the exact behaviour of windows at the end of the vector.
    let mut iteration_counter = 0;
    while let Some(element) = moving_window.next() {
        if check_no_duplicates_in_slice(element) {
            println!("found 4 unique elements at iteration {}, result for Part 1 is {}", iteration_counter, iteration_counter + window_size);
            break;
        }
        iteration_counter += 1;
    }

    // Part 2 - Same but with 14 elements.
    let window_size = 14;
    let mut moving_window = input_line.windows(window_size);
    let mut iteration_counter = 0;
    while let Some(element) = moving_window.next() {
        if check_no_duplicates_in_slice(element) {
            println!("found 14 unique elements at iteration {}, result for Part 2 is {}", iteration_counter, iteration_counter + window_size);
            break;
        }
        iteration_counter += 1;
    }

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_duplicates() {
        assert_eq!(check_no_duplicates_in_slice("awrrhjyj".as_bytes()), false);
        assert_eq!(check_no_duplicates_in_slice("a".as_bytes()), true);
        assert_eq!(check_no_duplicates_in_slice("abcdefghi".as_bytes()), true);
        assert_eq!(check_no_duplicates_in_slice("aAbBcCdD".as_bytes()), true);
    }
}