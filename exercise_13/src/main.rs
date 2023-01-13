// Exercise 13: validating the ordering of lists of lists of ... you get it.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// utility
use std::cmp;


// The vector is either made of numbers or more vectors of the same type
#[derive(Clone)]
#[derive(Debug)]
enum OrderedListValue {
    Number(i32),
    Vector(Vec<OrderedListValue>),
}


// Parsing a single element of a line. Returns the remaining part of the string, if present.
fn parse_element (input : String) -> (OrderedListValue, Option<String>) {
    
    // If first element is not bracket, retrieving it (till the next ',') 
    // and returning the rest.
    if !input.starts_with("["){
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
                if input.len() > index + 2 {
                    return (parse_list(input[0..index + 1].to_string()), Some(input[index + 2..].to_string()));
                } 
                return (parse_list(input[0..index + 1].to_string()), None);
            }
        }
    }

    panic!("No element found to parse!");
}


// Parsing a whole line.
fn parse_list (input : String) -> OrderedListValue {

    let mut elements_vector = Vec::<OrderedListValue>::new();

    // Looking for the two outmost square parenthesis
    let mut substrings = Some(input.split_once("[").unwrap().1.rsplit_once("]").unwrap().0.to_string());

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
        elements_vector.push(list_elem);
    }

    OrderedListValue::Vector(elements_vector)
}


// Order checking. Note that it might return true, false or None (the two elements are equal)
// A return type of cmp::Ordering could have worked as well, but I learned about it mid-way.
fn check_ordered(mut left : OrderedListValue, mut right : OrderedListValue) -> Option<bool> {

    // If both are numbers, just comparing
    if let (OrderedListValue::Number(left_number), OrderedListValue::Number(right_number)) = (&left, &right) {
        match *left_number as i32 - *right_number as i32 {
            n if n < 0 => return Some(true),
            n if n > 0 => return Some(false),
            0 => return None,
            _ => panic!("it's mathematically impossible to fall here!"),
        };
    } 

    // If one of the two is number, replacing with vec with one element instead.
    if let OrderedListValue::Number(left_number) = &left {
        left = OrderedListValue::Vector(vec!(OrderedListValue::Number(left_number.clone())));
    }    
    if let OrderedListValue::Number(right_number) = &right {
        right = OrderedListValue::Vector(vec!(OrderedListValue::Number(right_number.clone())));
    }

    // Expecting both to be lists now.
    if let (OrderedListValue::Vector(left_vector), OrderedListValue::Vector(right_vector)) = (&left, &right) {
        // If at least one is a list, treating as list.
        // I could have used itertools.zip_shortest() but I wanted to stick with
        // few modules for now.
        for index in 0..cmp::min(left_vector.len(), right_vector.len()) {
            match check_ordered(left_vector[index].clone(), right_vector[index].clone()) {
                Some(result) if !result => return Some(false),
                Some(result) if result => return Some(true),
                None => continue,
                _ => panic!("it's mathematically impossible to fall here!"),
            }
        }

        // If this point is reached it means that all elements have been reached.
        // If the right list still has elements, the result is true.
        match left_vector.len() as i32 - right_vector.len() as i32 {
            n if n < 0 => return Some(true),
            n if n > 0 => return Some(false),
            0 => return None,
            _ => panic!("it's mathematically impossible to fall here!"),
        }
    } 
    // In this case no element is present, so no comparison is done.
    None
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

    // Parsing the lines to create lists of lists
    let mut good_pairs_counter : u32 = 0;
    for (index, list_pairs) in lines_vec.clone().chunks(3).enumerate() {
        let left_list = parse_list(list_pairs[0].clone());
        let right_list = parse_list(list_pairs[1].clone());

        if check_ordered(left_list, right_list).unwrap() {
            good_pairs_counter += index as u32 + 1 /* it's a 1-based index*/;
        }
    }
    result_part_1 = good_pairs_counter;

    // For Part we must then SORT all packets, removing the blank lines, and adding two new packets to the mix.
    lines_vec.retain(|line| !line.is_empty());
    lines_vec.push("[[2]]".to_string());
    lines_vec.push("[[6]]".to_string());
    lines_vec.sort_by(|el_a, el_b| {
        match check_ordered(parse_list(el_a.to_string()), parse_list(el_b.to_string())) {
            Some(true) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Equal,
            _ => std::cmp::Ordering::Greater,
        }
    });

    // Finding [[2]] and [[6]]
    let index_a = lines_vec.iter().position(|line| line == "[[2]]").unwrap() + 1;
    let index_b = lines_vec.iter().position(|line| line == "[[6]]").unwrap() + 1;
    result_part_2 = (index_a * index_b) as u32;

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

    // Generates the input string from the structures. Just for debugging purposes.
    fn convert_to_string (input : OrderedListValue ) -> String {
        match input {
            OrderedListValue::Number(value) => return value.to_string(),
            OrderedListValue::Vector(vect) => {
                
                // Converting to string (recursively) each element of the vector. 
                // Then removing the last comma and adding brackets.
                let mut temp_string = vect.into_iter().map(|val| convert_to_string(val) + ",").collect::<String>();
                temp_string.pop();
                return format!("[{}]", temp_string);
            },
        };
    }


    // General Test
    #[test]
    fn global_test_part_1() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 13);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 140);
    }    
    

    // Parsing test
    #[test]
    fn string_parsing_test() {
        let test_string = "[[9,[],[[7,8,10],1,[7,6,9],[7,5,0],8]],\
        [[3,[],[4,8],[]]],[10,[9,0,7,9],[[0,8,8],[],[6],[4,6],[8,7,0,9]],\
        [10,[],8,[],5]],[[[7,10],[5,10],[4,0,8,3,9],[]],[2]]]".to_string();

        assert!(test_string.clone().eq(&convert_to_string(parse_list(test_string))));
    }    
}