// Exercise 10: Reading commands and executing operations on a register with clock.

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct RegisterCounter {
    cycle_counter : u32,
    cumulate_value : i32,
}

impl RegisterCounter {
    fn new() -> RegisterCounter {
        RegisterCounter{cycle_counter : 0, cumulate_value : 0}
    }

    // For Part 1:
    fn cumulate_step_40(&mut self, value : &i32) {
        // Increasing the counter
        self.cycle_counter += 1;

        // If one of the precious steps, cumulating.
        if self.cycle_counter >= 20 && (self.cycle_counter - 20) % 40 == 0 {
            self.cumulate_value += value * self.cycle_counter as i32;
            
            println!("Adding cumulated of {}*{}: sum is {}", value, self.cycle_counter, self.cumulate_value )
        }
    }
}

// For Part 2:
fn check_pixel(sprite_position : &i32, pixel_index : &u32) -> char {
    let line_index = *pixel_index % 40;
    if line_index as i32 >= *sprite_position && (line_index as i32) < (sprite_position + 3) {
        return '#';
    }
    '.'
}

// Primary Function
fn execute (input_path : String)  -> Option<(u32, String)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1;
    let mut result_part_2;

    // First reading the input string - easy.
    let mut lines_vec = Vec::<(String, Option<i32>)>::new();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            lines_vec.push((
                line.split(" ").next().unwrap().to_string(), 
                line.split(" ").nth(1).unwrap_or("").parse::<i32>().ok()));
        }
    }
    println!("read {} lines from input", lines_vec.len());
    assert!(lines_vec.len() > 1);

    // Depending on the input, the command either advances by 1 the clock or it atvances by 2 and changes the value.
    // Since the operation is quasi-dense, a brute force approach is probably just as convenient.
    let mut reg_value = 1;
    let mut register_counter = RegisterCounter::new(); 
    for line in &lines_vec {
        match line.0.as_str() {
            "addx" => {
                register_counter.cumulate_step_40(&reg_value);
                register_counter.cumulate_step_40(&reg_value);
                reg_value += line.1.unwrap();
            },
            "noop" => register_counter.cumulate_step_40(&reg_value),
            _ => panic!("wrong line command!"),
        }
    }
    println!("After the commands the value is {}", register_counter.cumulate_value);
    result_part_1 = register_counter.cumulate_value.try_into().unwrap();

    // For Part 2, things are a bit more complicated. The commands above now move a 3 pixels wide sprite
    // and each cycle it checks the position of the sprite compared to the position of a pixel.
    let mut out_screen = Vec::<char>::new();
    let mut sprite_position : i32 = 1; // sprite is 3 wide, counting the left-most position.
    let mut screen_cursor = 1;
    for line in &lines_vec {
        match line.0.as_str() {
            "addx" => {
                // Adding two values to the resulting screen
                out_screen.push(check_pixel(&sprite_position, &(screen_cursor as u32)));
                screen_cursor += 1;
                out_screen.push(check_pixel(&sprite_position, &(screen_cursor as u32)));
                screen_cursor += 1;

                // Updating the position (after the two iterations)
                sprite_position = sprite_position as i32 + line.1.unwrap();
            },
            "noop" => {
                // Just one more value, the sprite didn't move.
                out_screen.push(check_pixel(&sprite_position, &(screen_cursor as u32)));
                screen_cursor += 1;
            },
            _ => panic!("wrong line command!"),
        }
    }

    // Plotting result: 
    result_part_2 = "\n".to_string();
    for line_idx in 0..6{
        let slice = &out_screen[line_idx*40..(line_idx + 1)*40];
        let substring = String::from_iter(slice);
        result_part_2 += &(format!("{}\n", substring));
    }

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 10!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 13140);
    }    

    // Skipping the second test since it's just visual
    // #[test]
    // fn global_test_part_2() {
    //     //assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    // }    
}