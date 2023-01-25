// Exercise 16: find the path that maximizes the flux of water over time

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::collections::HashMap;
use regex::Regex;


// Simple graph data structure. Since it's a small graph, we can afford 
// using the name of the nodes as reference rather than pointers.
#[derive(Debug)]
#[derive(Clone)]
struct Valve {
    flux : u32,
    connected : Vec<String>,
    connected_distance : Vec<u32>,
}
impl Valve {

    // Valves are created from text input in the format:
    // Valve XX has flow rate=YY; tunnels lead to valves ZZ1, ZZ2, ZZ3
    fn new_from_line(input_string : &String) -> Option<(String, Valve)> {

        let regex_string = 
        r"Valve\s(?P<val1>\S+)\D+=(?P<val2>\-*\d+)(.\stunnel)(s?)(\slead)(s?)(\sto\svalve)(s?)(\s)(?P<val3>\D+)";
        let regex = Regex::new(regex_string).unwrap();
        match regex.captures(input_string) {
            Some(caps) => {
                let val1 = caps.name("val1").unwrap().as_str().to_owned();
                let val2 = caps.name("val2").unwrap().as_str().parse::<u32>().unwrap();
                let val3: Vec<String>= 
                    caps.name("val3").unwrap().as_str().to_owned().split(", ") // i got vec<&str>
                    .map(|substr| substr.to_owned()).collect(); // converting to vec<String>
                Some((val1, 
                    Valve{flux: val2, connected: val3.clone(), connected_distance: vec![1; val3.len()]}))
            }
            None => None,
        }
    }
}


// Checks if the current room is worth being mapped or it's just
// part of the path between two relevant valves (and/or splits)
fn is_working_valve_or_split(valve : &Valve) -> bool {
    valve.connected.len() != 2 || 
    valve.flux != 0
}


// Moves through empty "corridors" until finding the next room
fn get_next_working_valve_or_split(
    valves_map : &HashMap<String, Valve>,
    current_valve_str: &String,
    next_valve_str : &String) 
    -> Option<(String, u32)> {

        // Checking if the current valve has one of the following:
        // more than two rooms connected: it's a split
        // A flux != 0 : it's a working valve.
        let next_valve = valves_map.get(next_valve_str).unwrap();
        if is_working_valve_or_split(next_valve) {
            return Some((next_valve_str.to_owned(), 1));
        }
        else {
            // Retrieving the next valve name. One should be the previous
            let next_next_valve_str = next_valve.connected
            .iter()
            .enumerate()
            .filter(|elem| elem.1 != current_valve_str)
            .map(|elem| elem.1.clone())
            .collect::<Vec<_>>()
            .get(0).unwrap().to_owned();

            let next_next_valv_pair = 
                get_next_working_valve_or_split(
                    valves_map, 
                    next_valve_str, 
                    &next_next_valve_str)
                .unwrap();

            return Some((next_next_valv_pair.0, next_next_valv_pair.1 + 1));
        }
}


// Creates a new valves map, removing all unnecessary rooms.
// The new connections for each room will have distances that are >= 1
fn simplify_valves_map (valves_map : &HashMap<String, Valve>) -> HashMap<String, Valve> {
    let mut new_map = HashMap::<String, Valve>::new();

    // For each element of the map, checking all working valves:
    for (valve_name, valve) in valves_map {

        // Ignoring all the valves that are not splits or working valves
        if valve_name != "AA" && !is_working_valve_or_split(&valve) {
            continue;
        }

        // Otherwise, updating all connection with the proper length.
        let mut temp_valve = Valve{
            connected : Vec::<String>::new(),
            connected_distance : Vec::<u32>::new(),
            flux : valve.flux};
        for valve_connection in &valve.connected {
            let neighbour = get_next_working_valve_or_split(
                valves_map, 
                valve_name,
                valve_connection).unwrap();
            temp_valve.connected.push(neighbour.0);
            temp_valve.connected_distance.push(neighbour.1);
        }

        new_map.insert(valve_name.clone(), temp_valve);
    }

    new_map
}


// Recursively searches through the rooms, returning the output through the various
// paths.
// Note that the output is returned directly as the flux times the remaining time.
fn find_path_maximum_steam (
    valves_map : &HashMap<String, Valve>, 
    path : Vec<String>,
    starting_valve_str : String,
    starting_flux : u32,
    total_steam : u32,
    current_iteration : u32,
    max_iterations : u32) -> (Vec<String>, u32) {

    let current_valve = valves_map.get(&starting_valve_str).unwrap();

    // Takes one turn to activate the valve, but only if it's a new one.
    let new_iteration;
    let new_steam;
    let new_flux;
    // if !path.contains(&starting_valve_str){
    //     new_iteration = current_iteration + 1;
    //     new_steam = total_steam + starting_flux; // one iteration at current flux
    //     new_flux = starting_flux + current_valve.flux;
    // }
    // else
    {
        new_iteration = current_iteration;
        new_steam = total_steam;
        new_flux = starting_flux;
    }

    // Updating the path with the current 
    let mut new_path = path;
    //new_path.push(starting_valve_str.clone());

    // moving through all the connections, as long as there's enough remaining iterations
    let mut max_flux = 0;
    let mut max_path = Vec::<String>::new();
    for (connection_name, connection_distance) in
        current_valve.connected.iter().zip(&current_valve.connected_distance) {

        // If the target is too far in the path just return the final flux
        // Since it takes one step to activate the valve, using a -2 in the check
        if new_iteration + connection_distance > max_iterations - 2
        {
            let final_flux = new_steam + new_flux * (max_iterations - new_iteration - 1);
            if final_flux > 2000 {
            println!("Cell {}, iteration {}, distance {}, max {}, adding {}, final {}", 
            starting_valve_str, new_iteration, connection_distance, max_iterations,  
            new_flux * (max_iterations - new_iteration - 1),final_flux);}

            return (new_path.clone(), final_flux);
        }

        // Otherwise checking the next connections:
        let (found_path, found_flux) = find_path_maximum_steam (
        valves_map, 
        new_path.clone(),
        connection_name.clone(),
        new_flux,
        new_steam + new_flux * connection_distance,
        new_iteration + connection_distance,
        max_iterations);

        if found_flux > max_flux {
            max_flux = found_flux;
            max_path = found_path;
        }
    }

    // At this point all the values obtained are after 30 iterations, so I just
    // search the maximum of them all.
    (max_path, max_flux)
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
    let mut valves_map = HashMap::<String, Valve>::new();
    for line in lines_vec {
        let pair = Valve::new_from_line(&line).unwrap();
        println!("Creating: {:?}", pair);
        valves_map.insert(pair.0, pair.1);
    }

    // Creating a data structure that ignores the rooms with flux == 0,
    // which are de facto not valves.
    let valves_map = simplify_valves_map(&valves_map);
    println!("there are {} active valves", valves_map.len());

    // Iterating on ALL permutations. It's not THAT many. 
    let (path_taken, max_steam) = find_path_maximum_steam(
        &valves_map, 
        Vec::<String>::new(),
        "AA".to_string(), 
        0, 
        0, 
        0, 
        30);
    println!("Path taken is {:?} for a total of {} steam.", path_taken, max_steam);


    result_part_1 = max_steam;  
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