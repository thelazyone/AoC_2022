// Exercise 17: 

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum Directions {
    Left,
    Right,
}

#[derive(PartialEq)]
enum BlockType {
    Horizontal,
    Vertical,
    Cross,
    Square,
    El,
}

struct TetrisBlock {
    // Blocks are defined from the bottom-left corner.
    shape : Vec<Vec<bool>>,
    altitude : u32,
    pos : u32,
}
impl TetrisBlock {
    fn new(block_type : BlockType, altitude : u32) -> TetrisBlock {

        // Creating the vec based on the shape:
        let mut block = Vec::<Vec<bool>>::new();
        match block_type {
            BlockType::Horizontal => block.push(vec![true, true, true, true]),
            BlockType::Vertical => {
                block.push(vec![true]);
                block.push(vec![true]);
                block.push(vec![true]);
                block.push(vec![true]);},
            BlockType::Cross => {
                block.push(vec![false, true, false]);
                block.push(vec![true, true, true]);
                block.push(vec![false, true, false]);},
            BlockType::Square => {
                block.push(vec![true, true]);
                block.push(vec![true, true]);
                vec![[true, true], [true, true]];},
            BlockType::El => {
                block.push(vec![true, true, true]);
                block.push(vec![false, false, true]);
                block.push(vec![false, false, true]);},
        };
        TetrisBlock{shape : block, altitude : altitude, pos : 2}
    }

    fn move_block(&mut self, direction : Directions, board : &TetrisBoard) {
        if !self.collision_with_borders(board.get_width(), &direction) {
            match direction {
                Directions::Left => self.pos = self.pos - 1,
                Directions::Right => self.pos = self.pos + 1,
            };
        }
    }

    // Returning if collided with bottom
    fn fall_block(&mut self, board : &TetrisBoard) -> Option<Vec<Vec<bool>>> {

        // Check the line below the current one.
        for y_idx in 0..self.get_height() {

            if board.board.len() < (self.altitude + y_idx) as usize {
                continue;
            }

            if self.collision_with_line(&board.board[(self.pos - 1) as usize]) {

                // Sanity check: I expect the board to be as tall
                // as po
    
                // Creating the new board adding white lines if necessary.
                let mut new_board = board.board.clone();
                while new_board.len() < (self.altitude + self.get_height()) as usize {
                    new_board.push(vec![false; new_board[0].len()]);
                }

                for shape_line in 0..self.get_height() {
                    for x_idx in 0..self.get_width() {
                        new_board[(self.pos + shape_line) as usize][(x_idx + self.pos) as usize] =
                        new_board[(self.pos + shape_line) as usize][(x_idx + self.pos) as usize] || 
                        self.shape[shape_line as usize][x_idx as usize];
                    }
                }

                return Some(new_board);
            }
            else {
                self.pos -= 1;
            }
        }
        None
    }


    fn collision_with_borders(&self, board_width: u32, direction : &Directions) -> bool {
        (self.pos == 0 && direction == &Directions::Left) ||
        (self.pos >= board_width - self.get_width() && direction == &Directions::Left)
    } 

    
    fn collision_with_line(&self, line: &Vec<bool>) -> bool {
        // Sanity Check: 
        if line.len() < (self.pos + self.get_width()) as usize {
            panic!("Wrong inputs!");
        }
        
        // checking each line:
        for y_idx in 0..self.get_height(){
            for x_idx in 0..self.get_width() {
                if line[(y_idx + self.pos) as usize] && self.shape[0][x_idx as usize] {
                    return true;
                }
            }
        }
        false
    }


    // Dimensional values
    fn get_width(&self) -> u32 {
        self.shape[0].len() as u32
    }

    fn get_height(&self) -> u32 {
        self.shape.len() as u32
    }
}


struct TetrisBoard {
    board : Vec<Vec<bool>>
}
impl TetrisBoard {

    fn new(width : u32) -> TetrisBoard {
        let one_line = std::vec::from_elem(false, width as usize);
        TetrisBoard {board : std::vec::from_elem(one_line, 1)}
    }

    fn get_width(&self) -> u32 {
        self.board[0].len() as u32
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
    println!("read {} lines from input (as expected)", lines_vec.len());
    assert!(lines_vec.len() == 1);

    // Converting in left-right commands:
    let mut commands_vec = Vec::<Directions>::new();
    for character in lines_vec[0].chars() {
        match character {
            '>' => commands_vec.push(Directions::Right),
            '<' => commands_vec.push(Directions::Right),
            _ => panic!("Wrong input character!"),
        }
    }




    result_part_1 = 0;
    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 17!");

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