use crate::{
    config::{CommandOption, Config}, // Importing Config struct
    menu::print_command_table,
    utils::pause,
};
use inquire::Select; // Importing Select prompt from inquire crate
use std::{env, fs, path::Path}; // Importing necessary modules from standard library // Importing functions and structs from other modules

// Function to import commands from CSV
pub fn import_commands(config: &mut Config, changes_made: &mut bool) {
    let dir = env::current_dir().unwrap_or_else(|_| ".".into()); // Getting current directory
    let files = match list_csv_files(&dir) {
        // Listing CSV files in current directory
        Ok(files) => files, // Handling successful file listing
        Err(e) => {
            // Handling error
            println!("⚠️ Could not list files: {e}"); // Printing error message
            return; // Returning from function
        }
    };
    if files.is_empty() {
        // Checking if no files found
        println!("⚠️ No files found in {}", dir.display()); // Printing message
        return; // Returning from function
    }

    // Prompting user to select a file to import
    let path = Select::new("Select a file to import: ", files)
        .prompt()
        .expect("Failed to display menu");

    // Reading commands from the selected file
    let mut commands = match read_commands(&path) {
        Ok(commands) => commands, // Handling successful file reading
        Err(e) => {
            // Handling error
            println!("❌  Could not import file: {e}");
            return;
        }
    };

    // Printing the number of commands imported successfully
    let num_commands = commands.len();
    println!("Found {num_commands} commands from {path}");
    print_command_table(&commands);

    // Options for appending, overwriting, or canceling import
    let menu_options = vec![
        "a. APPEND to current commands",
        "o. OVERWRITE current commands",
        "c. CANCEL import",
    ];
    let menu_prompt = Select::new("Select an option: ", menu_options)
        .prompt()
        .expect("❌ Failed to display menu");

    match menu_prompt {
        "a. APPEND to current commands" => {
            // Appending imported commands to the current config
            config.commands.append(&mut commands);
            *changes_made = true;
            println!("{num_commands} commands appended.");
        }
        "o. OVERWRITE current commands" => {
            // Overwriting the current config with the imported commands
            config.commands = commands;
            *changes_made = true;
            println!("Replaced config with {num_commands} commands from {path}");
        }
        _ => {
            // Canceling import
            println!("❌  Canceled import");
            pause();
        }
    }
}

// Function to read commands from a CSV file
fn read_commands(path: &str) -> anyhow::Result<Vec<CommandOption>> {
    let mut reader = csv::Reader::from_path(path)?; // Creating CSV reader
    let mut commands = Vec::new(); // Initializing vector to hold commands
    for result in reader.deserialize() {
        commands.push(result?); // Pushing deserialized command to vector
    }
    Ok(commands)
}

// Function to list CSV files in a directory
fn list_csv_files(dir_path: &Path) -> anyhow::Result<Vec<String>> {
    let dir_reader = fs::read_dir(dir_path)?; // Creating directory reader
    let mut files = Vec::new(); // Initializing vector to hold file names
    for entry in dir_reader {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue; // Skipping directories
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue; // Skipping invalid file names
        };
        if name.ends_with(".csv") {
            files.push(name); // Adding CSV file names to the vector
        }
    }
    Ok(files)
}
