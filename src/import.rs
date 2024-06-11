use std::{env, fs, path::Path}; // Importing necessary modules from standard library

use inquire::Select; // Importing Select prompt from inquire crate

use crate::{config::{print_command_table, CommandOption, Commands}, utils::pause}; // Importing functions and structs from other modules

pub fn import_commands(config: &mut Commands, changes_made: &mut bool) {
    let dir = env::current_dir().unwrap_or_else(|_| ".".into()); // Getting current directory
    let files = match list_csv_files(&dir) { // Listing CSV files in current directory
        Ok(files) => files, // Handling successful file listing
        Err(e) => { // Handling error
            println!("⚠️ Could not list files: {e}"); // Printing error message
            return; // Returning from function
        }
    };
    if files.is_empty() { // Checking if no files found
        println!("⚠️ No files found in {}", dir.display()); // Printing message
        return; // Returning from function
    }
    let path = Select::new("Select a file to import: ", files) // Prompting user to select file
        .prompt()
        .expect("Failed to display menu"); // Handling error

    let mut commands = match read_commands(&path) { // Reading commands from selected file
        Ok(commands) => commands, // Handling successful file reading
        Err(e) => { // Handling error
            println!("❌  Could not import file: {e}"); // Printing error message
            return; // Returning from function
        }
    };

    // Printing the number of commands imported successfully
    println!("Found {} commands from {}", commands.len(), path);
    print_command_table(&commands); // Printing imported commands

    let num_commands = commands.len(); // Getting the number of commands

    // Options for appending, overwriting, or canceling import
    let menu_options = vec![
        "a. APPEND to current commands",
        "o. OVERWRITE current commands",
        "c. CANCEL import",
    ];
    let menu_prompt = Select::new("Select an option: ", menu_options) // Prompting user for option
        .prompt()
        .expect("❌ Failed to display menu"); // Handling error
    match menu_prompt {
        "a. APPEND to current commands" => { // Appending commands to current config
            config.commands.append(&mut commands);
            *changes_made = true; // Flagging changes made
            println!("{num_commands} commands appended."); // Printing success message
        }
        "o. OVERWRITE current commands" => { // Overwriting current config with imported commands
            config.commands = commands;
            *changes_made = true; // Flagging changes made
            println!("Replaced config with {num_commands} commands from {path}"); // Printing success message
        }
        _ => { // Canceling import
            println!("❌  Canceled import"); // Printing message
            pause(); // Pausing execution
        }
    }
}

// Function to read commands from CSV file
fn read_commands(path: &str) -> anyhow::Result<Vec<CommandOption>> {
    let mut reader = csv::Reader::from_path(path)?; // Creating CSV reader
    let mut commands = Vec::new(); // Initializing vector to hold commands
    for result in reader.deserialize() { // Iterating over each row in CSV
        commands.push(result?); // Pushing deserialized command to vector
    }
    Ok(commands) // Returning vector of commands
}

// Function to list CSV files in a directory
fn list_csv_files(dir_path: &Path) -> anyhow::Result<Vec<String>> {
    let dir_reader = fs::read_dir(dir_path)?; // Creating directory reader
    let mut files = Vec::new(); // Initializing vector to hold file names
    for entry in dir_reader { // Iterating over each entry in directory
        let entry = entry?; // Unwrapping entry
        if entry.file_type()?.is_dir() { // Checking if entry is directory
            continue; // Skipping directories
        }
        let Ok(name) = entry.file_name().into_string() else { // Getting file name
            continue; // Skipping if file name is invalid
        };
        if name.ends_with(".csv") { // Checking if file is CSV
            files.push(name); // Adding file name to vector
        }
    }
    Ok(files) // Returning vector of CSV file names
}
