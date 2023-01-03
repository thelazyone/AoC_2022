// Exercise 13: validating the ordering of lists of lists of ... you get it.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;

#[derive(Clone)]
#[derive(Debug)]
enum OrderedListValue {
    Number(i32),
    Vector(Vec<OrderedListValue>),
}

fn convert_to_string (input : OrderedListValue ) -> String {
    match input {
        OrderedListValue::Number(value) => return value.to_string(),
        OrderedListValue::Vector(vect) => {
            
            // Converting to string (recursively) each element of the vector. 
            // Then removing the last comma and adding brackets.
            let mut tempString = vect.into_iter().map(|val| convert_to_string(val) + ",").collect::<String>();
            tempString.pop();
            return format!("[{}]", tempString);
        },
    };
}

fn parse_element (input : String) -> (OrderedListValue, Option<String>) {
    
    // If first element is not bracket, retrieving it (till the next ',') 
    // and returning the rest.
    if !input.starts_with("["){
        //println!("element is number");
        match input.split_once(",") {
            Some(splits) => return (OrderedListValue::Number(splits.0.parse::<i32>().unwrap()), Some(splits.1.to_string())),
            None => return (OrderedListValue::Number(input.parse::<i32>().unwrap()), None),
        };
    }
    else {
        // Searching for the closing of [. If more brackets open, waiting for them to close.
        // Applying a simple approach, but Regex could be probably used too.
        let mut counter = 0;
        for (index, character) in input.clone().chars().enumerate() {
            match character {
                '[' => counter += 1,
                ']' => counter -= 1,
                _ => continue,
            }
            if counter == 0 {
                //println!("found closing parenthesis at {}", index);
                if input.len() > index + 1 {
                    return (parse_list(input[0..index + 1].to_string()), Some(input[index + 2..].to_string()));
                } 
                return (parse_list(input[0..index + 1].to_string()), None);
            }
        }
    }

    panic!("No element found to parse!");
}

fn parse_list (input : String) -> OrderedListValue {

    let mut elementsVector = Vec::<OrderedListValue>::new();

    // Looking for the two outmost square parenthesis
    //println!("DEBUG: parse_list input {}", input);
    let mut substrings = Some(input.split_once("[").unwrap().1.rsplit_once("]").unwrap().0.to_string());
    //println!("DEBUG: parse_list input STR {:?}", substrings);

    // Special case: if the substring is empty, returning and empty vector!
    // Note: the .clone().unwrap() implies a copy while i'm just reading it.
    // A borrowing would be better.
    if substrings.clone().unwrap().is_empty() {
        return OrderedListValue::Vector(Vec::<OrderedListValue>::new());
    }

    // Splitting numbers and lists.
    while substrings.is_some() { // TODO bad while
        let list_elem : OrderedListValue;
        (list_elem, substrings) = parse_element(substrings.unwrap().to_string());
        //println!("Adding {:?}, remaining is {:?}", list_elem, substrings);
        elementsVector.push(list_elem);
    }

    OrderedListValue::Vector(elementsVector)
}

fn check_ordered(mut left : OrderedListValue, mut right : OrderedListValue) -> bool {

    println!("Comparing {:?} and {:?}", convert_to_string(left.clone()), convert_to_string(right.clone()));

    // If both are numbers, just comparing
    if let (OrderedListValue::Number(left_number), OrderedListValue::Number(right_number)) = (&left, &right) {
        println!("result is {}",left_number <= right_number);
        return left_number <= right_number;
    } 

    // If one of the two is number, replacing with vec with one element instead.
    if let OrderedListValue::Number(left_number) = &left {
        println!("converting {}",left_number);
        left = OrderedListValue::Vector(vec!(OrderedListValue::Number(left_number.clone())));
    }    
    if let OrderedListValue::Number(right_number) = &right {
        println!("converting {}",right_number);
        right = OrderedListValue::Vector(vec!(OrderedListValue::Number(right_number.clone())));
    }

    // Expecting both to be lists now.
    if let (OrderedListValue::Vector(left_vector), OrderedListValue::Vector(right_vector)) = (&left, &right) {
        println!("it's vector");

        // if left_vector.len() > right_vector.len() {
        //     println!("result is false (len {} > {})",left_vector.len() ,right_vector.len());
        //     return false;
        // }

        // If at least one is a list, treating as list.
        // I could have used itertools.zip_shortest() but I wanted to stick with
        // few modules for now.
        for index in 0..cmp::min(left_vector.len(), right_vector.len()) {
            if !check_ordered(left_vector[index].clone(), right_vector[index].clone()) {
                println!("result is false (subv)");
                return false;
            }
        }
    } 

    println!("result is true");
    true
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

    // Parsing the lines to create lists of lists
    let mut good_pairs_counter : u32 = 0;
    for (index, list_pairs) in lines_vec.chunks(3).enumerate() {
        let left_list = parse_list(list_pairs[0].clone());
        let right_list = parse_list(list_pairs[1].clone());

        println!("Original is:\n{}\n{}\nParsed is:\n{}\n{}",
            list_pairs[0].clone(),
            list_pairs[1].clone(),
            convert_to_string(left_list.clone()),
            convert_to_string(right_list.clone())
        );
        
        if check_ordered(left_list, right_list) {
            good_pairs_counter += index as u32 + 1 /* it's a 1-based index*/;
            println!("Ordering is correct!");
        }
    }
    result_part_1 = good_pairs_counter;

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 13!");

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
        //assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}