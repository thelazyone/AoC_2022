// Exercise 7: creating a filesystem representation and finding the large directories

// For reading/parsing
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// For handles
use std::rc::Rc;
use std::cell::RefCell;

// Maps are useful
use std::collections::HashMap;

// Folder structure!
#[derive(Debug)]
struct Folder {
    subfolders : HashMap::<String, Rc<RefCell<Folder>>>,
    files : HashMap::<String, u32>,
    parent: Option<Rc<RefCell<Folder>>>,
}
impl Folder {

    fn new() -> Folder {
        Folder {
            subfolders:  HashMap::<String, Rc<RefCell<Folder>>>::new(),
            files: HashMap::<String, u32>::new(),
            parent: None,
        }
    }

    fn get_size (&self) -> u32 {
        let mut total_size = 0;
        for file in &self.files {
            total_size += file.1;
        }
        
        for (_, subfolder) in &self.subfolders {
            total_size += subfolder.borrow_mut().get_size();
        }
        total_size
    }
}


// Parsing the command line: returning an enum 
#[derive(Debug)]
enum LineCommands {
    GoToRoot,
    GoToParent,
    GoToFolder(String),
    AddFolder(String),
    AddFile((String, u32)),
    Ignore,
}
fn get_line_command(input_line : String) -> LineCommands {

    // matching the start of the string:
    match input_line.as_str() {
        "$ cd /" => return LineCommands::GoToRoot,
        "$ cd .." => return LineCommands::GoToParent,
        line if line.contains("$ cd ") => {
            return LineCommands::GoToFolder(input_line.strip_prefix("$ cd ").unwrap().to_string())
        },
        line if line.contains("dir ") => {
            return LineCommands::AddFolder(input_line.strip_prefix("dir ").unwrap().to_string())
        },
        line if line.split(' ').next().is_some() && line.split(' ').next().unwrap().parse::<i32>().is_ok() => {
            return LineCommands::AddFile((line.split(' ').nth(1).unwrap().to_string(), line.split(' ').nth(0).unwrap().parse::<u32>().unwrap()))
        },
        "$ ls" => return LineCommands::Ignore,
        _ => panic!("Command not recognized: '{}'", input_line),
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
    let root_folder = Folder::new();
    let root_folder_cursor = Rc::<RefCell<Folder>>::new(RefCell::<Folder>::new(root_folder));
    let mut current_cursor = root_folder_cursor.clone();

    // Vector of all folders whatsoever.
    let mut folders_vector = Vec::<Rc<RefCell<Folder>>>::new();
    folders_vector.push(root_folder_cursor.clone());
    
    // Now we need a way to handle the "cursor" of the commands, assuming that the user moved around a lot.
    for command in commands_vec{
        let enum_command = get_line_command(command);
        match enum_command {
            LineCommands::GoToRoot => {
                current_cursor = root_folder_cursor.clone();
            },
            LineCommands::GoToParent => {
                let curr_parent = current_cursor.borrow_mut().parent.clone();
                current_cursor = curr_parent.unwrap().clone();
            },
            LineCommands::GoToFolder(folder_name) => {
                let support_cursor = current_cursor.borrow().subfolders.get(&folder_name).expect("No subfolder found.").clone();
                current_cursor = support_cursor;
            },
            LineCommands::AddFolder(folder_name) => {
                // First check if it exists already:
                if !current_cursor.borrow().subfolders.contains_key(&folder_name)
                {
                    let mut support_new_folder = Folder::new();
                    support_new_folder.parent = Some(current_cursor.clone());
                    let new_refcell = Rc::<RefCell<Folder>>::new(RefCell::<Folder>::new(support_new_folder));
                    current_cursor.borrow_mut().subfolders.insert(folder_name.clone(), new_refcell.clone());
                    
                    // Pushing in the vector, to retrieve later:
                    folders_vector.push(new_refcell);
                }
            },
            LineCommands::AddFile((file_name, file_size)) => {
                current_cursor.borrow_mut().files.insert(file_name.clone(), file_size.clone());
            },
            LineCommands::Ignore => {},
        }
    }

    // For Part 1 i must add up the sum of sizes of all folders lesser or equal to 100.000
    println!("Total size of root is: {}. folders vector size is {}", 
        root_folder_cursor.borrow().get_size(),
        folders_vector.len());
    let mut total_small_folders_sum = 0;
    for folder in &folders_vector {
        let folder_size = folder.borrow().get_size();
        if folder_size <= 100000 {
            total_small_folders_sum += folder_size;
        }
    }
    println!("The result of Part 1 is {}.", total_small_folders_sum);

    // For part 2 I must find the smallest folder greater or equal to 8381165.
    // I'm gonna go with a blunt approach.
    let total_space = folders_vector[0].borrow().get_size();
    let free_space = 70000000 - total_space;
    let required_space_to_free = 30000000 - free_space;
    let mut chosen_folder_size = total_space;
    println!("Required free space is {}", required_space_to_free);

    for folder in &folders_vector {
        let folder_size = folder.borrow().get_size();
        if folder_size >= required_space_to_free && folder_size < chosen_folder_size
        {
            chosen_folder_size = folder_size;
        }
    }
    println!("The result of Part 2 is {}.", chosen_folder_size);

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