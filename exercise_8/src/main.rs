// Exercise 7: creating a filesystem representation and finding the large directories

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


// Primary Function
fn execute (input_path : String)  -> Option<(u32, u32)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let mut result_part_1 = 0;
    let mut result_part_2 = 0;

    // First reading the input string - easy.
    let mut lines_vec = Vec::<String>::new();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            lines_vec.push(line);
        }
    }
    println!("read {} lines from input", lines_vec.len());

    // Returning both results.
    Some((result_part_1, result_part_2))
}


// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 8!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 21);
    }
}