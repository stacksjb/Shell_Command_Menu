use crate::{
    config::{edit_cmd_sound, save_config, validate_json, CommandOption},
    csv::import_commands,
    utils::{pause, play_sound, run_command},
}; // Importing functions from utils module
use inquire::Select;
use prettytable::{row, Cell, Row, Table}; // Importing types for creating tables
use std::{
    io::{stdout, Write},
    path::PathBuf,
    process,
};
use termion::{clear, cursor, terminal_size};
use textwrap::fill; // Importing fill function from textwrap crate // Importing IntoRawMode trait for entering raw mode
                    // Importing Select prompt from inquire crate
use std::process::exit; // Importing exit function from std::process module

//Function for main execution Menu
#[tokio::main]
pub async fn display_menu(config_path: &PathBuf) {
    let mut selected_commands: Vec<usize> = vec![]; // Initializing vector to hold selected commands
    let mut last_selected: Option<usize> = None; // Initializing variable to hold index of last selected command

    loop {
        // Checking if the config is valid or exists; editing if not
        let config = match crate::config::load_config(config_path) {
            // Loading config or handling errors
            Ok(config) => config, // Handling successful config loading
            Err(_) => {
                // Handling error
                println!("⚠️ Config does not exist or is invalid; editing new config"); // Printing message
                edit_menu(config_path); // Editing config
                selected_commands.clear(); // Clearing selected commands
                last_selected = None; // Resetting last selected index
                continue; // Continuing loop
            }
        };
        // Get the terminal height to set the height of the inquire prompt
        let term_height = get_terminal_height() as usize;
        let display_height = term_height.saturating_sub(3); // Adjust height to avoid overflow

        // Clear the terminal screen before displaying the menu
        clear_screen();
        // Create a list of menu options
        let mut menu_options = generate_menu(&config.commands, &selected_commands);

        menu_options.push("e. EDIT Commands".to_string()); // Adding option to edit commands
        menu_options.push("q. EXIT".to_string()); // Adding option to exit

        // Display the menu and prompt the user to select an option
        let menu_prompt = if let Some(last) = last_selected {
            // Checking if last selected index exists
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:", // Prompt message
                menu_options,
            )
            .with_starting_cursor(last) // Setting starting cursor to last selected index
            .with_page_size(display_height) // Setting page size to terminal height
        } else {
            // If no last selected index
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:", // Prompt message
                menu_options,
            )
            .with_page_size(display_height) // Setting page size to terminal height
        };

        match menu_prompt.prompt() {
            Ok(choice) => {
                if choice == "q. EXIT" {
                    // Checking if user chose to exit
                    println!("Exiting..."); // Printing exit message
                    exit(0); // Exiting program
                } else if choice == "e. EDIT Commands" {
                    // Checking if user chose to edit commands
                    edit_menu(config_path); // Editing commands

                    selected_commands.clear(); // Clearing selected commands
                    last_selected = None; // Resetting last selected index
                    continue; // Continuing loop
                } else {
                    // Extract the command number from choice, trim to remove any leading/trailing spaces
                    let num_str = choice.split('.').next().unwrap().trim();
                    let num: usize = num_str.parse().unwrap(); // Parse the number

                    if let Some(index) = num.checked_sub(1) {
                        // Checking if index is valida
                        if let Some(command) = config.commands.get(index) {
                            // Getting command at index
                            //If cmd_sound is set, play the sound
                            if config.cmd_sound != Some(PathBuf::new()) {
                                if let Some(cmd_sound) = &config.cmd_sound {
                                    tokio::spawn(play_sound(cmd_sound.clone()));
                                    // Playing sound asynchronously
                                }
                            }
                            run_command(&command.command); // Running selected command
                            selected_commands.push(num); // Adding command number to selected commands
                            last_selected = Some(index); // Updating last selected index
                        } else {
                            println!("❌  Invalid choice, please try again."); // Printing error message
                        }
                    } else {
                        println!("❌  Invalid choice, please try again."); // Printing error message
                    }
                }
            }
            Err(_) => println!("❌  Error reading input. Please try again."), // Handling input error
        }
    }
}

