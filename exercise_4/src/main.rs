// Exercise 4: get the common element between two halves of a string.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;


// Parsing the syntax: AA-BB,CC-DD
fn parse_elf_assignments(input : &str) -> Option<((u32, u32), (u32, u32))> {
    let regex_string = 
        r"(?P<val1>\d+)-(?P<val2>\d+),(?P<val3>\d+)-(?P<val4>\d+)";
    let regex = Regex::new(regex_string).unwrap();
    match regex.captures(input) {
        Some(caps) => {
            let internal_parse = |key| {
                caps.name(key).unwrap().as_str().parse::<u32>().unwrap()
            };
            let val1 = internal_parse("val1");
            let val2 = internal_parse("val2");
            let val3 = internal_parse("val3");
            let val4 = internal_parse("val4");
            Some(((val1, val2), (val3, val4)))
        }
        None => None,
    }
}


// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 4!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // Reading in two vectors, then using the "zip" functionality to work along them
    let mut elf_pairs_assignments = Vec::<((u32, u32), (u32, u32))>::new();
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            elf_pairs_assignments.push(parse_elf_assignments(&line.to_owned()).unwrap());
        }
    }

    println!("There are {} elements in the assignments.", elf_pairs_assignments.len());

    // Sanity check: the first index should always be left from the second.
    for elem in &elf_pairs_assignments {
        if elem.0.0 > elem.0.1 || elem.1.0 > elem.1.1 {
            panic!("Indeces not ordered: {}-{},{}-{}", elem.0.0, elem.0.1, elem.1.0, elem.1.1);
        }
    }

    // Part 1 is extremely simple if the interval A is contained in B or vice versa, it's a +1 on the counter:
    let mut counter = 0;
    for elem in &elf_pairs_assignments {
        // If elf 1 contains elf 2 or elf 2 contains elf 1.
        if (elem.0.0 <= elem.1.0 && elem.0.1 >= elem.1.1) ||  (elem.1.0 <= elem.0.0 && elem.1.1 >= elem.0.1) {
            counter += 1;
        }
    }

    println!("number of entirely overlapping sets is {}.", counter);

    // Turns out Part 2 is just as simple: counting if there is any overlap at all.
    let mut counter = 0;
    for elem in &elf_pairs_assignments {
        // checking if the two sets are entirely non-intersecting. Either A is left from B or B is left from A.
        if (elem.0.0 <= elem.1.0 && elem.1.0 <= elem.0.1) || (elem.1.0 <= elem.0.0 && elem.0.0 <= elem.1.1) {
            counter += 1;
        }
    }

    println!("number of partially overlapping sets is {}.", counter);

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_assignment() {
        assert_eq!(parse_elf_assignments("42-44,2-333").unwrap(), ((42,44),(2,333)));
    }
}