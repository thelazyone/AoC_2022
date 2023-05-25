// Exercise 22: Finding your way across a weird space-wrapped map!

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex;

// utility
#[derive(Clone, Debug)]
enum RotationDirection {
    Right,
    Left,
}

#[derive(Clone, Debug)]
enum CursorDirection {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
fn number_to_cursor_direction(number : u32) -> CursorDirection {
    match number {
        0=>CursorDirection::Right,
        1=>CursorDirection::Down,
        2=>CursorDirection::Left,
        3=>CursorDirection::Up,
        _=> panic!("only four directions exist!"),
    }
}

#[derive(Clone)]
enum MovementCommand {
    Advance(usize),
    Rotate(RotationDirection),
}

#[derive(PartialEq, Eq, Clone)]
enum WrappedBlock {
    Floor,
    Wall, 
    Skip
}

#[derive(Clone, Debug)]
struct WorldCursor{
    position : (usize, usize), // col, pos
    direction : CursorDirection
}



struct WrappedMap {
    world_map : Vec<Vec<WrappedBlock>>,
    movement_commands : Vec<MovementCommand>,
    cursor : Option<WorldCursor>
}

impl WrappedMap {
    fn new() -> WrappedMap {
        WrappedMap {
            world_map : Vec::new(),
            movement_commands : Vec::new(),
            cursor : None,
        }
    }

    fn _get_wrapped_block_char(block_type : &WrappedBlock) -> char {
        match block_type {
            WrappedBlock::Floor=>'.',
            WrappedBlock::Wall =>'#',
            WrappedBlock::Skip =>' ',
        }
    }

    // Parsing one line of the map.
    fn add_line(&mut self, input_line : &String) {
        self.world_map.push(input_line.chars().map(|character| {
            match character {
                '.'=>WrappedBlock::Floor,
                '#'=>WrappedBlock::Wall,
                ' '=>WrappedBlock::Skip,
                _ => panic!("Unexpected char!"),
            }
        }).collect());

        // If it's the first line, positioning the cursor.0
        if self.world_map.len() == 1 {
            let start_position = self.world_map[0].iter().position(|elem| elem == &WrappedBlock::Floor).unwrap();
            self.cursor = Some(WorldCursor { 
                position: (start_position, 0),
                direction: CursorDirection::Right});
        }

        // This is costly but it's the more general way to ensure that the matrix is well defined.
        self.pad_lines();
    }

    fn pad_lines(&mut self) {

        // Checking the max among the lines, and padding al the other lines.
        let max_length = self.world_map.iter()
            .max_by(|x, y| x.len().cmp(&y.len())).unwrap().len();

        for mut line in &mut self.world_map {
            if line.len() < max_length {
                line.extend(vec![WrappedBlock::Skip; max_length - line.len()]);
            }
        } 
    }

    // Parsing the last line of commands.
    fn set_movement_commands(&mut self, input_line : &String) {
        let re = regex::Regex::new(r"[0-9]+|[RL]").unwrap();
        self.movement_commands = re.find_iter(input_line).map(|mat| {
            match mat.as_str() {
                "R"=>MovementCommand::Rotate(RotationDirection::Right),
                "L"=>MovementCommand::Rotate(RotationDirection::Left),
                _ => {
                    if let Ok(num) = mat.as_str().parse::<i32>() {
                        MovementCommand::Advance(num as usize)
                    } else {
                        panic!("Unexpected command: {}", mat.as_str())
                    }
                },
            }
        }).collect();
    }

    fn get_block_at_position(&self, pos: &(usize, usize)) -> Option<WrappedBlock> {
        if self.world_map.is_empty(){
            return None;
        }
        if self.world_map[0].is_empty(){
            return None;
        }

        Some(self.world_map
            [(self.world_map.len() + pos.1) % self.world_map.len()]
            [(self.world_map[0].len() + pos.0) % self.world_map[0].len()].clone())
    }

    fn get_next_cursor_position(&self, cursor: &WorldCursor) -> (usize, usize) {
        let mut current_position = (cursor.position.0 as i32, cursor.position.1 as i32);
        loop {
            let mut new_position = match cursor.direction {
                CursorDirection::Down=>(current_position.0, current_position.1 + 1),
                CursorDirection::Up=>(current_position.0, current_position.1 - 1),
                CursorDirection::Right=>(current_position.0 + 1, current_position.1),
                CursorDirection::Left=>(current_position.0 - 1, current_position.1),
            };  

            // applying the module to both col and row
            new_position = (
                (self.world_map[0].len() as i32 + new_position.0) % self.world_map[0].len() as i32,
                (self.world_map.len() as i32 + new_position.1) % self.world_map.len() as i32,);

            let found_block = 
                self.get_block_at_position(&(new_position.0 as usize, new_position.1 as usize)).unwrap();
            if found_block != WrappedBlock::Skip{
                return (new_position.0 as usize, new_position.1 as usize);
            }

            // If it's a "skip", moving forward until found a proper block.
            current_position = new_position;
        }
    }

    // Handling the cursor movements
    fn move_cursor(&mut self, steps: usize) {
        let mut temp_cursor = self.cursor.as_ref().unwrap().clone();
        for _ in 0..steps {
            // Checking the next block in position
            let next_position = self.get_next_cursor_position(&temp_cursor);
            temp_cursor.position = match self.get_block_at_position(&next_position) {
                Some(WrappedBlock::Floor)=>{
                    next_position
                },
                Some(WrappedBlock::Wall)=>temp_cursor.position,//do nothing
                Some(WrappedBlock::Skip)=>panic!("Encountered a 'skip' block while moving!"),
                None=> panic!("found no block!"),
            }
        }

        self.cursor.as_mut().unwrap().direction = temp_cursor.direction;
        self.cursor.as_mut().unwrap().position = temp_cursor.position;
    }

    fn rotate_cursor(&mut self, direction: RotationDirection) {
        let current_direction = self.cursor.as_mut().unwrap().direction.clone() as u32;
        self.cursor.as_mut().unwrap().direction = match direction {
            RotationDirection::Right=>number_to_cursor_direction((current_direction + 1) % 4),
            RotationDirection::Left=>number_to_cursor_direction((current_direction + 3) % 4),
        }
    }

    fn apply_all_movements(&mut self) {
        for command in self.movement_commands.clone() {
            match command {
                MovementCommand::Advance(value)=>self.move_cursor(value.clone()),
                MovementCommand::Rotate(value)=>self.rotate_cursor(value.clone()),
            }
        }
    }

    // For Debug Only:
    fn _display_map(&self) {
        for line in self.world_map.iter() {
            for block in line {
                print!("{}", WrappedMap::_get_wrapped_block_char(block));
            }
            println!("");
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

    // Creating the wrapped map: 
    println!("Starting Part 1...");
    let mut world_map = WrappedMap::new();
    for line in lines_vec {
        if  line.len() > 0 {
            if !line.starts_with(' ') && !line.starts_with('.') && !line.starts_with('#')  {
                world_map.set_movement_commands(&line);
            }
            else {
                world_map.add_line(&line);
            }
        }
        else {
            continue;
        }
    }

    // For debug only:
    //world_map._display_map();

    println!("Applying movements...");
    world_map.apply_all_movements();
    let final_cursor = world_map.cursor.as_ref().unwrap().clone();
    result_part_1 = ((final_cursor.position.1 + 1) * 1000 +
         (final_cursor.position.0 + 1) * 4) as u32 + final_cursor.direction as u32;
    

    // For part 2 the only difference is how to apply the wrapping. 
    

    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 22!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 6032);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}