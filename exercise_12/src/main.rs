// Exercise 12: calculating the path of a cursor following the movement of another

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// // utility
use std::collections::HashMap;

// 2D graph-like dense matrix structure
struct WorldMap {
    world_dimensions : (usize, usize),
    elevations_matrix : Vec<u32>,
}

impl WorldMap {

    fn get_point(&self, coord : (usize, usize)) -> u32 {
        self.elevations_matrix[self.get_index_from_coords(coord)]
    }

    fn get_coords_from_index(&self, index : &usize) -> (usize, usize) {
        (index % self.world_dimensions.0, index / self.world_dimensions.0)
    }

    fn get_index_from_coords(&self, coords : (usize, usize)) -> usize {
        coords.0 + coords.1 * self.world_dimensions.0
    }

    pub fn is_passable(&self, current_position : (usize, usize), new_position : (usize, usize)) -> bool {
        if self.get_point(new_position) <= self.get_point(current_position) + 1 {
            return true;
        }
        return false;
    }

    pub fn get_neighbours(&self, current_index : &usize) -> Vec<usize> {
        
        // For each direction comparing elevations.
        let mut resulting_elevations = Vec::<usize>::new();
        let current_position = self.get_coords_from_index(current_index);

        if (current_position.1 < self.world_dimensions.1 - 1) && self.is_passable(current_position, (current_position.0, current_position.1 + 1)) {
            resulting_elevations.push(self.get_index_from_coords((current_position.0, current_position.1 + 1)));
        }

        if current_position.1 > 0 && self.is_passable(current_position, (current_position.0, current_position.1 - 1)) {
            resulting_elevations.push(self.get_index_from_coords((current_position.0, current_position.1 - 1)));
        }

        if (current_position.0 < self.world_dimensions.0 - 1) && self.is_passable(current_position, (current_position.0 + 1, current_position.1)) {
            resulting_elevations.push(self.get_index_from_coords((current_position.0 + 1, current_position.1)));
        }

        if current_position.0 > 0 && self.is_passable(current_position, (current_position.0 - 1, current_position.1)) {
            resulting_elevations.push(self.get_index_from_coords((current_position.0 - 1, current_position.1)));
        }

        resulting_elevations
    }
}

// Implementing Dijkstra's algoritm (with weight zero)
struct DijkstraGraphNode {
    current_distance : u32,
    previous_node : u32,
}

impl DijkstraGraphNode {
    fn new () -> DijkstraGraphNode {
        DijkstraGraphNode {
            current_distance : u32::MAX,
            previous_node : 0,
        }
    }
}

fn run_dijkstra (world_map : WorldMap, starting_point : u32, target_point : u32,) -> Option<u32> {

    // Rough sanity check:
    if world_map.elevations_matrix.is_empty() {
        return None;
    }

    // Creating a vector of the graph nodes. No need to look for the connections here.
    let mut unused_nodes : HashMap<u32, DijkstraGraphNode> = 
        (0..world_map.elevations_matrix.len() as u32).map(|x| (x.clone(), DijkstraGraphNode::new())).collect(); 
    let mut used_nodes = Vec::<u32>::new();

    // Setting the distance of the starting node as Zero.
    //unused_nodes.set(&starting_point, unused_nodes.get(&starting_point).unwrap().current_distance = 0;
    unused_nodes.entry(starting_point).and_modify(|value| value.current_distance = 0);

    // Iterating the Dijkstra steps
    while !unused_nodes.is_empty() {

        // First finding the smaller value in the map.
        let smallest_key : u32 = unused_nodes.iter().max_by(|a, b| b.1.current_distance.cmp(&a.1.current_distance))
            .map(|(k, _v)| k).unwrap().clone();
        let smallest_distance = unused_nodes.get(&smallest_key).unwrap().current_distance;

        // Sanity check: if the smallest distance is MAX it means that there are no paths to go through.
        if smallest_distance == u32::MAX {

            // Nothing to do, this distance is not good
            return None;
        }
        
        // If the current position is the target, ending the loop.
        if smallest_key == target_point as u32 {
            return Some(unused_nodes.get(&smallest_key).unwrap().current_distance);
        }

        // Iterating on all the available paths from the vector: 
        let current_neighbours = world_map.get_neighbours(&(smallest_key as usize));
        for neighbour_idx in current_neighbours {

            // If already processed, skip
            if used_nodes.contains(&(neighbour_idx as u32)) {
                continue;
            }

            // Otherwise, adding the distance (always 1 for this case) to the already present one.
            let neighbour_distance = unused_nodes.get(&(neighbour_idx as u32)).unwrap().current_distance;
            let new_distance = 1 + smallest_distance;
            if new_distance < neighbour_distance {

                // Updating the distance and the "last node before" index
                unused_nodes.entry(neighbour_idx as u32).and_modify(
                    |value| value.current_distance = new_distance);
                unused_nodes.entry(neighbour_idx as u32).and_modify(
                    |value| value.previous_node = smallest_key);
            }
        }

        // Once done, removing the element from the map and adding the index to the "used" ones.
        unused_nodes.remove(&smallest_key);
        used_nodes.push(smallest_key);
    }

    panic!("Something went wrong, no path found!");
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
    println!("Read {} lines from input", lines_vec.len());
    assert!(lines_vec.len() > 1);

    // Converting lines into a map of elevations: 
    let col_number = lines_vec[0].len();
    let lines_number = lines_vec.len();
    let mut elevations_vector = Vec::<u32>::new();
    let mut starting_point = 0;
    let mut target_point = 0;
    for line in lines_vec {
        for chararacter in line.chars() {
            if chararacter == 'S' {
                starting_point = elevations_vector.len();
                elevations_vector.push(0 /* elevation as a */);
            }
            else if chararacter == 'E' {
                target_point = elevations_vector.len();
                elevations_vector.push(25 /* elevation as z */);
            }
            else {
                elevations_vector.push((chararacter as i32 - 97) as u32);
            }
        }
    }

    // Creating the World map and pass it to Dijkstra to be consumed:
    println!("Part 1: Calculating the path from index {} to index {}...", starting_point, target_point);
    result_part_1 = 
    run_dijkstra(WorldMap {
        world_dimensions : (col_number, lines_number),
        elevations_matrix : elevations_vector.clone()} ,
        starting_point as u32,
        target_point as u32,
    ).unwrap();

    // For Part 2 I'll run the same logic from ALL points that have an 'a' (elevation zero)
    // and find the shortest.
    println!("Part 2: Calculating multiple paths and finding the smallest. This may take a while...");
    let mut shortest_route = u32::MAX;
    for (index, map_pixel) in elevations_vector.clone().iter().enumerate() {

        // Skipping everything that is not an 'a'
        if map_pixel != &0 {
            continue;
        }

        // Running Dijkstra on the current position.
        let pixel_distance = run_dijkstra(WorldMap {
            world_dimensions : (col_number, lines_number),
            elevations_matrix : elevations_vector.clone()} ,
            index as u32,
            target_point as u32,
        );

        match pixel_distance {
            Some(value) => {
                if value < shortest_route {
                    shortest_route = value;
                }
            },
            None => continue,
        };
    }
    result_part_2 = shortest_route;

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 12!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 31);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 29);
    }    
}