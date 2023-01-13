// Exercise 15: intersecting intervals in a 2 dimensions map of sensors and beacons

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;
use regex::Regex;


// Same structure of the code from Day 4 with different regex. 
fn parse_locations(input : &str) -> Option<((i32, i32), (i32, i32))> {

    let regex_string = 
        r"\D+=(?P<val1>\-*\d+)\D+=(?P<val2>\-*\d+)\D+=(?P<val3>\-*\d+)\D+=(?P<val4>\-*\d+)";

    let regex = Regex::new(regex_string).unwrap();

    match regex.captures(input) {
        Some(caps) => {
            let internal_parse = |key| {
                caps.name(key).unwrap().as_str().parse::<i32>().unwrap()
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


#[derive(Clone)]
#[derive(Debug)]
struct Intervals {
    intervals : Vec<(i32, i32)>,
    is_union : bool,
}


impl Intervals {
    fn new() -> Intervals {
        Intervals {intervals : Vec::<(i32, i32)>::new(), is_union : true}
    }

    fn add_interval(&mut self, input:(i32, i32)) {
        self.intervals.push(input);
        self.is_union = false;
    }

    fn get_intervals(&self) -> Vec<(i32, i32)> {
        self.intervals.clone()
    }

    // Provides the union of the current intervals.
    // From
    //      /....../           /......../
    //           /......./              /../
    // To 
    //      /............/     /.........../
    fn union(&mut self) {

        // If already union, skipping.
        if self.is_union {
            return;
        }

        // Merging the intervals where possible.
        // To do so, I store in a single vector the starts and stops of intervals and order them. as long as there
        // are more starts than stop along the direction, we're inside the final intersection.
        let mut margins_vect = Vec::<(i32, i32)>::new(); // true: start; false: end
        for element in &self.intervals {
            margins_vect.push((element.0, 1));
            margins_vect.push((element.1, -1));
        }
        margins_vect.sort_by(|a, b| {a.0.cmp(&b.0)});

        // Now iterating on the margins vector to calculate the total vector.
        let mut margins_counter = 0;
        let mut intersected_intervals = Vec::<(i32, i32)>::new();
        let mut temp_interval = (0,0);

        for element in margins_vect {

            // If the counter STARTS as zero, a new interval has begun.
            if margins_counter == 0 {
                temp_interval.0 = element.0;
            }

            margins_counter += element.1;
            assert!(margins_counter >= 0);

            // If the counter ENDS as zero the new interval is completed.
            if margins_counter == 0 {

                // Interval is done!
                temp_interval.1 = element.0;

                // If the end of the last element is the start of the current, extending the last.
                if !intersected_intervals.is_empty() && intersected_intervals.last().unwrap().1 == temp_interval.0 {
                    intersected_intervals.last_mut().unwrap().1 = element.0;
                }
                else {
                    intersected_intervals.push(temp_interval.clone());
                }
            }
        }

        self.intervals = intersected_intervals;
    }

    // Calculates the total size of the sets AFTER a union.
    fn get_total_size(&mut self) -> u32 {

        // Removing any overlapped intervals.
        self.union();

        let mut total_size : u32 = 0;
        for element in &self.intervals {
            total_size += (element.1 - element.0) as u32 + 1;
        }
        total_size
    }

    // Intersects with a single set.
    // From /....../    /../   /......../
    // With       /............../     
    // To         //    /../   /./
    fn intersect_with(&mut self, input_interval : (i32, i32)) {

        // Removing any overlapped intervals.
        self.union();

        // Removing all elements that are entirely outside of the range.
        self.intervals.retain(|elem| elem.1 >= input_interval.0 && elem.0 <= input_interval.1);

        // Setting the limits for the remaining sets. Up to two will intersect the
        // input interval, since they are all not overlapped.
        for element in &mut self.intervals {
            element.0 = cmp::max(element.0, input_interval.0);
            element.1 = cmp::min(element.1, input_interval.1);
        }
    }

    // Subtracts with a single set.
    // From /....../    /../   /......../
    // With       /............../     
    // To   /..../                /...../
    fn difference_with(&mut self, input_interval : (i32, i32)) {

        // Removing any overlapped intervals.
        self.union();

        // Removing all elements that are entirely inside of the range.
        self.intervals.retain(|elem| elem.0 < input_interval.0 || elem.1 > input_interval.1);

        // Setting the limits for the remaining sets, just like for the intersection. 
        for element in &mut self.intervals {
            if element.0 >= input_interval.0 {element.0 = cmp::max(element.0, input_interval.1 + 1)};
            if element.1 <= input_interval.1 {element.1 = cmp::min(element.1, input_interval.0 - 1)};
        }
    }
}


// Given the beacons at certain distances, it creates the "exclusion" intervals.
fn make_exclusion_zone (
    input_positions : &Vec<((i32, i32), (i32, i32))>, 
    test_line : i32) -> Intervals {

    let mut intervals_struct = Intervals::new();
    for element in input_positions {
        // Measuring Manhattan distance from its beacon:
        let distance_to_beacon = (element.0.0 - element.1.0).abs() + (element.0.1 - element.1.1).abs();

        // If the distance is less than the distance from the test line, skipping.
        let intersection_with_line = distance_to_beacon - (element.0.1 - test_line).abs();
        if intersection_with_line <= 0 {
            continue;
        }

        // Otherwise, applying the exclusion.
        intervals_struct.add_interval((element.0.0 - intersection_with_line, element.0.0 + intersection_with_line));
    }

    // Retrieving the struct.
    intervals_struct
}


// Primary Function
fn execute (input_path : String, test_line : i32, square_side : u32)  -> Option<(u64, u64)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : u64;
    let result_part_2 : u64;

    // First reading the input string - easy.
    let mut lines_vec = Vec::<String>::new();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            lines_vec.push(line);
        }
    }
    println!("read {} lines from input", lines_vec.len());
    assert!(!lines_vec.is_empty());

    // Parsing each line, retrieving the two sets of coordinates.
    let mut sensors_and_beacons = Vec::<((i32, i32), (i32, i32))>::new();
    for line in lines_vec {
        if let Some(coords) = parse_locations(&line) {
            sensors_and_beacons.push(coords);
        }
    }

    // For Part 1, checking how many slots for a given line can NOT contain a beacon
    // This is done by running each sensor and see how many slots fall in their
    // exclusion zone.
    let mut intervals_part_1 = make_exclusion_zone(
        &sensors_and_beacons, 
        test_line);

    // Checking how many beacons and sensors exist within the interval and
    // counting the remaining spaces that MUST be empty.
    let mut occupied_spaces = Vec::<i32>::new();
    for element in &sensors_and_beacons {
        if element.0.1 == test_line {occupied_spaces.push(element.0.0);};
        if element.1.1 == test_line {occupied_spaces.push(element.1.0);};
    }
    occupied_spaces.sort();
    occupied_spaces.dedup();
    result_part_1 = intervals_part_1.get_total_size().checked_sub(occupied_spaces.len() as u32).unwrap() as u64;

    // For part 2, the search is performed on a 4 millions x 4 millions square area.
    // The optimization done above should work here.
    let mut found_slots = Vec::<u64>::new();
    for line_idx in 0..square_side as i32 {
        let mut current_interval = make_exclusion_zone(
            &sensors_and_beacons, 
            line_idx as i32);
        current_interval.intersect_with((0, square_side as i32));
        if current_interval.get_total_size() != square_side + 1 {            
            // Now inverting the interval (difference with the full range)
            let mut full_range = Intervals::new();
            full_range.add_interval((0, square_side as i32));
            for single_interval in current_interval.get_intervals() {
                full_range.difference_with(single_interval);
            }
            let free_slot = ( 
                full_range.get_intervals().first().unwrap().0 as u64,
                line_idx as u64,);

                found_slots.push(free_slot.0 * square_side as u64 + free_slot.1);

            println!("found a slot in x {} and y {} {}", free_slot.0, free_slot.1, square_side);
        }

        if line_idx % 1000000 == 0 {
            println!("parsing line {}", line_idx);
        }
    }

    // There should only be ONE point remaining!
    assert!(found_slots.len() == 1);
    result_part_2 = found_slots[0];

    Some((result_part_1, result_part_2))
}


// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 15!");

    //let results = execute("./data/test.txt".to_string(), 10, 20).unwrap();
    let results = execute("./data/input.txt".to_string(), 2000000, 4000000).unwrap();
    
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
        assert_eq!(execute("./data/test.txt".to_string(), 10, 4000000).unwrap().0, 26);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string(), 10, 20).unwrap().1, 291);
    }    
}