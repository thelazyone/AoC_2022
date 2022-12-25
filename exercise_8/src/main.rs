// Exercise 7: creating a filesystem representation and finding the large directories

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;

// Index from coordinates:
fn index_from_coords (i_coords : &(usize, usize), i_rows_number : &usize) -> Option<usize> {
    Some(i_coords.0 + i_coords.1 * i_rows_number)
}

// Check visibility along one direction:
fn is_visibile_along (
        i_data : &Vec::<u8>, 
        i_coords : &(usize, usize),
        i_matrix_size : &(usize, usize),
        i_movement : &dyn Fn((usize, usize)) -> Option<(usize, usize)>) -> bool {

    // Setting the current cursor and then moving it giving the function passed as input
    let tree_height = i_data[index_from_coords(&i_coords, &i_matrix_size.0).unwrap()];
    let mut current_position = i_coords.clone();
    for _ in 0..cmp::max(i_matrix_size.0, i_matrix_size.1) {

        // Move to new position, check if taller, return if it is.
        let new_position = i_movement(current_position);
        if new_position.is_none() {
            return true;
        }
        let new_position = new_position.unwrap();

        // If reached the border the visibility is OK
        if new_position.0 < 0 || new_position.0 >= i_matrix_size.0 || 
        new_position.1 < 0 || new_position.1 >= i_matrix_size.1 {
            return true;
        }

        // Checking if the element is shorter.
        if tree_height <= i_data[index_from_coords(&new_position, &i_matrix_size.0).unwrap()] {
            return false;
        }

        // Otherwise, updating the position and trying again.
        current_position = new_position;
    }
    true
}

// For part 2, calculate the view distance along directions
fn get_view_distance  (
    i_data : &Vec::<u8>, 
    i_coords : &(usize, usize),
    i_matrix_size : &(usize, usize),
    i_movement : &dyn Fn((usize, usize)) -> Option<(usize, usize)>) -> u32 {

    // As before, moving along the direction until either an ending has been reached or a tall tree.
    let tree_height = i_data[index_from_coords(&i_coords, &i_matrix_size.0).unwrap()];
    let mut current_position = i_coords.clone();
    let mut view_distance = 0;
    for _ in 0..cmp::max(i_matrix_size.0, i_matrix_size.1) {

        // Move to new position, check if taller, return if it is.
        let new_position = i_movement(current_position);
        if new_position.is_none() {
            return view_distance;
        }
        let new_position = new_position.unwrap();

        // If reached the border the visibility is OK
        if new_position.0 < 0 || new_position.0 >= i_matrix_size.0 || 
        new_position.1 < 0 || new_position.1 >= i_matrix_size.1 {
            return view_distance;
        }

        // Checking if the element is shorter.
        if tree_height <= i_data[index_from_coords(&new_position, &i_matrix_size.0).unwrap()] {
            return view_distance + 1; // The tree in view is part of it.
        }

        // Otherwise, updating the position and trying again.
        current_position = new_position;
        view_distance += 1;
    }
    view_distance
}

// Movements functions:
// Note that coordinates start from top-right.
fn go_up (i_pos : (usize, usize)) -> Option<(usize, usize)> {
    if i_pos.1 == 0 {
        return None
    }
    Some((i_pos.0, i_pos.1 - 1))
}
fn go_down (i_pos : (usize, usize)) -> Option<(usize, usize)> {
    Some((i_pos.0, i_pos.1 + 1))
}
fn go_left (i_pos : (usize, usize)) -> Option<(usize, usize)> {
    if i_pos.0 == 0 {
        return None
    }
    Some((i_pos.0 - 1, i_pos.1))
}
fn go_right (i_pos : (usize, usize)) -> Option<(usize, usize)> {
    Some((i_pos.0 + 1, i_pos.1))
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

    // Relatively brute-force approach (pushing each element rather than p).
    // Loading all values in a non-indicized space. 
    let rows_number = lines_vec.len();
    let cols_number = lines_vec[0].len();
    let mut all_trees_matrix = Vec::<u8>::with_capacity(rows_number*cols_number);
    for curr_line in lines_vec {
        for curr_char in curr_line.chars() {
            all_trees_matrix.push(curr_char as u8);
        }
    }

    // Checking tree height for each position from each direction.
    // Looks like the four directions require a bit of redundant code.
    let mut visible_trees_counter = 0;
    for col_idx in 0..cols_number {
        for row_idx in 0..rows_number {
            if is_visibile_along(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_right) {
                visible_trees_counter += 1;
            }
            else if is_visibile_along(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_left) {
                visible_trees_counter += 1;
            }
            else if is_visibile_along(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_up) {
                visible_trees_counter += 1;
            }
            else if is_visibile_along(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_down) {
                visible_trees_counter += 1;
            }
        }
    }
    result_part_1 = visible_trees_counter;

    // For part 2, checking all the view distances: the score is a product of all four.
    // The result is the higher "scenic score" among all trees.
    let mut scenic_scores = Vec::<u32>::new();
    for col_idx in 0..cols_number {
        for row_idx in 0..rows_number {
            let mut scenic_score = get_view_distance(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_right);
            scenic_score *= get_view_distance(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_left); 
            scenic_score *= get_view_distance(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_up); 
            scenic_score *= get_view_distance(&all_trees_matrix, &(col_idx, row_idx), &(cols_number, rows_number), &go_down);
            scenic_scores.push(scenic_score);
        }
    }
    result_part_2 = scenic_scores.iter().max().unwrap_or(&0).clone();

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 21);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}