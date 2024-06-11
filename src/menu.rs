use crate::edit::edit_menu; // Importing edit_menu function from edit module
use crate::utils::{generate_menu, get_page_size, play_sound, run_command}; // Importing functions from utils module
use inquire::Select; // Importing Select prompt from inquire crate
use std::process::exit; // Importing exit function from std::process module

#[tokio::main]
pub async fn display_menu(config_path: &str) {
    let mut selected_commands: Vec<usize> = vec![]; // Initializing vector to hold selected commands
    let mut last_selected: Option<usize> = None; // Initializing variable to hold index of last selected command

    loop {
        // Checking if the config is valid or exists; editing if not
        let config = match crate::config::load_config(config_path) { // Loading config or handling errors
            Ok(config) => config, // Handling successful config loading
            Err(_) => { // Handling error
                println!("⚠️ Config does not exist or is invalid; editing new config"); // Printing message
                edit_menu(config_path); // Editing config
                selected_commands.clear(); // Clearing selected commands
                last_selected = None; // Resetting last selected index
                continue; // Continuing loop
            }
        };

        // Determine the terminal height and set the page size accordingly
        let page_size = get_page_size(); // Getting terminal height and setting page size accordingly

        // Create a list of menu options
        let mut menu_options = generate_menu(&config, &selected_commands); // Generating menu options from config and selected commands
        menu_options.push("e. EDIT Commands".to_string()); // Adding option to edit commands
        menu_options.push("q. EXIT".to_string()); // Adding option to exit

        // Display the menu and prompt the user to select an option
        let menu_prompt = if let Some(last) = last_selected { // Checking if last selected index exists
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:", // Prompt message
                menu_options,
            )
            .with_starting_cursor(last) // Setting starting cursor to last selected index
            .with_page_size(page_size) // Setting page size
        } else {
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:", // Prompt message
                menu_options,
            )
            .with_page_size(page_size) // Setting page size
        };

        match menu_prompt.prompt() {
            Ok(choice) => {
                if choice == "q. EXIT" { // Checking if user chose to exit
                    println!("Exiting..."); // Printing exit message
                    exit(0); // Exiting program
                } else if choice == "e. EDIT Commands" { // Checking if user chose to edit commands
                    edit_menu(config_path); // Editing commands
                    selected_commands.clear(); // Clearing selected commands
                    last_selected = None; // Resetting last selected index
                    continue; // Continuing loop
                } else {
                    let num: usize = choice.split('.').next().unwrap().parse().unwrap(); // Extracting command number from choice
                    if let Some(index) = num.checked_sub(1) { // Checking if index is valid
                        if let Some(command) = config.commands.get(index) { // Getting command at index
                            tokio::spawn(play_sound("whoosh-6316.mp3")); // Playing sound asynchronously
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