//Function to generate the Menu
// Function to generate a menu based on provided commands and selected commands
pub fn generate_menu(commands: &[CommandOption], selected_commands: &[usize]) -> Vec<String> {
    let max_number_width = commands.len().to_string().len(); // Calculate width of max number for padding
    commands
        .iter()
        .enumerate()
        .map(|(index, cmd)| {
            let number = index + 1; // Get command number
            let padded_number = format!("{: >width$}", number, width = max_number_width); // Pad number
            if selected_commands.contains(&number) {
                // If selected, apply strike-through
                format!("{}. {}", padded_number, strike_through(&cmd.display_name))
            } else {
                format!("{}. {}", padded_number, &cmd.display_name) // Normal display if not selected
            }
        })
        .collect()
}

// Function to get the terminal height
pub fn get_terminal_height() -> u16 {
    let (_, height) = terminal_size().unwrap();
    height
}

// Function to clear the screen
pub fn clear_screen() {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush().unwrap();
}

// Function to strike through text
fn strike_through(text: &str) -> String {
    let mut result = String::new(); // Initializing an empty string to hold the result
    for c in text.chars() {
        // Iterating over each character in the text
        result.push(c); // Appending the character to the result string
        result.push('\u{0336}'); // Adding a Unicode character for strike-through
    }
    result // Returning the resulting string with strike-through
}

//Functions for the Edit Menu
pub fn edit_menu(config_path: &PathBuf) {
    let mut config = crate::config::load_config(config_path) // Loading config or creating default if not found
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e); // Print the error message
            process::exit(1) // Exit with a status code of 1
        }); // Handling errors by creating default config
    let _original_config = config.clone(); // Cloning the original config
    let mut changes_made = false; // Flag to track changes made to config

    loop {
        println!("🛠️ Welcome to the Edit Command Menu 🛠️"); // Printing menu header
        print_commands(&config.commands); // Printing current commands

        // Options for adding, editing, deleting, or returning to main menu
        let menu_options = vec![
            "a. ADD a new command",
            "e. EDIT a command",
            "d. DELETE a command",
            "r. RESET (clear all commands)",
            "i. IMPORT from .csv",
            "s. SET sound file path",
            "q. Return to Main Menu (prompt to save changes)",
        ];
        // Displaying menu and prompting user for selection
        let menu_prompt = Select::new("Select an option: ", menu_options)
            .prompt()
            .expect("Failed to display menu");

        // Handling user selection
        match menu_prompt {
            "a. ADD a new command" => add_command(&mut config, &mut changes_made), // Adding a new command
            "e. EDIT a command" => edit_command(&mut config, &mut changes_made), // Editing an existing command
            "d. DELETE a command" => delete_command(&mut config, &mut changes_made), // Deleting a command
            "r. RESET (clear all commands)" => clear_all_commands(&mut config, &mut changes_made), // Clearing all commands
            "s. SET sound file path" => {
                // Setting sound file path
                edit_cmd_sound(&mut config, &mut changes_made);
            }
            "i. IMPORT from .csv" => {
                // Importing commands from CSV file
                import_commands(&mut config, &mut changes_made);
                print!("Press any key to return to Edit Command Menu..."); // Prompting user to return to menu
                pause(); // Pausing execution
            }
            "q. Return to Main Menu (prompt to save changes)" => {
                // Returning to main menu
                if changes_made {
                    // If changes were made
                    let save_prompt = Select::new("Save changes?", vec!["Yes", "No"])
                        .prompt()
                        .expect("Failed to display menu"); // Prompting user to save changes
                    if save_prompt == "Yes" {
                        // If user chooses to save changes
                        if validate_json(&config) {
                            // Validating JSON format
                            save_config(config_path, &config); // Saving changes to config file
                            println!("✅  Changes Saved. Press any key to return to Main Menu..."); // Printing success message
                            pause(); // Pausing execution
                        } else {
                            println!("❌  Error: Invalid JSON format. Changes not saved.");
                            // Printing error message
                        }
                    } else {
                        println!("❌  Changes not saved. Press any key to return to Main Menu..."); // Printing message
                        pause(); // Pausing execution
                    }
                }
                break; // Exiting loop
            }
            _ => println!("❌  Invalid choice, please try again."), // Handling invalid input
        }
    }
}

