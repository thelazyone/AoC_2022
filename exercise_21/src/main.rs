// Exercise 21: decypher a set of unknown variables in a very long linear system
// There are SEVERAL approaches to solve systems of linear equations, but since these 
// are extremely sparse, I'll first try with a brute-force approach.

// For reading/parsing
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

// utility

#[derive(Debug, Clone)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Operation {
    first : String,
    second : String,
    operation : OperationType,
}

#[derive(Debug, Clone)]
enum Statement {
    Value(i64),
    Human(),
    Function(Operation),
}

fn get_operation(operation_type : &OperationType) -> fn(i64, i64) -> i64 {
    match operation_type {
        OperationType::Add=>|val1, val2|{val1 + val2},
        OperationType::Sub=>|val1, val2|{val1 - val2},
        OperationType::Mul=>|val1, val2|{val1 * val2},
        OperationType::Div=>|val1, val2|{val1 / val2},
    }
}

fn get_line_statement (line : String) -> (String, Statement) {

    // Splitting between name and content.
    let split_str = line.split(": ").collect::<Vec<&str>>();
    let name = split_str[0];

    // If it's just a value
    if !split_str[1].contains(" ") {
        return (name.to_string(), Statement::Value(split_str[1].parse().unwrap()));
    }

    // If it's a full function
    else {
        let split_function_str = split_str[1].split(" ").collect::<Vec<&str>>();
        let operator = match split_function_str[1] {
            "+"=>OperationType::Add,
            "-"=>OperationType::Sub,
            "*"=>OperationType::Mul,
            "/"=>OperationType::Div,
            _=>panic!("Wrong operator!"),
        };
        return (name.to_string(), Statement::Function(Operation {
            first: split_function_str[0].parse().unwrap(),
            second: split_function_str[2].parse().unwrap(),
            operation: operator,
        }))
    }
}

fn simplify_statements_map (map : &mut HashMap<String, Statement>, use_humn : bool) {

    // First replacing "humn" 
    if use_humn{
        let _ = map.insert("humn".to_string(), Statement::Human());
    }

    // For each element in the vector, if it's not a "value" searching
    // for the two members. If both members are values, replacing that with a value and returning.
    let mut statement_new_value : i64 = 0;
    loop {
        let mut last_statement_found : Option<String> = None;
        for (name, statement) in map.iter() /* Unmutable iterator */ {
            if let Statement::Function(operation) = statement{
                if let Some(Statement::Value(val1)) = map.get(&operation.first) {
                    if let Some(Statement::Value(val2)) = map.get(&operation.second) {
                        last_statement_found = Some(name.clone());
                        statement_new_value = (get_operation(&operation.operation))(*val1, *val2);
                        break;
                    }
                }
            }
        }

        // If a new statement has been found, updating the map.
        if let Some (statement) = last_statement_found {


            let _ = map.insert(statement, Statement::Value(statement_new_value));
        }
        else {
            // If part 1 i am expecting this to solve it all.
            if !use_humn {
                if let Statement::Function(_) = map.get(&"root".to_string()).unwrap() {
                    panic!("Root is not solved yet!");
                }
            }

            println!("No more elements to simplify, root has been solved");
            return;
        }
    }
}


// TODO used only once for root - TODO RENAME
fn get_operation_and_value (map : &HashMap<String, Statement>, input_key : String) -> Option<(&String, &Operation, &i64)> {
    let mut key = None;
    let mut operation: Option<&Operation> = None;
    let mut value = None;
    if let Statement::Function(root_operation) = map.get(&input_key).unwrap() {
        if let Statement::Function(operation_temp) = map.get(&root_operation.first).unwrap() {
            if let Statement::Value(value_temp) = map.get(&root_operation.second).unwrap() {
                key = Some(&root_operation.first);
                operation = Some(operation_temp);
                value = Some(value_temp);
            }
        } 
        else if let Statement::Value(value_temp) = map.get(&root_operation.first).unwrap() {
            if let Statement::Function(operation_temp) = map.get(&root_operation.second).unwrap() {
                key = Some(&root_operation.second);
                operation = Some(operation_temp);
                value = Some(value_temp);
            }
        } 
        else {
            panic!("root is not right! {:?}", root_operation);
        }
    }

    Some ((key.unwrap(), operation.unwrap(), value.unwrap()))
}


fn solve_with_humn (map : &HashMap<String, Statement>) -> i64 /* Value of humn */ {
    // Starting from root, but the operation is ignored, simply taking the statement
    // that is not a value and expecting it to be a value
    let root_stats = get_operation_and_value(map, "root".to_string()).unwrap();

    println!("Starting with root with value {} and operation {:?}", root_stats.2, root_stats.1);

    solve_operation_iteratively(&map, root_stats.1, root_stats.2)
}


fn solve_operation_iteratively (map : &HashMap<String, Statement>, operation : &Operation, input_value : &i64) -> i64 {

    // Given the operation i'm looking for both sides. Being a bit verbose but it should work.
    let partial_value: &i64;
    let is_first_element_value;
    let mut next_operation = None;
    let mut is_last_iteration = false;
    if let Statement::Value(temp_value) = map.get(&operation.first).unwrap() {
        partial_value = temp_value;
        is_first_element_value = true;
        if let Statement::Human() = map.get(&operation.second).unwrap() {
            is_last_iteration = true;
        }
        else if let Statement::Function(operation) = map.get(&operation.second).unwrap() {
            next_operation = Some(operation);
        }
        else {
            panic!("unexpected operation type");
        }
    }
    else if let Statement::Value(temp_value) = map.get(&operation.second).unwrap() {
        partial_value = temp_value;
        is_first_element_value = false;
        if let Statement::Human() = map.get(&operation.first).unwrap() {
            is_last_iteration = true;
        }
        else if let Statement::Function(operation) = map.get(&operation.first).unwrap() {
            next_operation = Some(operation);
        }
        else {
            panic!("unexpected operation type");
        }
    }    
    else {
        panic!("No element is a value in the operation!");
    }

    // Inverse of the original operation: 
    let resulting_value= match operation.operation {
        OperationType::Add=>input_value - partial_value,
        OperationType::Sub=>{
            if is_first_element_value {
                partial_value - input_value
            }
            else {
                input_value + partial_value
            }
        }
        OperationType::Mul=>input_value / partial_value,
        OperationType::Div=>input_value * partial_value,
    };

    if is_last_iteration {
        return resulting_value;
    }
    else {
        return solve_operation_iteratively(&map, next_operation.unwrap(), &resulting_value);
    }
}


// Primary Function
fn execute (input_path : String)  -> Option<(i64, i64)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : i64;
    let result_part_2 : i64;

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

    // Storing in a vector.
    let mut statements_map = HashMap::<String, Statement>::new();
    for line in lines_vec {
        //println!("line is {}", line);
        let statement = get_line_statement(line);
        statements_map.insert(statement.0, statement.1);
    }
    let mut statements_map_part_1 = statements_map.clone();
    simplify_statements_map(&mut statements_map_part_1, false);
    if let Statement::Value(root_value ) = statements_map_part_1.get(&"root".to_string()).unwrap() {
        result_part_1 = root_value.clone();
        println!("part 1 completed.")
    }
    else {
        panic!("Part 1 failed!");
    }

    // For part 2 first solving everything that is not "contamined" by the human
    println!("Starting part 2...");
    let mut statements_map_part_2= statements_map.clone();
    simplify_statements_map(&mut statements_map_part_2, true /* Using humn */);
    result_part_2 = solve_with_humn(&statements_map_part_2);
    println!("Part 2 completed.");
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 21!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 152);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 301);
    }    
}