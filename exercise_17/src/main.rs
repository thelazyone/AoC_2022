// Exercise 17: 

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
#[derive(Debug)]
enum Directions {
    Left,
    Right,
    Down,
}

#[derive(PartialEq)]
enum BlockType {
    Horizontal,
    Vertical,
    Cross,
    Square,
    El,
}

#[derive(Clone)]
struct TetrisBlock {
    // Blocks are defined from the bottom-left corner.
    shape : Vec<Vec<bool>>,
    altitude : u32,
    x_pos : u32,
}
impl TetrisBlock {
    fn new(block_type : BlockType, altitude : u32, pos : u32) -> TetrisBlock {
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
        TetrisBlock{shape : block, altitude : altitude, x_pos : pos}
    }


    fn try_move_block(&mut self, direction : &Directions, board : &TetrisBoard) -> Option<Vec<Vec<bool>>> {
        
        if self.collision_with_borders(board.get_width(), &direction) ||
            self.collision_with_map(board, direction){
                return Some(self.add_block_to_map(board));
            }

        self.move_block(direction);
        None
    }


    fn move_block(&mut self, direction : &Directions) {
        match direction {
            &Directions::Left => self.x_pos -= 1,
            &Directions::Right => self.x_pos += 1,
            &Directions::Down => self.altitude -= 1,
        };
    }


    fn add_block_to_map (&self, board : &TetrisBoard) -> Vec<Vec<bool>> {

        // Creating the new board adding white lines if necessary.
        let mut new_board = board.board.clone();
        while new_board.len() < (self.altitude + self.get_height() + 3) as usize {
            new_board.push(vec![false; new_board[0].len()]);
        }

        for shape_line in 0..self.get_height() {
            for x_idx in 0..self.get_width() {
                new_board[(self.altitude + shape_line) as usize][(x_idx + self.x_pos) as usize] =
                new_board[(self.altitude + shape_line) as usize][(x_idx + self.x_pos) as usize] || 
                self.shape[shape_line as usize][x_idx as usize];
            }
        }

        new_board
    }


