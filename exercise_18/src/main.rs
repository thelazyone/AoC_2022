// Exercise 18: Working with voxels volumes and areas.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

// Defining a simple 3D point, don't want to use complex stuff for this.
const DIMENSIONS : usize= 3;
type Point = [i32; DIMENSIONS];


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


    fn find_cluster_around_lava(&self, pos : &Point, limits : &Vec::<(i32, i32)>) -> HashSet<Point> {

        let mut previous_visited = HashSet::<Point>::new();
        self.find_cluster_iterative(pos, &mut previous_visited, limits);
        previous_visited
    }


    fn find_cluster_iterative(
        &self, pos : &Point,
        previous_visited : &mut HashSet<Point>, 
        limits : &Vec::<(i32, i32)>) {
        
        // Check if already visited.
        if previous_visited.contains(pos) || self.voxels.contains(pos){
            return;
        }

        // Otherwise add to previous, and call function to all neighbours
        previous_visited.insert(pos.clone());
        for neighbour in VoxelSet::get_adjacent_coords(pos) {
            if neighbour[0] < limits[0].0 || neighbour[0] > limits[0].1 ||
                neighbour[1] < limits[1].0 || neighbour[1] > limits[1].1 ||
                neighbour[2] < limits[2].0 || neighbour[2] > limits[2].1 {
                    continue;
                }

            self.find_cluster_iterative(&neighbour, previous_visited, limits);
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
    
    // Sampling random points and clustering until reaching the edge. 
    // To that, finding the bounding box.
    let mut limits = Vec::<(i32, i32)>::new();
    for dim_idx in 0..DIMENSIONS {
        limits.push((
            lava_space.voxels.iter()
            .min_by(|a, b| {a[dim_idx].cmp(&b[dim_idx])}).unwrap()[dim_idx] - 1,
            lava_space.voxels.iter()
            .max_by(|a, b| {a[dim_idx].cmp(&b[dim_idx])}).unwrap()[dim_idx] + 1));
    }

    // Exploring the bounded space starting from the bottom-left point (which is outside)
    // Till this point dimensions are parametrical. For the iteration i set it fixed, but a better 
    // solution could be found.let zero_point: Point = vec!{limits[0].0, limits[1].0, limits[2].0}.try_into().unwrap();
    let zero_point: Point = vec!{limits[0].0, limits[1].0, limits[2].0}.try_into().unwrap();
    println!("Finding clusters, starting from {:?}.", zero_point);
    let outside_voxels = lava_space.find_cluster_around_lava(&zero_point, &limits).clone();
    println!("outside voxels are {}.", outside_voxels.len());

    // Finding all points: 
    let mut lava_space_filled : VoxelSet = VoxelSet::new();
    for x in limits[0].0..limits[0].1 + 1 {
        for y in limits[1].0..limits[1].1 + 1 {
            for z in limits[2].0..limits[2].1 + 1 {
                let temp_point: Point = vec!{x, y, z}.try_into().unwrap();
                if !outside_voxels.contains(&temp_point) {
                    lava_space_filled.add_voxel(&temp_point);
                }
            }
        }
    }
    println!("reciprocal voxels are {}.", lava_space_filled.voxels.len());
    result_part_2 = lava_space_filled.calculate_surface() as u32;

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