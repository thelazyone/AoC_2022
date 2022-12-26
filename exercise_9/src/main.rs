// Exercise 9: calculating the path of a cursor following the movement of another

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

//Utility
use std::cmp;
use std::num;

// Handling the directions
enum Directions {
    U,
    D,
    L,
    R,
}


// Given the positions of head and tail, moving multiple times along segment with the head to follow.
fn move_segment(positions : &((i32, i32), (i32, i32)), direction: &Directions) -> ((i32, i32), (i32, i32)) {
    let mut out_positions = positions.clone();
    out_positions.0 = move_head(&out_positions.0, direction);
    out_positions.1 = follow_head(&out_positions);
    out_positions
}


// Basic movement, but wraps the enum match.
fn move_head(position: &(i32, i32), direction: &Directions) -> (i32, i32) {

    match direction {
        &Directions::U => return (position.0, position.1 + 1),
        &Directions::D => return (position.0, position.1 - 1),
        &Directions::L => return (position.0 - 1, position.1),
        &Directions::R => return (position.0 + 1, position.1),
    }
}


// Since the tail can move diagonally, the max distance between X and Y is the distance.
fn calculate_distance(positions : &((i32, i32), (i32, i32))) -> i32 {
    cmp::max((positions.0.0 - positions.1.0).abs(), (positions.0.1 - positions.1.1).abs())
}


// Given the positions of head and tail, moving the tail to follow the head.
fn follow_head(positions : &((i32, i32), (i32, i32))) -> (i32, i32)
{
    // Check if already in contact:
    let x_distance = positions.0.0 - positions.1.0;
    let y_distance = positions.0.1 - positions.1.1;
    if cmp::max(x_distance.abs(), y_distance.abs()) <= 1 {
        
        // The tail is already in a comfortable position.
        return positions.1;
    }

    // Tail follows head in order to be always in contact in the closest way.
    let mut tail_pos = positions.1.clone();
    if x_distance.abs() > 0 {
        tail_pos.0 += (x_distance).signum();
    }
    if y_distance.abs() > 0 {
        tail_pos.1 += (y_distance).signum();
    }
    tail_pos
}


// For Part 2, the follow is called multiple times!
// Given the positions of head and tail, moving multiple times along segment with the head to follow.
fn move_chain(positions : &Vec<(i32, i32)>, direction: &Directions) -> Vec<(i32, i32)> {
    let mut out_positions =  Vec::<(i32, i32)>::new();
    out_positions.push(move_head(&positions.first().unwrap(), direction));

    for idx in 1..positions.len() {
        out_positions.push(follow_head(&(out_positions.last().unwrap().clone(), positions[idx])));
    }

    out_positions
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
    let mut commands_vect = Vec::<(Directions, i32)>::new();

    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            let mut split_line = line.split(' '); // This should not be MUT, but they ask it for the following borrows
            match split_line.next().unwrap() {
                "U" => commands_vect.push((Directions::U, split_line.next().unwrap().parse::<i32>().unwrap())),
                "D" => commands_vect.push((Directions::D, split_line.next().unwrap().parse::<i32>().unwrap())),
                "L" => commands_vect.push((Directions::L, split_line.next().unwrap().parse::<i32>().unwrap())),
                "R" => commands_vect.push((Directions::R, split_line.next().unwrap().parse::<i32>().unwrap())),
                _ => panic!("Wrong input syntax!"),
            }
        }
    }
    println!("read {} lines from input", commands_vect.len());
    assert!(commands_vect.len() > 1);

    // Positions
    let mut head_tail_positions = ((0,0), (0,0));

    // tracking all tails positions
    let mut all_tail_positions = Vec::<(i32,i32)>::new();

    // Applying the commands, and adding to the vector of all the positions of tail
    for command in &commands_vect {
        for _ in 0..command.1 {
            let new_positions = move_segment(&head_tail_positions, &command.0, );
            all_tail_positions.push(new_positions.1);
            head_tail_positions = new_positions;
        }
    }

    // Removing duplicate positions
    all_tail_positions.sort();
    all_tail_positions.dedup();
    let result_part_1 = all_tail_positions.len() as u32;

    // For Part 2, we now have TEN knots! 
    let mut all_links_positions = vec![(0, 0); 10];

    // tracking all tails positions
    let mut all_tail_positions = Vec::<(i32,i32)>::new();
    all_tail_positions.push((0,0));

    // Applying the commands, and adding to the vector of all the positions of tail
    for command in &commands_vect {
        for _ in 0..command.1 {
            let new_positions = move_chain(&all_links_positions, &command.0, );
            all_tail_positions.push(new_positions.last().unwrap().clone());
            all_links_positions = new_positions;
        }
    }

    // Again removing duplicates in the vector.
    all_tail_positions.sort();
    all_tail_positions.dedup();
    let result_part_2 = all_tail_positions.len() as u32;

    Some((result_part_1, result_part_2))
}


// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 9!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 13);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test_2.txt".to_string()).unwrap().1, 36);
    }    
}