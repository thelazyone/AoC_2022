// Exercise 14: calculating the falling of sand in a rock structure

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
enum Materials {
    Air,
    Rock,
    Sand,
}

// Defining the 2D space as a dense matrix
struct SandBox {
    start : (usize, usize),
    size : (usize, usize),
    data : Vec<Materials>,
}

// States of the grain of sand after a movement.
enum SandMovement{
    NewPosition((usize, usize)),
    Stuck,
    Gone
}

impl SandBox {

    fn new(start : (usize, usize), size : (usize, usize)) -> SandBox{
        SandBox {
            start : start,
            size : size,
            data : vec![Materials::Air; size.0 * size.1],
        }
    }


    fn get_coords_from_index(&self, index : &usize) -> (usize, usize) {
        (index % self.size.0 + self.start.0, index / self.size.0 + self.start.1)
    }


    fn get_index_from_coords(&self, coords : (usize, usize)) -> usize {

        // Sanity check:
        if coords.0 < self.start.0 || coords.1 < self.start.1 {
            panic!("Coordinates before the start of the map! {:?} {:?}",coords, self.start);
        }
        if coords.0 > self.start.0 + self.size.0 || coords.1 > self.start.1 + self.size.1 {
            panic!("Coordinates after the end of the map! {:?} {:?} {:?}",coords, self.start, self.size);
        }

        coords.0.checked_sub(self.start.0).unwrap() + coords.1.checked_sub(self.start.1).unwrap() * self.size.0
    }


    fn get_value(&self, coords : (usize, usize)) -> Materials {
        self.data[self.get_index_from_coords(coords)]
    }


    // Sets a sand grain but does not perform any gravity simulation.
    fn add_sand_in_coords(&mut self, coords : (usize, usize)) {
        let target_index = self.get_index_from_coords(coords);
        self.data[target_index] = Materials::Sand;
    }


    // Draws a line of rock in the map.
    // Only for vertical or Horizontal rock segments.
    fn add_rock_segment(&mut self, start : (usize, usize), end : (usize, usize)) {
        let difference = (end.0 as i32 - start.0 as i32, end.1 as i32 - start.1 as i32);
        let direction = (
            difference.0.checked_div(difference.0.abs()).unwrap_or_else(|| 0),
            difference.1.checked_div(difference.1.abs()).unwrap_or_else(|| 0));

        for index in 0..cmp::max(difference.0.abs(), difference.1.abs()) + 1 {
            let new_coords = (
                (start.0 as i32 + (index * direction.0)) as usize,
                (start.1 as i32 + (index * direction.1)) as usize);
            let data_index = self.get_index_from_coords(new_coords);
            self.data[data_index] = Materials::Rock;
        }
    }

    
    // Drops a grain from a specific position and iterates until
    // - the grain either reachesa static place (Stuck) OR
    // - it falls to the bottom (Gone) OR 
    // - it cannot be spawned at all because the drop position is occupied (Gone)
    fn drop_sand_grain(&mut self, add_position : (usize, usize)) -> (usize, Option<(usize, usize)>) {

        // Looping until the grain has stopped moving or has reached the bottom.
        let mut sand_cursor = add_position;
        for counter in 0..self.size.1 + 1 {
            match self.get_sand_direction(sand_cursor) {
                SandMovement::NewPosition(new_position) => sand_cursor = new_position,
                SandMovement::Stuck => return (counter , Some(sand_cursor)),
                SandMovement::Gone => return (counter, None),
            }
        }

        // The operation should never reach this point.
        panic!("Reached iterations limit for sand grain.")
    }


    // Keeps adding sand to the sandbox, until the first grain is Gone instead
    // of Stuck. At that point it returns the number of sand grains.
    fn add_all_sand(&mut self, add_position : (usize, usize)) -> usize{

        // Looping until found.
        let mut sand_counter = 0;
        loop {
            match self.drop_sand_grain(add_position).1 {
                Some(sand_position) => {
                    self.add_sand_in_coords(sand_position);
                    sand_counter += 1;},
                None => return sand_counter,
            }
        }
    }


    // Checking in the sandbox what's below, provides the next positoin for the grain.
    fn get_sand_direction(&self, curr_position: (usize, usize)) -> SandMovement {

        let mut new_position : (usize, usize) = curr_position; 

        // Checking if it has reached the bottom of the map:
        if new_position.1 >= self.start.1 + self.size.1 - 1 {
            return SandMovement::Gone;
        }

        // Check the three objects below: first straight below, then bottom left, then bottom right.
        if self.get_value((curr_position.0, curr_position.1 + 1)) == Materials::Air {
            new_position = (curr_position.0, curr_position.1 + 1);
        }

        // Bottom left
        else if curr_position.0 > self.start.0 && curr_position.1 < self.start.1 + self.size.1 && 
            self.get_value((curr_position.0 - 1, curr_position.1 + 1)) == Materials::Air {
                new_position = (curr_position.0.checked_sub(1).unwrap(), curr_position.1 + 1);
        }

        // Bottom right
        else if curr_position.0 < self.start.0 + self.size.0 && curr_position.1 < self.start.1 + self.size.1 &&
            self.get_value((curr_position.0 + 1, curr_position.1 + 1)) == Materials::Air {
                new_position = (curr_position.0 + 1, curr_position.1 + 1);
        }

        // otherwise it's stuck.
        else {

            // Checking if already overlapping an existing sand:
            if self.get_value((curr_position.0, curr_position.1)) != Materials::Air {
                return SandMovement::Gone;
            }

            // Otherwise, it's legit stuck
            return SandMovement::Stuck;
        }

        // If arrived here all the check cases have been passed.
        SandMovement::NewPosition(new_position)
    }


