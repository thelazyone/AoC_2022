// Exercise 22: Finding your way across a weird space-wrapped map!

// For reading/parsing
use std::fs::File;
use std::hash::Hash;
use std::io::{self, prelude::*, BufReader};
use std::cmp;
use regex;
use std::collections::HashMap;

// utility
#[derive(Clone, Debug)]
enum RotationDirection {
    Right,
    Left,
    Flip,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Clone)]
enum WrapMode {
    Flat, // The wrapping applies on the same line OR column 
    Cube, // the wrapping implies that the map is composed of six areas.
}

type SeamMap = HashMap<(usize, usize, CursorDirection), ((usize, usize), Option<RotationDirection>)>;

// Creating two iterators from the two found points.
struct EdgeElement<'a> {
    current_pos : (usize, usize),
    prev_pos : (usize, usize),
    start_pos : (usize, usize),
    direction : CursorDirection,
    wrapped_map : &'a WrappedMap<'a>,
}

impl<'a> Iterator for EdgeElement<'a> {
    type Item = EdgeElement<'a>;
    fn next(&mut self) -> Option<EdgeElement<'a>> {

        // Finding all the nearby elements, checking which ones could be edges
        let neighbours = self.wrapped_map.get_neighbours(self.current_pos);

        let mut next_element_position = None;
        for neighbour_position in neighbours {
            let skip_elements = self.wrapped_map.get_number_of_neighbours_of_type(
                neighbour_position, WrappedBlock::Skip, true);
            if neighbour_position != self.current_pos &&
                neighbour_position != self.prev_pos &&
                neighbour_position != self.start_pos &&
                self.wrapped_map.get_block_at_position(&neighbour_position).unwrap() != WrappedBlock::Skip &&
                skip_elements > 0 {
                
                // The next block has been identified: however, before considering wether or not it is the next element
                // there are two special cases: 
                // 
                // 1 : Concave seam
                // 
                // #####
                // ####X####
                // #########
                // In this case, the element is skipped and we search directly the next one. 
                //
                // 2 : Convex Seam
                //      a
                // #####Xb
                // ######
                // In this case the element is added twice, first facing A then facing B.
                if skip_elements == 1 /* Case 1 */ {
                    //
                }
                else if skip_elements == 5 /* Case 2 */ {
                    //
                }
                else {
                    if next_element_position.is_none() {
                        next_element_position = Some(neighbour_position);
                    }
                    else {
                        panic!("more than one element found as possible new element!");
                    }
                }
            }
        }

        if let Some(position) = next_element_position {

            // TODO fix direction!
            let next_element = EdgeElement {
                current_pos : position,
                prev_pos : self.current_pos,
                start_pos : self.start_pos,
                direction : self.wrapped_map.get_direction_between_points(self.current_pos, position).unwrap(),
                wrapped_map : self.wrapped_map,
            };

            self.prev_pos = self.current_pos;
            self.current_pos = next_element.current_pos.clone();
            self.direction = next_element.direction.clone();

            return Some(next_element);
        }

        None
    }

}




