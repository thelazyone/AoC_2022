// Exercise 18: Working with voxels volumes and areas.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

// Defining a simple 3D point, don't want to use complex stuff for this.
type Point = [i32; 3];

#[derive(Debug)]
struct VoxelSet {
    voxels : HashSet<Point>,
}
impl VoxelSet {
    fn new () -> VoxelSet {
        VoxelSet{ voxels : HashSet::<Point>::new() }
    }


    fn add_voxel(&mut self, pos : &Point) {
        self.voxels.insert(pos.clone());
    }


    fn calculate_surface(&self) -> usize {
        
        // This could be written in a more idiomatic way.
        let mut total_surface : usize = 0;
        for elem in &self.voxels {
            total_surface += self.check_adjacent(elem);
        }
        total_surface
    }


    fn check_adjacent(&self, pos : &Point) -> usize {
        
        let adj_coords = VoxelSet::get_adjacent_coords(pos);
        let mut adj_counter : usize = 0;
        for elem in adj_coords {
            if !self.voxels.contains(&elem) {
                adj_counter += 1;
            }
        }
        adj_counter
    } 

    
    fn get_adjacent_coords(pos : &Point) -> Vec<Point> {
        
        // Ugly approach, but that's how it is with tuples.
        let mut all_coords = Vec::<Point>::new();
        let mut temp_point = pos.clone();
        for dim_idx in 0..pos.len() {

            // Increasing by 1, then going to -1 then back to +0
            temp_point[dim_idx] += 1;
            all_coords.push(temp_point.clone());
            temp_point[dim_idx] -= 2;
            all_coords.push(temp_point.clone());
            temp_point[dim_idx] += 1;
        }
        all_coords
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

    // Converting lines in coordinates 
    let mut lava_space : VoxelSet = VoxelSet::new();
    for line in lines_vec {
        lava_space.add_voxel(&line.split(",")
        .map(|dim| {dim.parse::<i32>().unwrap()})
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap_or_else(|v: Vec<i32>| panic!("Expected a Vec of length {} but it was {}", 3, v.len())));
    }

    result_part_1 = lava_space.calculate_surface() as u32;

    // part 2 requires to find air pockets within the lava and remove them from the surface calculation.
    


    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 18!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 64);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 58);
    }    
}