use crate::config::{create_default_config, print_commands, save_config, CommandOption, Commands};
use crate::import::import_commands;
use crate::utils::{pause, prompt};
use inquire::Select;

pub fn edit_menu(config_path: &str) {
    let mut config = crate::config::load_config(config_path)
        .unwrap_or_else(|_e| create_default_config(config_path));
    let _original_config = config.clone();
    let mut changes_made = false;

    loop {
        print_commands(&config.commands);

        // Provide the options to add, edit, delete, or return to main menu

        let menu_options = vec![
            "a. ADD a new command",
            "e. EDIT a command",
            "d. DELETE a command",
            "r. RESET (clear all commands)",
            "i. IMPORT from .csv",
            "q. Return to Main Menu",
        ];

        // Display the menu and prompt the user to select an option
        let menu_prompt = Select::new("Select an option: ", menu_options)
            .prompt()
            .expect("Failed to display menu");
        // Parse the selected option
        match menu_prompt {
            "a. ADD a new command" => add_command(&mut config, &mut changes_made),
            "e. EDIT a command" => edit_command(&mut config, &mut changes_made),
            "d. DELETE a command" => delete_command(&mut config, &mut changes_made),
            "r. RESET (clear all commands)" => clear_all_commands(&mut config, &mut changes_made),
            "i. IMPORT from .csv" => {
                import_commands(&mut config, &mut changes_made);
                pause();
            }
            "q. Return to Main Menu" => {
                if changes_made {
                    let save_prompt = Select::new("Save changes?", vec!["Yes", "No"])
                        .prompt()
                        .expect("Failed to display menu");
                    if save_prompt == "Yes" {
                        if validate_json(&config) {
                            save_config(config_path, &config);
                            println!("✅  Config File Updated.");
                        } else {
                            println!("❌  Error: Invalid JSON format. Changes not saved.");
                        }
                    } else {
                        println!("❌  Changes not saved.");
                    }
                }
                break;
            }
            _ => println!("❌  Invalid choice, please try again."),
        }
    }
}

fn add_command(config: &mut Commands, changes_made: &mut bool) {
    let display_name = prompt("Enter display name: ");
    let command = prompt("Enter command: ");

    config.commands.push(CommandOption {
        display_name,
        command,
    });
    *changes_made = true;
}

fn edit_command(config: &mut Commands, changes_made: &mut bool) {
    let menu_options: Vec<String> = config
        .commands
        .iter()
        .map(|c| c.display_name.clone())
        .collect();

    let selected_command = Select::new("Select a command to edit:", menu_options)
        .prompt()
        .expect("Failed to display menu");

    // Find the index of the selected command
    let index = config
        .commands
        .iter()
        .position(|c| c.display_name == selected_command)
        .expect("Selected command not found");

    // Proceed to edit the command
    let command = &mut config.commands[index];
    println!("Editing command: {}", command.display_name);
    println!("Current command: {}", command.command);

    let new_display_name = prompt("Enter new display name (leave empty to keep current): ");
    if !new_display_name.is_empty() {
        command.display_name = new_display_name;
        *changes_made = true;
    }

    let new_command = prompt("Enter new command (leave empty to keep current): ");
    if !new_command.is_empty() {
        command.command = new_command;
        *changes_made = true;
    }
}

fn delete_command(config: &mut Commands, changes_made: &mut bool) {
    let menu_options: Vec<String> = config
        .commands
        .iter()
        .map(|c| c.display_name.clone())
        .collect();
    let selected_command = Select::new("Select a command to delete:", menu_options)
        .prompt()
        .expect("❌  Failed to display menu");
    // Find the index of the selected command
    let index = config
        .commands
        .iter()
        .position(|c| c.display_name == selected_command)
        .expect("⚠️  Selected command not found");
    // Remove the command
    config.commands.remove(index);
    *changes_made = true;
}

fn clear_all_commands(config: &mut Commands, changes_made: &mut bool) {
    config.commands.clear();
    *changes_made = true;
}

fn validate_json(config: &Commands) -> bool {
    // Convert the config to a JSON string and check for errors
    serde_json::to_string(&config).is_ok()
}
