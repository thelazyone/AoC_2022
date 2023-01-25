// Exercise 16: find the path that maximizes the flux of water over time

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::collections::HashMap;
use regex::Regex;
use std::time::Instant;

// Implementing Dijkstra's algoritm (Similar to Day 12)
// TODO move it in a "utilities" with generic type.
#[derive(Debug)]
struct DijkstraGraphNode {
    current_distance : u32,
    previous_node : String,
}
impl DijkstraGraphNode {
    fn new () -> DijkstraGraphNode {
        DijkstraGraphNode {
            current_distance : u32::MAX,
            previous_node : "".to_string(),
        }
    }
}

fn run_dijkstra (graphs_map : &HashMap<String, Valve>, starting_point : String, target_point : String,) -> Option<u32> {

    // Creating a vector of the graph nodes. No need to look for the connections here.
    let mut unused_nodes : HashMap<String, DijkstraGraphNode> = graphs_map.keys().map(|x| (x.clone(), DijkstraGraphNode::new())).collect(); 
    let mut used_nodes = Vec::<String>::new();

    // Setting the distance of the starting node as Zero.
    unused_nodes.entry(starting_point).and_modify(|value| value.current_distance = 0);

    // Iterating the Dijkstra steps
    while !unused_nodes.is_empty() {

        // First finding the smaller value in the map.
        let smallest_key : String = unused_nodes
        .iter()
        .max_by(|a, b| b.1.current_distance.cmp(&a.1.current_distance))
            .map(|(k, _v)| k).unwrap().clone();
        let smallest_distance = unused_nodes.get(&smallest_key).unwrap().current_distance;

        // Sanity check: if the smallest distance is MAX it means that there are no paths to go through.
        if smallest_distance == u32::MAX {

            // Nothing to do, this distance is not good
            return None;
        }
        
        // If the current position is the target, ending the loop.
        if smallest_key == target_point {
            return Some(unused_nodes.get(&smallest_key).unwrap().current_distance);
        }

        // Iterating on all the available paths from the vector: 
        let current_neighbours = graphs_map.get(&smallest_key).unwrap().connected.clone();
        for (neighbour_idx, neighbour_key) in current_neighbours.iter().enumerate() {

            // If already processed, skip
            if used_nodes.contains(&neighbour_key) {
                continue;
            }

            // Otherwise, adding the distance to the already present one.
            let neighbour_distance = unused_nodes.get(neighbour_key).unwrap().current_distance;
            let new_distance = smallest_distance + graphs_map.get(&smallest_key).unwrap().connected_distance[neighbour_idx];
            if new_distance < neighbour_distance {

                // Updating the distance and the "last node before" index
                unused_nodes.entry(neighbour_key.clone()).and_modify(
                    |value| value.current_distance = new_distance.clone());
                unused_nodes.entry(neighbour_key.clone()).and_modify(
                    |value| value.previous_node = smallest_key.clone());
            }
        }

        // Once done, removing the element from the map and adding the index to the "used" ones.
        unused_nodes.remove(&smallest_key);
        used_nodes.push(smallest_key);
    }

    panic!("Something went wrong, no path found!");
}


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
fn simplify_valves_map (valves_map : &HashMap<String, Valve>, starting_valve_name : &String) -> HashMap<String, Valve> {
    let mut new_map = HashMap::<String, Valve>::new();

    // For each element of the map, checking all working valves:
    for (valve_name, valve) in valves_map {

        // Ignoring all the valves that are not splits or working valves
        if valve_name != starting_valve_name && !is_working_valve_or_split(&valve) {
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


fn calculate_all_distances (valves_map : &HashMap<String, Valve>) -> HashMap<String, HashMap<String, u32>> {
    // For each valve calculating the distance of all the other valves.
    let mut all_valves_distances = HashMap::<String, HashMap<String, u32>>::new();
    let valves_names = valves_map.keys().cloned().collect::<Vec<String>>();
    for (valve_str, _) in valves_map {
        let mut valve_distances =  HashMap::<String, u32>::new();
        for other_valve_str in &valves_names {
            valve_distances.insert(
                other_valve_str.clone(),
                run_dijkstra(valves_map, valve_str.clone(), other_valve_str.clone()).unwrap());
        }
        all_valves_distances.insert(valve_str.clone(), valve_distances);
    }

    all_valves_distances
}


// Recursively searches through the rooms, returning the output through the various
// paths.
// Note that the output is returned directly as the flux times the remaining time.
fn find_path_maximum_steam (
    valves_map : &HashMap<String, Valve>, 
    distances_map : &HashMap<String, HashMap<String, u32>>, 
    path : Vec<String>,
    starting_valve_str : String,
    starting_flux : u32,
    total_steam : u32,
    current_iteration : u32,
    max_iterations : u32) -> (Vec<String>, u32) {

    let current_valve = valves_map.get(&starting_valve_str).unwrap();
    let mut new_path = path.clone();

    // Takes one turn to activate the valve, but only if it's a new one.
    new_path.push(starting_valve_str.clone());
    let new_iteration = current_iteration + 1;
    let new_steam = total_steam + starting_flux; // one iteration at current flux
    let new_flux = starting_flux + current_valve.flux;

    // moving to all the other valves, as long as there's enough remaining iterations
    let mut max_steam = new_steam;
    let mut max_path = new_path.clone();
    for (other_valve_str, other_valve_dist) in distances_map.get(&starting_valve_str).unwrap() {

        // If the target is too far in the path just return the final flux
        // Since it takes one step to activate the valve, using a -2 in the check
        // Same applies if the target to reach has been done already: this tests what happens
        // if the actor doesn't move until the end.
        if new_path.contains(other_valve_str) || new_iteration + other_valve_dist > max_iterations - 1
        {
            let final_steam = new_steam + new_flux * (max_iterations - new_iteration);
            if final_steam > max_steam {
                max_steam = final_steam;
            }
        }
        else {
            let (found_path, found_steam) = find_path_maximum_steam (
            valves_map, 
            distances_map,
            new_path.clone(),
            other_valve_str.clone(),
            new_flux,
            new_steam + new_flux * other_valve_dist,
            new_iteration + other_valve_dist,
            max_iterations);

            if found_steam > max_steam {
                max_steam = found_steam;
                max_path = found_path;
            }
        }
    }

    // At this point all the values obtained are after N iterations, so I just
    // search the maximum of them all.
    (max_path, max_steam)
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
        valves_map.insert(pair.0, pair.1);
    }

    // Creating a data structure that ignores the rooms with flux == 0,
    // which are de facto not valves.
    let valves_map = simplify_valves_map(&valves_map, &"AA".to_string());
    println!("there are {} active valves:", valves_map.len());
    for line in &valves_map {
        println!("room is: {:?}", line);
    }

    // Calculating all distances once: 
    let distances_map = calculate_all_distances(&valves_map);
    
    // Iterating on ALL permutations. It's not THAT many. 
    let now = Instant::now();
    let max_iterations = 30;
    let (path_taken, max_steam) = find_path_maximum_steam(
        &valves_map, 
        &distances_map, 
        Vec::<String>::new(),
        "AA".to_string(), 
        0, 
        0, 
        0, 
        max_iterations + 1 /* For the valve to open */);
    println!("Path taken is {:?} for a total of {} steam.", path_taken, max_steam);
    result_part_1 = max_steam;  
    println!("Part A took {} ms", now.elapsed().as_millis());

    // For two actors, using a dumb but very feasible approach: iterating on all the possible pairs
    // of subsets of the valves. Each time we got to re-calculate the distances, run the find function
    // and look for the faster.
    let mut max_steam_two_actors = 0;
    let max_iterations = 26;
    let now = Instant::now();
    for subset_idx in 0..i32::pow(2, (valves_map.len() - 1) as u32) {

        if subset_idx % 100 == 0 {
            println!("iteration {} of {}", subset_idx, i32::pow(2, (valves_map.len() - 1) as u32));
        }

        // Setting a path of "previously explored" paths to be avoided, so that
        // the algo won't have to go through them.
        let mut path_a = Vec::<String>::new();
        let mut path_b = Vec::<String>::new();
        for (elem_index, elem) in valves_map.iter().enumerate() {
            if subset_idx / i32::pow(2, elem_index as u32) % 2 == 0{
                path_a.push(elem.0.clone());
            }
            else {
                path_b.push(elem.0.clone());
            }
        }

        let (_, max_steam_a) = find_path_maximum_steam(
            &valves_map, 
            &distances_map, 
            path_a,
            "AA".to_string(), 
            0, 
            0, 
            0, 
            max_iterations + 1 /* For the valve to open */);

        let (_, max_steam_b) = find_path_maximum_steam(
            &valves_map, 
            &distances_map, 
            path_b,
            "AA".to_string(), 
            0, 
            0, 
            0, 
            max_iterations + 1 /* For the valve to open */);

        if max_steam_two_actors < max_steam_a + max_steam_b {
           max_steam_two_actors = max_steam_a + max_steam_b;
        }
    }
    println!("Part B took {} ms", now.elapsed().as_millis());

    result_part_2 = max_steam_two_actors;
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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 1707);
    }    
}