//Add Command Function
pub fn add_command(config: &mut crate::config::Config, changes_made: &mut bool) {
    let display_name = inquire::Text::new("Enter the display name for the command:")
        .with_help_message("This name will be displayed in the menu")
        .prompt()
        .expect("Failed to get display name"); // Prompting user for display name
    let command = inquire::Text::new("Enter the command to execute:")
        .with_help_message("This command will be executed in the shell")
        .prompt()
        .expect("Failed to get command"); // Prompting user for command

    config.commands.push(CommandOption {
        display_name,
        command,
    }); // Adding new command to config
    *changes_made = true; // Setting changes made flag
}
//Edit Command Function
pub fn edit_command(config: &mut crate::config::Config, changes_made: &mut bool) {
    let num_commands = config.commands.len(); // Getting the number of commands
    if num_commands == 0 {
        // Checking if there are no commands
        println!("❌  No commands to edit. Please add a command first."); // Printing error message
        return; // Returning from the function
    }

    // Creating a list of command display names for selection
    let command_names: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}. {}", i + 1, cmd.display_name))
        .collect();

    // Prompting user to select a command to edit
    let command_index = Select::new("Select a command to edit:", command_names)
        .prompt()
        .expect("Failed to display menu");

    // Find the index of the selected command by parsing the number from the string
    let command_number = command_index
        .split('.')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    // Prompting user for new display name with the current value pre-filled
    let display_name = inquire::Text::new("Enter the new display name for the command:")
        .with_initial_value(&config.commands[command_number].display_name)
        .with_help_message("This name will be displayed in the menu")
        .prompt()
        .expect("Failed to get display name");

    // Prompting user for new command with the current value pre-filled
    let command = inquire::Text::new("Enter the new command to execute:")
        .with_initial_value(&config.commands[command_number].command)
        .with_help_message("This command will be executed in the shell")
        .prompt()
        .expect("Failed to get command");

    // Updating the command in the config
    config.commands[command_number] = CommandOption {
        display_name,
        command,
    };

    *changes_made = true; // Setting changes made flag
}

//Delete Command Function
pub fn delete_command(config: &mut crate::config::Config, changes_made: &mut bool) {
    let num_commands = config.commands.len(); // Getting the number of commands
    if num_commands == 0 {
        // Checking if there are no commands
        println!("❌  No commands to delete. Please add a command first."); // Printing error message
        return; // Returning from the function
    }

    // Creating a list of command display names for selection
    let command_names: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}. {}", i + 1, cmd.display_name))
        .collect();

    // Prompting user to select a command to delete
    let command_index = Select::new("Select a command to delete:", command_names)
        .prompt()
        .expect("Failed to display menu");

    // Find the index of the selected command by parsing the number from the string
    let command_number = command_index
        .split('.')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let deleted_command = config.commands.remove(command_number); // Removing the selected command
    println!(
        "✅  Command '{}' deleted successfully.",
        deleted_command.display_name
    ); // Printing success message
    *changes_made = true; // Setting changes made flag
}

// Print the command table
pub fn print_commands(commands: &[CommandOption]) {
    print_num_commands(commands); // Printing the count of commands
    print_command_table(commands); // Printing the command table
}

// Print the number of commands
pub fn print_num_commands(commands: &[CommandOption]) {
    println!("{} total commands:", commands.len()); // Printing the count of commands
}

pub fn print_command_table(commands: &[CommandOption]) {
    let terminal_width = termion::terminal_size().unwrap().0 as usize; // Getting terminal width
    if !commands.is_empty() {
        // Checking if there are any commands
        let mut table = Table::new(); // Creating a new table
        table.add_row(row!["Number", "Display Name", "Command"]); // Adding table headers

        for (i, option) in commands.iter().enumerate() {
            // Iterating over commands
            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()), // Adding cell for command number
                Cell::new(&fill(&option.display_name, terminal_width / 3)), // Adding cell for display name with text wrapping
                Cell::new(&fill(&option.command, terminal_width / 3 * 2)), // Adding cell for command with text wrapping
            ]));
        }

        table.printstd(); // Printing the table to stdout
    }
}

//Clear All Commands Function
pub fn clear_all_commands(config: &mut crate::config::Config, changes_made: &mut bool) {
    config.commands.clear(); // Clearing all commands
    println!("✅  All commands cleared successfully."); // Printing success message
    *changes_made = true; // Setting changes made flag
}
