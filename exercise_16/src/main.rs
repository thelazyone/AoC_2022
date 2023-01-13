// Exercise 16: find the path that maximizes the flux of water over time

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;
use regex::Regex;


// Simple graph data structure. Since it's a small graph, we can afford 
// using the name of the nodes as reference rather than pointers.
#[derive(Debug)]
struct Valve {
    name : String,
    flux : i32,
    connected : Vec<String>,
}

impl Valve {

    // Valves are created from text input in the format:
    // Valve XX has flow rate=YY; tunnels lead to valves ZZ1, ZZ2, ZZ3
    fn new_from_line(input_string : &String) -> Option<Valve> {

        let regex_string = 
        //r"Valve\s(?P<val1>\-*\d+)\D+=(?P<val2>\-*\d+);\stunnels\slead\sto\svalves?\s(?P<val3>\-*\d+)";
        r"Valve\s(?P<val1>\S+)\D+=(?P<val2>\-*\d+)(.\stunnel)(s?)(\slead)(s?)(\sto\svalve)(s?)(\s)(?P<val3>\D+)";
        let regex = Regex::new(regex_string).unwrap();
        match regex.captures(input_string) {
            Some(caps) => {
                let val1 = caps.name("val1").unwrap().as_str().to_owned();
                let val2 = caps.name("val2").unwrap().as_str().parse::<i32>().unwrap();
                let val3 = 
                    caps.name("val3").unwrap().as_str().to_owned().split(", ") // i got vec<&str>
                    .map(|substr| substr.to_owned()).collect(); // converting to vec<String>
                Some(Valve{name: val1, flux: val2, connected: val3})
            }
            None => None,
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

    // Filling data for each valve:
    let mut valves_vec = Vec::<Valve>::new();
    for line in lines_vec {
        valves_vec.push(Valve::new_from_line(&line).unwrap());
        println!("Creating: {:?}", valves_vec.last().unwrap());
    }


    result_part_1 = 0;
    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 16!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 1651);
    }    

    #[test]
    fn global_test_part_2() {
        //assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}