    fn collision_with_borders(&self, board_width: u32, direction : &Directions) -> bool {
        (self.x_pos <= 0 && direction == &Directions::Left) ||
        (self.x_pos >= board_width - self.get_width() && direction == &Directions::Right) || 
        (self.altitude == 0 && direction == &Directions::Down)
    } 

    
    fn collision_with_map(&self, board : &TetrisBoard, direction : &Directions) -> bool {

        // Creating a copy of the block and moving it.
        let mut block_copy : TetrisBlock = self.clone();
        block_copy.move_block(direction);

        // checking each line:
        for y_idx in 0..block_copy.get_height(){
            let y_map = y_idx + block_copy.altitude; // Checking the row below

            if y_map >= board.board.len() as u32 {
                continue;
            }

            for x_idx in 0..block_copy.get_width() {

                let x_map = x_idx + block_copy.x_pos;

                if board.board[y_map as usize][x_map as usize] && 
                    block_copy.shape[y_idx as usize][x_idx as usize] {
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
    board : Vec<Vec<bool>>,
    time_counter : u32,
    type_counter : u32,
}
impl TetrisBoard {
    fn new(width : u32) -> TetrisBoard {
        let one_line = std::vec::from_elem(false, width as usize);
        TetrisBoard {board : std::vec::from_elem(one_line, 1), time_counter : 0, type_counter : 0}
    }


    fn get_width(&self) -> u32 {
        self.board[0].len() as u32
    } 


    fn get_height(&self) -> u32 {
        for (index, line) in self.board.iter().enumerate() {
            if !line.contains(&true) {
                return index as u32;
            }
        }

        self.board.len() as u32
    }


    fn draw_top_rows(&self, lines_number : usize) -> String {        
        let mut out_string = "".to_string();
        for row_idx in 0..lines_number {
            out_string += &self.draw_row(&self.board[self.board.len() - 1 - lines_number + row_idx]);
            out_string += "\n";
        }
        out_string
    }


    fn draw_row(&self, row : &Vec<bool>) -> String {
        row.iter().map(|&val| {
            match &val {
                true => '#',
                false => ' ',
            }
        }).collect::<String>().clone()
    }


    fn _draw_board(&self) -> String {
        let mut out_string = "".to_string();
        for row in &self.board {
            out_string += &self.draw_row(row);
            out_string += "\n";
        }
        out_string
    }


    fn add_block_till_bottom(&mut self, commands_vec : &Vec<Directions>) {
        let mut new_block = TetrisBlock::new(
            TetrisBlock::block_type_from_num(self.type_counter), 
            self.get_height() + 3,
            2 /* Always 2 */); 

        self.type_counter += 1;
            
        // Looping until the block reaches the bottom.
        loop {

            // Moving the block if possible.
            new_block.try_move_block(&commands_vec[self.time_counter as usize % commands_vec.len()], self);

            self.time_counter += 1;

            // Making the block fall:
            if let Some(new_map) = new_block.try_move_block(&Directions::Down, self) {
                self.board = new_map;
                break;
            }
        }
    }
}




// Primary Function
fn execute (input_path : String)  -> Option<(u32, u64)> {

    // Handling the reading/parsing
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    // Results variables:
    let result_part_1 : u32;
    let result_part_2 : u64;

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
    println!("There are {} directional commands", commands_vec.len());

    // Creating the tetris board: 
    let mut tetris_board = TetrisBoard::new(7);

    // Iterating through the various turns:
    let max_rocks = 2022;
    for _ in 0..max_rocks {
        // First adding a new stone
        tetris_board.add_block_till_bottom(&commands_vec);
    }
    result_part_1 = tetris_board.get_height();

    // For part 2 I am expected to iterate 1E12 times, which doesn't sound very feasible.
    // I'd instead search for a periodicity of the input values, and when found just multiply 
    // until necessary.
    let mut tetris_board = TetrisBoard::new(7);
    let mut remaining_rocks : u64 = 1000000000000;
    println!("Looping in search of periodicity.");

    // First applying a bunch of stones (1000), to make sure that the bottom is distant enough.
    let compare_start_time: u32;
    let start_rocks = 1000;
    for _ in 0..start_rocks {
        // First adding a new stone
        tetris_board.add_block_till_bottom(&commands_vec);
    }
    compare_start_time = tetris_board.time_counter.clone();
    remaining_rocks -= 1000;

    // Retrieving the last, say, 10 lines. it's not a guarantee that the periodicity is kept but it's safe enough.
    let start_pattern = tetris_board.draw_top_rows(10); 
    let start_height = tetris_board.get_height(); 
    let mut total_height : u64 = 0; 

    // Now applying stones until we reach a number of time iterations that is multiple of the instructions. Then, comparing the patterns.
    for loop_count in 1..100000 {
        // First adding a new stone
        tetris_board.add_block_till_bottom(&commands_vec);
        remaining_rocks -= 1;

        // Checking if the time is multiple: 
        if (tetris_board.time_counter - compare_start_time) as usize % commands_vec.len() == 0 {
            if start_pattern == tetris_board.draw_top_rows(10) {
                println!("found repeating pattern at loop {}.", loop_count);

                // This means that every loop_count iterations the pattern repeats.
                let delta_height = tetris_board.get_height() - start_height;

                // The remaining rocks number is divided in periodic steps, and the delta height is added each time.
                let number_of_periods = remaining_rocks / loop_count;
                remaining_rocks -= number_of_periods * loop_count;
                let multi_period_height = delta_height as u64 * number_of_periods;
                println!("There are {} periods.", number_of_periods);

                // Now iterating for the remaining rocks.
                for _ in 0..remaining_rocks {
                    tetris_board.add_block_till_bottom(&commands_vec);
                } 

                // height is this plus the periodic above. 
                total_height = tetris_board.get_height() as u64 + multi_period_height;

                // Exiting the loop.
                break;
            }
        }
    }

    result_part_2 = total_height;
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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 3068);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 1514285714288);
    }    
}