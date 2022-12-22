// Exercise 1: parse a file containing empty-line-separated sets of values, adding them together and finding the highest. 

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 1!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // Cumulated vec (one element per each empty-line separator) and temp value
    let mut cumulated_values_vec = Vec::<i32>::new();
    let mut cumulated_value = 0;
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            if line.is_empty(){
                // Adding the output in the vector and resetting the support variable.
                cumulated_values_vec.push(cumulated_value);
                cumulated_value = 0;
            }
            else{
                cumulated_value += line.parse::<i32>().unwrap();
            }
        }
    }

    // Returning the result of PART 1.
    println!("Greatest value among {} is {}.", cumulated_values_vec.len(), cumulated_values_vec.iter().max().unwrap());

    // PART 2 - find the three greatest and sum them.

    // Sorting the vector: 
    cumulated_values_vec.sort();

    // Retrieving the last and the last three combined:
    println!("Three highest values combined are {}.", cumulated_values_vec.iter().rev().take(3).sum::<i32>());

    // End of main
    Ok(())
}
