// Exercise 21: decypher a set of unknown variables in a very long linear system
// There are SEVERAL approaches to solve systems of linear equations, but since these 
// are extremely sparse, I'll first try with a brute-force approach.

// For reading/parsing
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

// utility
struct Operation {
    first : String,
    second : String,
    operation : fn(i64, i64) -> i64,
}

enum Statement {
    Value(i64),
    Function(Operation),
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
            "+"=>|val1, val2|{val1 + val2},
            "-"=>|val1, val2|{val1 - val2},
            "*"=>|val1, val2|{val1 * val2},
            "/"=>|val1, val2|{val1 / val2},
            _=>panic!("Wrong operator!"),
        };
        return (name.to_string(), Statement::Function(Operation {
            first: split_function_str[0].parse().unwrap(),
            second: split_function_str[2].parse().unwrap(),
            operation: operator,
        }))
    }
}

fn simplify_statements_map (map : &mut HashMap<String, Statement>) {

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
                        statement_new_value = (operation.operation)(*val1, *val2);
                        println!("Found statement {}", name);
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
            if let Statement::Function(_) = map.get(&"root".to_string()).unwrap() {
                panic!("Root is not solved yet!");
            }

            println!("No more elements to simplify, root has been solved");
            return;
        }
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

    simplify_statements_map(&mut statements_map);
    if let Statement::Value(root_value ) = statements_map.get(&"root".to_string()).unwrap() {
        result_part_1 = root_value.clone();
    }
    else {
        panic!("Part 1 failed!");
    }

    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day XXX!");

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