struct WrappedMap<'a> {
    world_map : Vec<Vec<WrappedBlock>>,
    movement_commands : Vec<MovementCommand>,
    cursor : Option<WorldCursor>,
    seam_map : &'a Option<SeamMap>,
}
impl<'a> WrappedMap<'a> {
    fn new() -> WrappedMap<'a> {
        WrappedMap {
            world_map : Vec::new(),
            movement_commands : Vec::new(),
            cursor : None,
            seam_map : &None,
        }
    }

    fn _get_wrapped_block_char(block_type : &WrappedBlock) -> char {
        match block_type {
            WrappedBlock::Floor=>'.',
            WrappedBlock::Wall =>'#',
            WrappedBlock::Skip =>' ',
        }
    }

    // Setting the cursor according to the rules of the exercise.
    fn reset_cursor(&mut self){
        let start_position = self.world_map[0].iter().position(|elem| elem == &WrappedBlock::Floor).unwrap();
        self.cursor = Some(WorldCursor { 
            position: (start_position, 0),
            direction: CursorDirection::Right});
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
            self.reset_cursor();
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

    // For part 2 it is necessary to find the seams between the different folding parts of the cube.

    fn get_map_size(&self) -> (usize, usize) {
        let row_number = self.world_map.len();

        if row_number == 0 {
            return (0,0);
        }

        let col_number = self.world_map[0].len();
        return (col_number, row_number);
    }

    fn border_count(&self, position: (usize, usize)) -> usize {
        let (col_number, row_number) = self.get_map_size();
        let mut counter = 0;
        for row_idx in cmp::max(1, position.1) - 1.. cmp::min(position.1 + 2, row_number) {
            for col_idx in cmp::max(1, position.0) - 1 .. cmp::min(position.0 + 2, col_number)  {
                if position == (position.0, row_idx) {
                    continue;
                }
                
                if self.get_block_at_position(&(col_idx, row_idx)).unwrap() == WrappedBlock::Skip {
                    counter += 1
                } 
            }
        }

        0
    }

    // Counting the neighbours of a certain kind: 
    fn get_neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        
        let (col_number, row_number) = self.get_map_size();
        let mut neighbours_vec = Vec::<(usize, usize)>::new();

        for row_idx in cmp::max(1, position.1) - 1.. cmp::min(position.1 + 2, row_number) {
            if position == (position.0, row_idx) {
                continue;
            }
            neighbours_vec.push((position.0, row_idx));
        }

        for col_idx in cmp::max(1, position.0) - 1 .. cmp::min(position.0 + 2, col_number)  {
            if position == (col_idx, position.1) {
                continue;
            }
            neighbours_vec.push((col_idx, position.1));
        }

        neighbours_vec
    }


    fn get_number_of_neighbours_of_type(&self, position: (usize, usize), block_type: WrappedBlock, use_diagonals: bool) -> usize {

        let (col_number, row_number) = self.get_map_size();

        // Searching in the world map.
        let mut counter = 0;
        let mut neighbours_vec;
        if !use_diagonals {
            neighbours_vec = self.get_neighbours(position);
        }
        else {
            neighbours_vec = Vec::<(usize, usize)>::new();
            for row_idx in (cmp::max(1, position.1) - 1).. cmp::min(position.1 + 2, row_number) {
                for col_idx in (cmp::max(1, position.0) - 1).. cmp::min(position.0 + 2, col_number)  {
                    if position == (col_idx, row_idx) {
                        continue;
                    }
                    
                    if self.get_block_at_position(&(col_idx, row_idx)).unwrap() == block_type {
                        neighbours_vec.push((col_idx, row_idx));
                    } 
                }
            }
        }

        for element_pos in neighbours_vec {
            if self.world_map[element_pos.1][element_pos.0] == block_type {
                counter += 1;
            }
        }

        // In case of edges, i'm assuming all the values there count as "skip"
        if block_type == WrappedBlock::Skip {
            let mut edge_counter = 0;
            if position.0 == 0 || position.0 + 1 == col_number {
                edge_counter += 1;
            }
            if position.1 == 0 || position.1 + 1 == row_number {
                edge_counter += 1;
            }

            // If it's touching two edges the total number of extra "skip" is 5, not 6.
            if !use_diagonals {
                counter += edge_counter;
            } else {
                counter += match edge_counter {
                    1 => 3,
                    2 => 5,
                    _ => 0,
                }  
            }
        }

        counter
    }

    fn get_concave_seam_start(&self) -> Option<(usize, usize)> {

        let (col_number, row_number) = self.get_map_size();

        // This is to be done only once, therefore a brute force is reasonable.
        for row_idx in 0..row_number {
            for col_idx in 0..col_number {
                let skip_edges = self.get_number_of_neighbours_of_type((col_idx, row_idx), WrappedBlock::Skip, true);
                if self.get_number_of_neighbours_of_type((col_idx, row_idx), WrappedBlock::Skip, true) == 1 {
                    return Some((col_idx, row_idx));
                }
            }
        }

        None
    }

    fn get_direction_between_points(&self, from : (usize, usize), to : (usize, usize)) -> Option<CursorDirection> {

        if from == to {
            return None
        }

        // Checking if the two are adjacent.
        if from.0 == to.0 {
            if to.1 == from.1 + 1 {
                return Some(CursorDirection::Down);
            }
            if to.1 + 1 == from.1 {
                return Some(CursorDirection::Up);
            }
        }

        if from.1 == to.1 {
            if to.0 == from.0 + 1 {
                return Some(CursorDirection::Right);
            }
            if to.0 + 1 == from.0 {
                return Some(CursorDirection::Left);
            }
        }

        None
    }

    fn get_edges_seams(&mut self) -> HashMap<(usize, usize, CursorDirection), ((usize, usize), Option<RotationDirection>)>{

        // the seams map tracks for each point on the edges:
        // 1 - which point it would end up to
        // 2 - which rotation (if any) will be applied.
        let mut seam_map = HashMap::<(usize, usize, CursorDirection), ((usize, usize), Option<RotationDirection>)>::new();

        // Starting by finding one walkable point on the map that has a CONCAVE corner.
        // Something like X: 
        //
        // #...#.#..
        // .....#..X
        // ..#....#.#....
        // 
        // And then moving with two generic cursors in the two directions: 
        // By associating one with the other, and checking the different direction
        // of the "end" of the map, we can create the seams map.
        let seam_start_position = self.get_concave_seam_start().unwrap();
        println!("seam is at {:?}", seam_start_position);

        // Finding the neighbours. two of them should always be on the edge of the shape. 
        // This assumes 90 degrees angles, if there are "peninsulas" this approach does not work.
        let start_neighbours = self.get_neighbours(seam_start_position);
        let mut start_iterators = Vec::<EdgeElement>::new();
        for element_position in start_neighbours {
            if self.get_block_at_position(&element_position).unwrap() != WrappedBlock::Skip &&
                self.get_number_of_neighbours_of_type(element_position, WrappedBlock::Skip, false) > 0 {

                start_iterators.push(EdgeElement{
                    current_pos : element_position.clone(),
                    prev_pos : seam_start_position.clone(), 
                    start_pos : seam_start_position.clone(),
                    direction : self.get_direction_between_points(seam_start_position.clone(), element_position.clone()).unwrap(),
                    wrapped_map: self}
                );
            }
        }
        
        if start_iterators.len() != 2 {
            panic!("Expecting two start iterators, found {}", start_iterators.len());
        }
        // Cycling on both iterators. I'm expecting the same amount of elements.
        while let (Some(item1), Some(item2)) = (start_iterators[0].next(), start_iterators[1].next()) {
            seam_map.insert(
                (item1.current_pos.0, item1.current_pos.1, CursorDirection::Down.clone() /* TODO */), 
                (item2.current_pos, None)
            );
        }

        println!("seam_map is {:?}", seam_map);

        seam_map
    }


    fn get_position_from_direction(&self, position: (i32, i32), direction: &CursorDirection) -> (i32, i32) {
        match direction {
            CursorDirection::Down=>(position.0, position.1 + 1),
            CursorDirection::Up=>(position.0, position.1 - 1),
            CursorDirection::Right=>(position.0 + 1, position.1),
            CursorDirection::Left=>(position.0 - 1, position.1),
        } 
    }


    fn get_next_cursor_position(&self, cursor: &WorldCursor, wrap_mode: &WrapMode) -> (usize, usize) {
        let mut current_position = (cursor.position.0 as i32, cursor.position.1 as i32);

        // If flat wrap, iterating on the skips until the new one is found.
        // I could re-write this with the logic of Part 2, which is more
        // general but also quite more complicated.
        if wrap_mode == &WrapMode::Flat{
            loop {
                let mut new_position = 
                    self.get_position_from_direction(current_position, &cursor.direction);
    
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

        // If wrap mode is cube, using the seams map to find out where the cursor
        // ends up, if necessary.
        else {
            let mut new_position = 
                self.get_position_from_direction(current_position, &cursor.direction);

            // If the new position is not a "skip" position, proceeding as normal.
            let found_block = 
                self.get_block_at_position(&(new_position.0 as usize, new_position.1 as usize)).unwrap();
            if found_block != WrappedBlock::Skip{
                return (new_position.0 as usize, new_position.1 as usize);
            }

            // If the position is a Skip, then searching in the seams map.
            // match seams_map.get((current_position.0 as usize, current_position.1 as usize, &cursor.direction)) {
            //     Some(value) => {
            //         // If the key exists, do something with the value.
            //         println!("Value for {} is {}", key, value);
            //     },
            //     None => {
            //         // If the key does not exist, panic with a message.
            //         panic!("Key {} does not exist in the map!", key);
            //     },
            // }


            return (0,0);
        }
    }

    // Handling the cursor movements
    fn move_cursor(&mut self, steps: usize, wrap_mode: &WrapMode) {
        let mut temp_cursor = self.cursor.as_ref().unwrap().clone();
        for _ in 0..steps {
            // Checking the next block in position
            let next_position = self.get_next_cursor_position(&temp_cursor, wrap_mode);

            // Depending on what is found, behaving differently.
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
            RotationDirection::Flip=>number_to_cursor_direction((current_direction + 2) % 4),
        }
    }

    fn apply_all_movements(&mut self, wrap_mode: WrapMode) {

        // Retrieving the seam map, then applying the movements:
        let seam_map = self.get_edges_seams();

        for command in self.movement_commands.clone() {
            match command {
                MovementCommand::Advance(value)=>self.move_cursor(value.clone(), &wrap_mode),
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
    world_map.apply_all_movements(WrapMode::Flat);
    let final_cursor: WorldCursor = world_map.cursor.as_ref().unwrap().clone();
    result_part_1 = ((final_cursor.position.1 + 1) * 1000 +
         (final_cursor.position.0 + 1) * 4) as u32 + final_cursor.direction as u32;
    

    // For part 2 the only difference is how to apply the wrapping. 
    world_map.reset_cursor();
    world_map.apply_all_movements(WrapMode::Cube);
    let final_cursor = world_map.cursor.as_ref().unwrap().clone();
    result_part_2 = ((final_cursor.position.1 + 1) * 1000 +
         (final_cursor.position.0 + 1) * 4) as u32 + final_cursor.direction as u32;

    Some((result_part_1, result_part_2))
}

// Main 
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 22!");

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
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().0, 6032);
    }    

    #[test]
    fn global_test_part_2() {
        assert_eq!(execute("./data/test.txt".to_string()).unwrap().1, 8);
    }    
}