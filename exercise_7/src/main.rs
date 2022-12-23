// Exercise 7: creating a filesystem representation and finding the large directories

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


struct Folder {
    name : String,
    local_size : u32,
    subfolders : Vec::<Folder>,
    files : Vec::<(String, u32)>,
    parent: Option<Box<Folder>>,
}
impl Folder {

    fn new(input_name : String) -> Folder {
        Folder {
            name: input_name,
            local_size: 0,
            subfolders: Vec::<Folder>::new(),
            files: Vec::<(String, u32)>::new(),
            parent: None,
        }
    }

    fn get_size (&self) -> u32 {
        let mut total_size = self.local_size;
        for file in &self.files {
            total_size += file.1;
        }
        
        for subfolder in &self.subfolders {
            total_size += subfolder.get_size();
        }
        total_size
    }
}

// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to Advent of Code 2022 - Day 7!");

    // Handling the reading/parsing
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    // First reading the input string - easy.
    let mut commands_vec = Vec::<String>::new();
    // Finally reading the stuff.
    for curr_line in reader.lines() {
        if let Ok(line) = curr_line {
            commands_vec.push(line);
        }
    }
    println!("read {} lines from input", commands_vec.len());

    // For Part 1 - creating the folders representation 

    // Root folder
    let mut root_folder = Folder::new("Root".to_string());

    // Now we need a way to handle the "cursor" of the commands, assuming that the user moved around a lot.


    // End of main
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO tbr if not used.
    // #[test]
    // fn test_check_duplicates() {
    //     assert_eq!(check_no_duplicates_in_slice("aAbBcCdD".as_bytes()), true);
    // }
}