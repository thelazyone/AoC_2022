// Exercise 11: calculating the path of a cursor following the movement of another
// Implementing only part 2, because the optimization made part 1 unfeasible : the "/3"
// operation doesn't work well with the implemented structure.


// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use std::collections::HashMap;

// parsing the info about the monkey
struct Monkey {
    items_vec : Vec<RotatingCounter>,
    operation : Box<dyn Fn(&RotatingCounter) -> RotatingCounter>,
    throwing_rule : Box<dyn Fn(&RotatingCounter) -> i32>,
    inspect_counter : i32,
}


// Using a rotating index for all the prime numbers. 
#[derive(Clone)]
#[derive(Debug)]
struct RotatingCounter {
    counters : HashMap<i32, i32>,
}
impl RotatingCounter {

    fn new(input : i32) -> RotatingCounter {
        RotatingCounter {
            counters: HashMap::from([
                (2 , input % 2 ),
                (3 , input % 3 ),
                (5 , input % 5 ),
                (7 , input % 7 ),
                (11, input % 11),
                (13, input % 13),
                (17, input % 17),
                (19, input % 19),
                (23, input % 23)
            ])
        }
    }

    fn add(&self, input: i32) -> RotatingCounter {
        RotatingCounter{counters : self.counters.iter().map(move |(base, value)| {(*base, (value + input) % * base as i32)}).collect()}
    }

    // Multiplying means that residuals are multiplied with the input
    fn multiply(&self, input: i32) -> RotatingCounter {
        RotatingCounter{counters : self.counters.iter().map(move |(base, value)| {(*base, (value * input) % * base as i32)}).collect()}
    }

    // Squaring the value doesn't affect anything that is already multiple, but 
    // any residual is squared too.
    fn square(&self) -> RotatingCounter {
        RotatingCounter{counters : self.counters.iter().map(move |(base, value)| {(*base, (value * value) % * base as i32)}).collect()}
    }


    fn is_divisible_by_prime(&self, input: i32) -> bool {
        *self.counters.get(&input).unwrap() == 0
    }
}


fn cleanup_monkey_input (mut input_lines : Vec<String>) -> Vec<String> {
    // Cleaning up the info
    assert!(input_lines.len() == 6);
    input_lines.remove(0); // Removing the "Monkey X:" line

    // Clearing anything before the ": " part.
    for sub_line in  &mut input_lines {
        *sub_line = sub_line.split(": ").nth(1).unwrap().to_string().clone();
    }

    input_lines
}


// Reading the text input and parsing
fn parse_monkey(input_lines : Vec<String>) -> Monkey {

    // Converting into iter, reading line after line and consuming it
    let mut input_iter = input_lines.into_iter();

    // first line - items
    let mut items_vec = Vec::<RotatingCounter>::new();
    for item_value in input_iter.nth(0).unwrap().split(", ") {
        items_vec.push(RotatingCounter::new(item_value.parse::<i32>().unwrap().clone()));
    }

    // second line - operation. syntax is new = old # X, where # is an operation and X a value.
    // First retrieving all the elements separated by " " and extracting the two relevant ones.
    let operation_elements : Vec<String> = input_iter.nth(0).unwrap().clone().split(" ").map(str::to_string).collect();

    // Operator is the * or +, operation is the resulting closure. 
    let operator : Box<dyn Fn(&RotatingCounter, &i32)->RotatingCounter>;
    let operation : Box<dyn Fn(&RotatingCounter)->RotatingCounter>;
    match operation_elements[3].as_str() {
        "*" => operator = Box::new(|elem_a : &RotatingCounter, elem_b : &i32| -> RotatingCounter {elem_a.multiply(elem_b.clone())}),
        "+" => operator = Box::new(|elem_a : &RotatingCounter, elem_b : &i32| -> RotatingCounter {elem_a.add(elem_b.clone())}),
        _ => panic!("unexpected command!")
    }

    // The move command is absolutely vital, because this forces the moving of the variables captured
    // in the current environment INSIDE the closure.
    match operation_elements[4].as_str() {
        
        // Important note: I decided to AVOID the case of old + old (which would be the equivalent of * 2)
        // because i know that the input text doesn't have it and it would ruin the double-closure architecture
        // In that case a matches-in-the-match architecture would have made more sense.
        "old" => operation = Box::new(move |old : &RotatingCounter| { old.square() /*operator(old, old)*/ }),
        numeric_line => {
            let temp_value = numeric_line.parse::<i32>().unwrap().clone();
            operation = Box::new(move |old : &RotatingCounter| {operator(old, &temp_value) });
        },
    }

    // The throwing rule depends on line four.
    let division_factor = input_iter.nth(0).unwrap().split("divisible by ").nth(1).unwrap().parse::<i32>().unwrap().clone();
    let target_case_true = input_iter.nth(0).unwrap().split("to monkey ").nth(1).unwrap().parse::<i32>().unwrap().clone();
    let target_case_false = input_iter.nth(0).unwrap().split("to monkey ").nth(1).unwrap().parse::<i32>().unwrap().clone();
    let throwing_rule : Box<dyn Fn(&RotatingCounter)->i32>;
    throwing_rule = Box::new(move |value : &RotatingCounter| -> i32 {
        if value.is_divisible_by_prime(division_factor) {
            if division_factor == 19 {
            }
            return target_case_true;
        }
        //println!("elem is not divisible by {}", division_factor);
        target_case_false
    });


    Monkey {
        items_vec: items_vec,
        operation: operation,
        throwing_rule: throwing_rule,
        inspect_counter: 0, 
        }
}


