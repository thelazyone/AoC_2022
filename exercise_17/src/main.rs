// Exercise 17: 

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
#[derive(Debug)]
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

    fn move_block(&mut self, direction : &Directions, board : &TetrisBoard) {
        if !self.collision_with_borders(board.get_width(), &direction) {
            match direction {
                &Directions::Left => self.pos = self.pos - 1,
                &Directions::Right => self.pos = self.pos + 1,
            };
        }
    }

    // Returning if collided with bottom
    fn fall_block(&mut self, board : &TetrisBoard) -> Option<Vec<Vec<bool>>> {

        // Check the line below the current one.
        for y_idx in 0..self.get_height() {

            let line_to_inspect = self.altitude + y_idx;
            if board.board.len() <= line_to_inspect as usize {
                continue;
            }

            if self.altitude <= 0 || self.collision_with_map(board) {

                // Creating the new board adding white lines if necessary.
                let mut new_board = board.board.clone();
                while new_board.len() < (self.altitude + self.get_height() + 3) as usize {
                    new_board.push(vec![false; new_board[0].len()]);
                }

                for shape_line in 0..self.get_height() {
                    for x_idx in 0..self.get_width() {
                        new_board[(self.altitude + shape_line) as usize][(x_idx + self.pos) as usize] =
                        new_board[(self.altitude + shape_line) as usize][(x_idx + self.pos) as usize]
                         || 
                        self.shape[shape_line as usize][x_idx as usize];
                    }
                }

                return Some(new_board);
            }
        }

        self.altitude -= 1;
        None
    }


    fn collision_with_borders(&self, board_width: u32, direction : &Directions) -> bool {
        (self.pos <= 0 && direction == &Directions::Left) ||
        (self.pos >= board_width - self.get_width() && direction == &Directions::Right)
    } 

    
    fn collision_with_map(&self, board : &TetrisBoard) -> bool {

        // checking each line:
        for y_idx in 0..self.get_height(){
            let y_coord = y_idx + self.altitude - 1; // Checking the space below
           
            for x_idx in 0..self.get_width() {

                let x_coord = x_idx + self.pos;

                if board.board.len() <= y_coord as usize {
                    continue;
                }
                
                if board.board[y_coord as usize][x_coord as usize] && 
                   self.shape[y_idx as usize][x_idx as usize] {
                   return true;
                }
            }
        }
        false
    }

    // Converting a counter into a block type:
    fn block_type_from_num(number : u32) -> BlockType {
        match number % 5 {
            0 => BlockType::Horizontal,
            1 => BlockType::Cross,
            2 => BlockType::El,
            3 => BlockType::Vertical,
            4 => BlockType::Square,
            _ => panic!("impossible to get here"),
        }
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

    fn get_height(&self) -> u32 {
        for (index, line) in self.board.iter().enumerate() {
            if !line.contains(&true) {
                println!("height is {}", index);
                return index as u32;
            }
        }

        self.board.len() as u32
    }

    fn _draw_board(&self) -> String {
        let mut out_string = "".to_string();
        for row in &self.board {
            let new_string = row.iter().map(|&val| {
                match &val {
                    true => '#',
                    false => ' ',
                }
            }).collect::<String>().clone(); 
            out_string += &new_string;
            out_string += "\n";
        }
        out_string
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
            '<' => commands_vec.push(Directions::Left),
            _ => panic!("Wrong input character!"),
        }
    }

    // Creating the tetris board: 
    let mut tetris_board = TetrisBoard::new(7);
    //println!("Debug: \n{}", tetris_board._draw_board());

    // Iterating through the various turns:
    let max_rocks = 2022;
    let mut current_rocks = 0;
    let mut block_type_counter = 0;
    let mut time_counter = 0;
    while current_rocks <= max_rocks {
        // First adding a new stone
        let mut new_block = TetrisBlock::new(TetrisBlock::block_type_from_num(block_type_counter), tetris_board.get_height() + 3); 
        //println!("Adding new rock at {}", tetris_board.get_height() + 3);

        current_rocks += 1;
        block_type_counter += 1;

        // Looping until the block reaches the bottom.
        loop {

            // Moving the block if possible.
            if commands_vec.len() > time_counter {
                new_block.move_block(&commands_vec[time_counter], &tetris_board);
                //println!("moving {:?}", commands_vec[time_counter]);
            }

            // Making the block fall:
            println!("falling to {}", new_block.altitude);
            if let Some(new_map) = new_block.fall_block(&tetris_board) {
                tetris_board.board = new_map;
                //println!("Debug: \n{}", tetris_board._draw_board());
                break;
            }

            time_counter += 1;
        }

        if current_rocks == 6 {println!("Debug: \n{}", tetris_board._draw_board()); panic!("ahiA");}
        //println!("current height at rock {} is {}", current_rocks,tetris_board.get_height());
    }
    result_part_1 = tetris_board.get_height();

    result_part_2 = 0;
    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 17!");

    let results = execute("./data/test.txt".to_string()).unwrap();
    
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