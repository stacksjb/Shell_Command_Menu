use crate::config::{create_default_config, print_commands, save_config, CommandOption, Commands}; // Importing functions and structs from other modules
use crate::import::import_commands; // Importing function from import module
use crate::utils::{pause, prompt}; // Importing functions from utils module
use inquire::Select; // Importing Select prompt from inquire crate

pub fn edit_menu(config_path: &str) {
    let mut config = crate::config::load_config(config_path) // Loading config or creating default if not found
        .unwrap_or_else(|_e| create_default_config(config_path)); // Handling errors by creating default config
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

// Function to add a new command
fn add_command(config: &mut Commands, changes_made: &mut bool) {
    let display_name = prompt("Enter display name:"); // Prompting user for display name
    let command = prompt("Enter command:"); // Prompting user for command

    config.commands.push(CommandOption {
        // Adding new command to config
        display_name,
        command,
    });
    *changes_made = true; // Flagging changes made
}

// Function to edit an existing command
fn edit_command(config: &mut Commands, changes_made: &mut bool) {
    let menu_options: Vec<String> = config // Creating menu options from existing commands
        .commands
        .iter()
        .map(|c| c.display_name.clone())
        .collect();

    let selected_command = Select::new("Select a command to edit:", menu_options)
        .prompt()
        .expect("Failed to display menu"); // Prompting user to select command to edit

    let index = config // Finding index of selected command
        .commands
        .iter()
        .position(|c| c.display_name == selected_command)
        .expect("Selected command not found"); // Handling error if selected command not found

    let command = &mut config.commands[index]; // Getting reference to selected command
    println!("Editing command: {}", command.display_name); // Printing selected command
    println!("Current command: {}", command.command); // Printing current command

    let new_display_name = prompt("Enter new display name (leave empty to keep current):"); // Prompting user for new display name
    if !new_display_name.is_empty() {
        // If new display name is provided
        command.display_name = new_display_name; // Updating display name
        *changes_made = true; // Flagging changes made
    }

    let new_command = prompt("Enter new command (leave empty to keep current):"); // Prompting user for new command
    if !new_command.is_empty() {
        // If new command is provided
        command.command = new_command; // Updating command
        *changes_made = true; // Flagging changes made
    }
}

// Function to delete an existing command
fn delete_command(config: &mut Commands, changes_made: &mut bool) {
    let menu_options: Vec<String> = config // Creating menu options from existing commands
        .commands
        .iter()
        .map(|c| c.display_name.clone())
        .collect();
    let selected_command = Select::new("Select a command to delete:", menu_options)
        .prompt()
        .expect("❌  Failed to display menu"); // Prompting user to select command to delete

    let index = config // Finding index of selected command
        .commands
        .iter()
        .position(|c| c.display_name == selected_command)
        .expect("⚠️  Selected command not found"); // Handling error if selected command not found

    config.commands.remove(index); // Removing selected command
    *changes_made = true; // Flagging changes made
}

// Function to clear all commands
fn clear_all_commands(config: &mut Commands, changes_made: &mut bool) {
    config.commands.clear(); // Clearing all commands
    *changes_made = true; // Flagging changes made
}

// Function to validate JSON format
fn validate_json(config: &Commands) -> bool {
    serde_json::to_string(&config).is_ok() // Converting config to JSON and checking for errors
}
