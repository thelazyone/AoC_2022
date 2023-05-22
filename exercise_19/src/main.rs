// Exercise 19: calculating the path of a cursor following the movement of another

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::max;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ResourceType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
    ResourcesNumber = 4,
}
type ResourcesVect = [u32; ResourceType::ResourcesNumber as usize];


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BuildingCommand {
    Build(ResourceType),
    WaitFor(Vec<ResourceType>),
}


#[derive(Debug)]
struct Blueprint {
    robot_costs : HashMap<u32, ResourcesVect>,
}

impl Blueprint {
    fn new_from_line(line : &String) -> Blueprint {
        let regex_string = 
        r"\D+costs\s(?P<val1>\S+)\D+costs\s(?P<val2>\S+)\D+costs\s(?P<val3>\S+)\sore and\s(?P<val4>\S+)\D+costs\s(?P<val5>\S+)\sore and\s(?P<val6>\S+)";
        let regex = Regex::new(regex_string).unwrap();
        match regex.captures(line) {
            Some(caps) => {
                let val1 = caps.name("val1").unwrap().as_str().parse::<u32>().unwrap();
                let val2 = caps.name("val2").unwrap().as_str().parse::<u32>().unwrap();
                let val3 = caps.name("val3").unwrap().as_str().parse::<u32>().unwrap();
                let val4 = caps.name("val4").unwrap().as_str().parse::<u32>().unwrap();
                let val5 = caps.name("val5").unwrap().as_str().parse::<u32>().unwrap();
                let val6 = caps.name("val6").unwrap().as_str().parse::<u32>().unwrap();
                
                Blueprint {
                    robot_costs : [
                        (ResourceType::Ore as u32, vec!{val1, 0, 0, 0}.try_into().unwrap()),
                        (ResourceType::Clay as u32, vec!{val2, 0, 0, 0}.try_into().unwrap()),
                        (ResourceType::Obsidian as u32, vec!{val3, val4, 0, 0}.try_into().unwrap()),
                        (ResourceType::Geode as u32, vec!{val5, 0, val6, 0}.try_into().unwrap())].iter().cloned().collect()
                }
            }
            None => panic!("Error reading line: {}b", line),
        }
    }

    fn calculate_maximum_geode_yield(&self, max_steps : u32) -> u32 {

        // Calculating a support max_cost variable:
        let mut max_costs = vec![0; ResourceType::ResourcesNumber as usize];
        for idx_cost in 0..ResourceType::ResourcesNumber as usize {
            for idx_robot in 0..ResourceType::ResourcesNumber as usize {
                max_costs[idx_cost] = max(max_costs[idx_cost], self.robot_costs[&(idx_robot as u32)][idx_cost])
            }
        }
        max_costs[ResourceType::Geode as usize] = std::u32::MAX;

        // Starting with a set of current robots (1 ore, none of the rest)
        // and passing it in a recursive function.        
        // Calling command "none" the first iteration, which means that no robot will be built
        // This is ok with the input data, but it is a limit of the implementation
        self.iterative_step_factory(
            max_steps, 
            0,
            &vec!{1, 0, 0, 0}.try_into().unwrap(), 
            &vec!{0, 0, 0, 0}.try_into().unwrap(), 
            BuildingCommand::WaitFor(Vec::<ResourceType>::new()),
            &max_costs)
    }

    fn get_available_commands(
        &self,
        last_command : BuildingCommand,
        remaining_steps : u32,
        resources: &ResourcesVect, 
        robots: &ResourcesVect, 
        best_score: u32,
        max_costs: &Vec<u32>,
        ) -> Vec<BuildingCommand> {
            
            let mut orders_vector = Vec::<BuildingCommand>::new();

            // If there is NO way to get to the "best" result in time, returning with no possible outputs:
            let mut max_possible_value = resources[ResourceType::Geode as usize];
            max_possible_value += robots[ResourceType::Geode as usize] * remaining_steps;
            max_possible_value += ((remaining_steps-1) as f32 * (remaining_steps as f32/ 2.)) as u32;
            if max_possible_value <= best_score {
                return orders_vector;
            }

            // If there's resources for a Geode, ignore all the others. This might be a wrong criteria, though.
            if Blueprint::resources_enough_for(resources, &self.robot_costs[&(ResourceType::Geode as u32)]){
                orders_vector.push(BuildingCommand::Build(ResourceType::Geode));
                return orders_vector;
            }

            // For each type of robot, adding only if there aren't enough and if it's affordable:
            for idx in 0..ResourceType::ResourcesNumber as usize {

                // If there are enough robots to cover *ANY* purchase, skip
                if robots[idx] >= max_costs[idx] {
                    continue;
                }

                // If you have more resources of a kind that you could possibly use, skip:
                if resources[idx] >= max_costs[idx] * remaining_steps {
                    continue;
                }

                // If the last command was a wait and the specific resource was one that could have been
                // built in the last command:
                if let BuildingCommand::WaitFor(delayed_command) = &last_command {
                    if delayed_command.contains(&(Blueprint::get_resource_type(idx as u32))){
                        continue;
                    }
                }

                // Finally adding the order, if there are resources enough to build.
                if Blueprint::resources_enough_for(resources, &self.robot_costs[&(idx as u32)]) {
                    orders_vector.push(BuildingCommand::Build(Blueprint::get_resource_type(idx as u32)))
                }
            }

            // Reversing the vector to get a better chance with higher-tier robots first.
            orders_vector.reverse();

            // Adding "rest" only if there is some robot that needs time to be purchased.
            let mut all_robots_purchaseable = true;
            for idx in 0..ResourceType::ResourcesNumber as usize {

                // 1 if you only have ore, you should check for clay and obsidian only.
                if robots[ResourceType::Clay as usize] == 0 && 
                robots[ResourceType::Obsidian as usize] == 0 && 
                robots[ResourceType::Geode as usize] == 0 &&
                idx > 1 {
                    continue;
                }

                // If you only have ore and clay, ignore the geode (idx 3):
                if robots[ResourceType::Obsidian as usize] == 0 && 
                robots[ResourceType::Geode as usize] == 0 &&
                idx > 2 {
                    continue;
                }

                if !Blueprint::resources_enough_for(resources, &self.robot_costs[&(idx as u32)]){
                    all_robots_purchaseable = false;
                }
            }
            if !all_robots_purchaseable {
                let building_orders = orders_vector.iter()
                .filter_map(|command| {
                    if let BuildingCommand::Build(resource) = command {
                        Some(resource.clone())
                    } else {
                        None
                    }
                })
                .collect();
                orders_vector.push(BuildingCommand::WaitFor(building_orders));
            }

            orders_vector
        }
    

