// Exercise 2: Given a Rock Paper Scissor "strategy guide" calculate the points.
// 1 for Rock
// 2 for Paper
// 3 for Scissor
// 0 for losing
// 3 for draw
// 6 for winning 

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum RockPaperScissors {
    R = 0,
    P = 1,
    S = 2,
}
impl RockPaperScissors {
    fn from_i32(value: i32) -> RockPaperScissors {
        match value {
            0 => RockPaperScissors::R,
            1 => RockPaperScissors::P,
            2 => RockPaperScissors::S,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

fn get_score(input: RockPaperScissors, output: RockPaperScissors) -> i32{

    // Calculating the "played hand" value:
    let mut score = 0;
    score += (output as i32) + 1;

    // To confront hands, it wins if output = (input + 1) % 3
    if input == output {
        score += 3;
    }
    else if output as i32 == ((input as i32) + 1) % 3 {
        score += 6;
    }

    score
}

fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 2!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // Reading in two vectors, then using the "zip" functionality to work along them
    let mut strategy_vec = Vec::<(RockPaperScissors, RockPaperScissors)>::new();
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {

            // Parsing input
            let input : RockPaperScissors;
            let mut couple = line.split(" ");
            match couple.next().unwrap() {
                "A" => input = RockPaperScissors::R,
                "B" => input = RockPaperScissors::P,
                "C" => input = RockPaperScissors::S,
                _ => panic!("Wrong input character!"),
            }

            // Parsing Output
            let output : RockPaperScissors;
            match couple.next().unwrap() {
                "X" => output = RockPaperScissors::R,
                "Y" => output = RockPaperScissors::P,
                "Z" => output = RockPaperScissors::S,
                _ => panic!("Wrong input character!"),
            }
        
            strategy_vec.push((input, output));
        }
    }

    // Calculating the total values:
    let mut total_score = 0;
    for element in &strategy_vec {
        total_score += get_score(element.0, element.1);
    }

    // Solution of PART 1
    println!("The maximum score of the {} moves would be {}.", strategy_vec.len(), total_score);

    // For PART 2 i have to reinterpret the second part of the vector:
    // R for lose
    // P for draw
    // S for win
    let mut total_score = 0;
    for element in &strategy_vec {
        // To get the element that makes you lose, you do -1 mod 3. Same to win (+1 mod 3).
        // The offset is basically the int value of the enum -1 (or +2 since it's mod).
        let new_move = ((element.0 as i32) + ((element.1 as i32) + 2)) % 3;
        total_score += get_score(element.0, RockPaperScissors::from_i32(new_move));
    }

    println!("The score for Part 2 would be {}.", total_score);

    // End of main
    Ok(())
}