// Primary Function
fn execute (input_path : String)  -> Option<(u64, u64)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : u64 = 0; // Part 1 is no longer accessible for this exercise. 
    let result_part_2 : u64;

    // First reading the input string - easy.
    let mut monkeys_lines_vec = Vec::<Vec<String>>::new();

    // Finally reading the stuff.
    let mut temp_monkey = Vec::<String>::new();
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            if line.is_empty() {

                // adding the monkey
                monkeys_lines_vec.push(cleanup_monkey_input(temp_monkey));
                temp_monkey = Vec::<String>::new();
            }
            else
            {
                temp_monkey.push(line);
            }
        }
    }    

    // Adding the last monkey too.
    monkeys_lines_vec.push(cleanup_monkey_input(temp_monkey));

    println!("\nread {} lines from input", monkeys_lines_vec.len());
    assert!(monkeys_lines_vec.len() > 1);

    // Creating the monkeys.
    let mut monkeys_vec = Vec::<Monkey>::new();
    for monkey_data in monkeys_lines_vec {
        let curr_monkey : Monkey = parse_monkey(monkey_data);
        monkeys_vec.push(curr_monkey);
    }

    // Part 1 is not working anymore, since the optimization for part 2 doesn't work with divisions.

    // // Each monkey then operates, 20 times:
    // for _ in 0..20 {

    //     // The right way to iterate here would be with the "for in" syntax, but 
    //     // since i'm modifying the vector inside the cycle i'll be using the index instead.
    //     for idx in 0..monkeys_vec.len() {

    //         // Monkeys inspect each item:
    //         monkeys_vec[idx].inspect_counter += monkeys_vec[idx].items_vec.len() as i32;
    //         for item in monkeys_vec[idx].items_vec.clone() {

    //             // Applies the operation and divide by 3
    //             // The "divide by 3" is impossible with the optimization done for Part 2. :( 
    //             let new_item_value = (monkeys_vec[idx].operation)(&item) /* /3 */;

    //             // depending on the value throws the item to another monkey.
    //             let target_index = (monkeys_vec[idx].throwing_rule)(&new_item_value) as usize;
    //             monkeys_vec[target_index].items_vec.push(new_item_value);
    //         }

    //         // All items has been thrown, clearing the list.
    //         monkeys_vec[idx].items_vec.clear();
    //     }
    // }

    // let mut items_inspected_vec = Vec::<i32>::new();
    // for idx in 0..monkeys_vec.len() {
    //     println!("\nMonkey {} inspected {} items", idx, monkeys_vec[idx].inspect_counter);
    //     items_inspected_vec.push(monkeys_vec[idx].inspect_counter);
    // }
    // items_inspected_vec.sort();
    // items_inspected_vec.reverse();
    // assert!(items_inspected_vec.len() >= 2);
    // result_part_1 = (items_inspected_vec[0]*items_inspected_vec[1]) as u64;


    // For part 2 it's all the same, only without the /3 and iterating 10.000 times.
    // This however poses a problem - numbers become huge, so i should track the prime
    // numbers involved!
    for _ in 0..10000 {
        // The right way to iterate here would be with the "for in" syntax, but 
        // since i'm modifying the vector inside the cycle i'll be using the index instead.
        for idx in 0..monkeys_vec.len() {

            // Monkeys inspect each item:
            monkeys_vec[idx].inspect_counter += monkeys_vec[idx].items_vec.len() as i32;
            for item in monkeys_vec[idx].items_vec.clone() {

                // Applies the operation.
                let new_item_value = (monkeys_vec[idx].operation)(&item);

                // depending on the value throws the item to another monkey.
                let target_index = (monkeys_vec[idx].throwing_rule)(&new_item_value) as usize;
                monkeys_vec[target_index].items_vec.push(new_item_value);
            }

            // All items has been thrown, clearing the list.
            monkeys_vec[idx].items_vec.clear();
        }
    }

    let mut items_inspected_vec = Vec::<i32>::new();
    for idx in 0..monkeys_vec.len() {
        println!("Monkey {} inspected {} items", idx, monkeys_vec[idx].inspect_counter);
        items_inspected_vec.push(monkeys_vec[idx].inspect_counter);
    }
    items_inspected_vec.sort();
    items_inspected_vec.reverse();
    assert!(items_inspected_vec.len() >= 2);
    println!("multiplying {} with {}", items_inspected_vec[0]  as u64, items_inspected_vec[1] as u64);
    result_part_2 = (items_inspected_vec[0] as u64) * (items_inspected_vec[1] as u64);

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 9!");

    let results = execute("./data/input.txt".to_string()).unwrap();
    // let results = execute("./data/test.txt".to_string()).unwrap();
    
    println!("\nPart 1 result is {}.", results.0);
    println!("\nPart 2 result is {}.", results.1);

    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // // General Test
    // #[test]
    // fn global_test_part_1() {
    //     assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 10605);
    // }    

    #[test]
    fn unit_test_rotating_counter_divisible_by() {
        let counter = RotatingCounter::new(65168454);
        assert_eq!(counter.is_divisible_by_prime(2), true);
        assert_eq!(counter.is_divisible_by_prime(3), true);
        assert_eq!(counter.is_divisible_by_prime(5), false);
        assert_eq!(counter.is_divisible_by_prime(7), false);
        assert_eq!(counter.is_divisible_by_prime(11), false);
        assert_eq!(counter.is_divisible_by_prime(13), true);
        assert_eq!(counter.is_divisible_by_prime(17), false);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
    }

    #[test]
    fn unit_test_rotating_counter_add() {
        let mut counter = RotatingCounter::new(350);
        assert_eq!(counter.is_divisible_by_prime(2), true);
        assert_eq!(counter.is_divisible_by_prime(3), false);
        assert_eq!(counter.is_divisible_by_prime(5), true);
        assert_eq!(counter.is_divisible_by_prime(7), true);
        assert_eq!(counter.is_divisible_by_prime(11), false);
        assert_eq!(counter.is_divisible_by_prime(13), false);
        assert_eq!(counter.is_divisible_by_prime(17), false);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
        counter = counter.add(35);
        assert_eq!(counter.is_divisible_by_prime(2), false);
        assert_eq!(counter.is_divisible_by_prime(3), false);
        assert_eq!(counter.is_divisible_by_prime(5), true);
        assert_eq!(counter.is_divisible_by_prime(7), true);
        assert_eq!(counter.is_divisible_by_prime(11), true);
        assert_eq!(counter.is_divisible_by_prime(13), false);
        assert_eq!(counter.is_divisible_by_prime(17), false);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
    }

    #[test]
    fn unit_test_rotating_counter_square() {
        let mut counter = RotatingCounter::new(3234);
        counter = counter.square(); //10458756
        assert_eq!(counter.is_divisible_by_prime(2), true);
        assert_eq!(counter.is_divisible_by_prime(3), true);
        assert_eq!(counter.is_divisible_by_prime(5), false);
        assert_eq!(counter.is_divisible_by_prime(7), true);
        assert_eq!(counter.is_divisible_by_prime(11), true);
        assert_eq!(counter.is_divisible_by_prime(13), false);
        assert_eq!(counter.is_divisible_by_prime(17), false);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
        counter = counter.add(35);
        assert_eq!(counter.is_divisible_by_prime(2), false);
        assert_eq!(counter.is_divisible_by_prime(3), false);
        assert_eq!(counter.is_divisible_by_prime(5), false);
        assert_eq!(counter.is_divisible_by_prime(7), true);
        assert_eq!(counter.is_divisible_by_prime(11), false);
        assert_eq!(counter.is_divisible_by_prime(13), false);
        assert_eq!(counter.is_divisible_by_prime(17), true);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
    }

    #[test]
    fn unit_test_rotating_counter_multiply() {
        let mut counter = RotatingCounter::new(3234);
        counter = counter.multiply(947); //3062598
        assert_eq!(counter.is_divisible_by_prime(2), true);
        assert_eq!(counter.is_divisible_by_prime(3), true);
        assert_eq!(counter.is_divisible_by_prime(5), false);
        assert_eq!(counter.is_divisible_by_prime(7), true);
        assert_eq!(counter.is_divisible_by_prime(11), true);
        assert_eq!(counter.is_divisible_by_prime(13), false);
        assert_eq!(counter.is_divisible_by_prime(17), false);
        assert_eq!(counter.is_divisible_by_prime(19), false);
        assert_eq!(counter.is_divisible_by_prime(23), false);
    }

     #[test]
     fn global_test_part_2() {
         assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 2713310158);
     }    
}