    fn iterative_step_factory (
        &self, 
        remaining_steps : u32, 
        current_best : u32,
        current_robots : &ResourcesVect, 
        current_resources : &ResourcesVect,
        command_to_build : BuildingCommand,
        max_costs : &Vec<u32>) -> u32 /*current result*/ {

        // If the conditions are such that pruning is necessary, doing so.
        let mut new_resources = current_resources.clone();
        let mut new_robots = current_robots.clone();

        // Checking the command, executing.
        if let BuildingCommand::Build(resource_type) = command_to_build {
            // Consuming the resources
            new_resources = Blueprint::subtract_resources(&new_resources, &self.robot_costs[&(resource_type as u32)]);

            // Adding a robot. But using the "current robots" value to calculate the extra resources later on.
            new_robots[resource_type as usize] += 1
        }

        // Adding the resources, one for each robot of that kind.
        for idx in 0..ResourceType::ResourcesNumber as usize {
            new_resources[idx] += current_robots[idx]; // Using the robots BEFORE the new one has been created.
        }

        // If the max iteration has been reached, returning the value.
        let final_score = new_resources[ResourceType::Geode as usize];
        if remaining_steps <= 0 {
            return final_score;
        }

        let mut new_best = current_best;
        if current_best < new_resources[ResourceType::Geode as usize]
        {
            new_best = new_resources[ResourceType::Geode as usize];
        }

        // If we have more steps, sending all the commands for future branches.
        let orders = self.get_available_commands(
            command_to_build,
            remaining_steps.clone(), 
            &new_resources,
            &new_robots,
            new_best,
            max_costs);
        if orders.is_empty(){
            return new_resources[ResourceType::Geode as usize];
        }
        
        // All final geodes results from the branching
        let mut all_scores = Vec::<u32>::new();
        for order in orders {
            all_scores.push(self.iterative_step_factory(
                remaining_steps - 1,
                new_best,
                &new_robots,
                &new_resources,
                order,
                max_costs));
        }

        // Returning the best of the results.
        all_scores.iter().max().unwrap_or(&new_resources[ResourceType::Geode as usize]).clone()
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
            0 => ResourceType::Ore,
            1 => ResourceType::Clay,
            2 => ResourceType::Obsidian,
            3 => ResourceType::Geode,
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
    assert!(lines_vec.len() >= 1);

    // Converting each line in a blueprint.
    let mut all_blueprints = Vec::<Blueprint>::new();
    for line in lines_vec {
        let blueprint = Blueprint::new_from_line(&line);
        println!("Read blueprint with robot costs: {:?}", &blueprint);
        all_blueprints.push(blueprint);
    }

    // For each blueprint calculating the maximum efficiency
    let mut cumulative_result = 0;
    for (index, blueprint) in all_blueprints.iter().enumerate(){
        let max_efficiency = blueprint.calculate_maximum_geode_yield(24 - 1);
        cumulative_result += max_efficiency * (index as u32 + 1);
        println!("Efficiency for blueprint is {}", max_efficiency);
    }

    result_part_1 = cumulative_result;

    // Part 2 is with 32 iterations, but only 3 blueprints.
    let mut cumulative_result = 1;
    for (index, blueprint) in all_blueprints.iter().enumerate(){
        let max_efficiency = blueprint.calculate_maximum_geode_yield(32 - 1);
        cumulative_result *= max_efficiency;
        println!("Efficiency for blueprint is {}", max_efficiency);
        if index >= 2 {
            break;
        }
    }
    result_part_2 = cumulative_result;
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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 33);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 62*56);
    }    
}