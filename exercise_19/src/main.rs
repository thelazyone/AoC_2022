// Exercise 19: calculating the path of a cursor following the movement of another

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum ResourceType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
    ResourcesNumber = 4,
}
type ResourcesVect = [u32; ResourceType::ResourcesNumber as usize];

struct Blueprint {
    robot_costs : HashMap<u32, ResourcesVect>,
}
impl Blueprint {
    fn new_from_line(line : &String) -> Blueprint {
        let regex_string = 
        r"ore robot costs\s(?P<val1>\S+)\D+clay robot costs(?P<val2>\S+)";
        let regex = Regex::new(regex_string).unwrap();
        match regex.captures(line) {
            Some(caps) => {
                let val1 = caps.name("val1").unwrap().as_str().parse::<u32>().unwrap();
                let val2 = caps.name("val2").unwrap().as_str().parse::<u32>().unwrap();
                
                Blueprint {
                    robot_costs : [
                        (ResourceType::Ore as u32, vec!{val1, 0, 0, 0}.try_into().unwrap()),
                        (ResourceType::Clay as u32, vec!{val2, 0, 0, 0}.try_into().unwrap()),
                        (ResourceType::Obsidian as u32, vec!{0, 0, 0, 0}.try_into().unwrap()),
                        (ResourceType::Geode as u32, vec!{0, 0, 0, 0}.try_into().unwrap())].iter().cloned().collect()
                }
            }
            None => panic!("Error reading line: {}b", line),
        }
    }

    fn calculate_maximum_geode_yield(&self, max_steps : u32) -> u32 {

        // Starting with a set of current robots (1 ore, none of the rest)
        // and passing it in a recursive function.
        
        // Calling command "none" the first iteration, which means that no clay robot will be built
        // This is ok with the input data, but it is a limit of the implementation
        self.iterative_step_factory(max_steps, &vec!{1, 0, 0, 0}.try_into().unwrap(), &vec!{0, 0, 0, 0}.try_into().unwrap(), None)
    }

    fn iterative_step_factory (
        &self, 
        remaining_steps : u32, 
        current_robots : &ResourcesVect, 
        current_resources : &ResourcesVect,
        command_to_build : Option<ResourceType>) -> u32 {

        let mut new_resources = current_resources.clone();
        let mut new_robots = current_robots.clone();
        
        // Checking the command, executing.
        if let Some(resource_type) = command_to_build {
            if Blueprint::resources_enough_for(&new_resources, &self.robot_costs[&(resource_type as u32)]) {

                // Consuming the resources
                new_resources = Blueprint::subtract_resources(&new_resources, &self.robot_costs[&(resource_type as u32)]);

                // Adding a robot. But using the "current robots" value to calculate the extra resources later on.
                new_robots[resource_type as usize] += 1
            }
            
            // If there are no resources, closing this branch: it's an invalid one.
            return 0;
        }

        // Adding the resources, one for each robot of that kind.
        for idx in 0..ResourceType::ResourcesNumber as usize {
            new_resources[idx] += current_robots[idx];
        }

        // If the max iteration has been reached, returning the value.
        if remaining_steps == 0 {
            return new_resources[ResourceType::Geode as usize]
        }

        // If we have more steps, sending all the commands for future branches.

        // All final geodes results from the branching
        let mut all_scores = Vec::<u32>::new();
        for idx in 0..ResourceType::ResourcesNumber as usize {
            all_scores.push(self.iterative_step_factory(
                remaining_steps - 1,
                &new_robots,
                &new_resources,
                Some(Blueprint::get_resource_type(idx as u32))));
        }
        
        // Pushing the "do nothing" command too.
        all_scores.push(self.iterative_step_factory(
            remaining_steps - 1,
            &new_robots,
            &new_resources,
            None));

        // Returning the best of the results.
        all_scores.iter().max().unwrap().clone()
    }

    // Utility functions

    fn resources_enough_for (a : &ResourcesVect, b : &ResourcesVect) -> bool {
        for idx in 0..ResourceType::ResourcesNumber as usize {
            if a[idx] < b[idx] {
                return false;
            }
        }
        true
    }
    
    fn subtract_resources (a : &ResourcesVect, b : &ResourcesVect) -> ResourcesVect {

        // Sanity Check: 
        if !Blueprint::resources_enough_for(a, b)
        {
            panic!("Invalid subtraction of {:?} from {:?}", b, a);
        }

        // Subtracting.
        let mut result = a.clone();
        for idx in 0..ResourceType::ResourcesNumber as usize{
            result[idx] -= b[idx];
        }
        result
    }

    fn get_resource_type (number : u32) -> ResourceType {
        match number {
            0 => ResourceType::Clay,
            1 => ResourceType::Clay,
            2 => ResourceType::Clay,
            3 => ResourceType::Clay,
            _ => panic!("no resource with this value!")
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

    // Converting each line in a blueprint.
    let mut all_blueprints = Vec::<Blueprint>::new();
    for line in lines_vec {
        all_blueprints.push(Blueprint::new_from_line(&line));
    }

    // For each blueprint calculating the maximum efficiency
    for blueprint in all_blueprints {

    }


    result_part_1 = 0;
    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 19!");

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