    // Generates a string with the sandbox.
    fn draw_map (&self) -> String {
        let mut outString = "".to_string();
        for row_index in 0..self.size.1 {
            let mut data_slice = vec![Materials::Air; self.size.0];
            data_slice.copy_from_slice(&self.data[row_index * self.size.0..(row_index + 1) * self.size.0]);
            let new_string = data_slice.iter().map(|&val| {
                match &val {
                    &Materials::Air => '.',
                    &Materials::Rock => '#',
                    &Materials::Sand => 'o',
                }
            }).collect::<String>().clone(); 
            outString += &new_string;
            outString += "\n";
        }

        outString
    } 
}

// Primary Function
fn execute (input_path : String)  -> Option<(u32, u32)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let mut result_part_1 : u32 = 0;
    let mut result_part_2 : u32 = 0;

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

    // Each line is made of coordinates separated by " -> "
    // Finding the map dimensions:
    let mut rock_paths = Vec::<Vec<(usize, usize)>>::new();
    let mut min_dimensions = (usize::MAX, 0);
    let mut max_dimensions = (0, 0);
    for curr_line in lines_vec {
        rock_paths.push(Vec::<(usize, usize)>::new());
        for pair in curr_line.split(" -> ") {
            let dimensions = pair.split_once(",").unwrap().clone();
            let dimensions = (dimensions.0.parse::<usize>().unwrap(), dimensions.1.parse::<usize>().unwrap());
            min_dimensions.0 = cmp::min(min_dimensions.0, dimensions.0 - 1); 
            // min_dimensions.1 = cmp::min(min_dimensions.1, dimensions.1); // Unnecessary
            max_dimensions.0 = cmp::max(max_dimensions.0, dimensions.0); 
            max_dimensions.1 = cmp::max(max_dimensions.1, dimensions.1); 
            rock_paths.last_mut().unwrap().push(dimensions);
        }
        assert!(!rock_paths.is_empty());
    }

    // Creating the map and filling it.
    let cave_size = (
        max_dimensions.0.checked_sub(min_dimensions.0).unwrap() + 1,
        max_dimensions.1.checked_sub(min_dimensions.1).unwrap() + 1);
    println!("For Part 1: Creating cave of size {:?}", cave_size);
    let mut cave_map = SandBox::new(
        min_dimensions,
        cave_size);  
    for line_points in rock_paths.clone() {
        for segment_idx in 1..line_points.len() {
            cave_map.add_rock_segment(line_points[segment_idx - 1], line_points[segment_idx]);
        }
    }

    // Pouring all the sand from 500, 0, as required
    let pouring_point = (500, 0);
    let grains_number = cave_map.add_all_sand(pouring_point);
    result_part_1 = grains_number as u32;
    
    // Debug only, for the test sized input or for a good laugh.
    //println!("Testing map:\n{}",cave_map.draw_map());

    // For Part 2 the map becomes a lot wider! A sparse matrix would maybe have been
    // more convenient here! Let's continue like this. 
    // We have to add a bottom to the map. It doesn't have to be infinitely wide, 
    // just twice as wide as it is tall.
    max_dimensions.1 = max_dimensions.1 + 2;
    max_dimensions.0 = pouring_point.0 + cave_size.0 + max_dimensions.1;
    min_dimensions.0 = pouring_point.0 - cave_size.0 - max_dimensions.1;
    let cave_size = (
        max_dimensions.0.checked_sub(min_dimensions.0).unwrap() + 1,
        max_dimensions.1.checked_sub(min_dimensions.1).unwrap() + 1);
    println!("For Part 1: Creating cave of size {:?}", cave_size);
    let mut cave_map = SandBox::new(
        min_dimensions,
        cave_size);  
    for line_points in rock_paths.clone() {
        for segment_idx in 1..line_points.len() {
            cave_map.add_rock_segment(line_points[segment_idx - 1], line_points[segment_idx]);
        }
    }

    // Adding a bottom segment:
    cave_map.add_rock_segment((min_dimensions.0, max_dimensions.1), (max_dimensions.0, max_dimensions.1));

    // Filling with sand again.
    let grains_number = cave_map.add_all_sand(pouring_point);
    result_part_2 = grains_number as u32;

    // Debug only, for the test sized input or for a good laugh.
    //println!("Testing map:\n{}",cave_map.draw_map());

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 14!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 24);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 93);
    }    
}