use crate::config::{save_config, CommandOption, Config};
use crate::utils::prompt;
use inquire::Select;
use prettytable::{Table, Row, Cell};
use prettytable::row; //Use prettytable::row to print table of commands
use textwrap::fill; // Import textwrap for wrapping text

pub fn edit_menu(config_path: &str) {
    let mut config = crate::config::load_config(config_path);
    let _original_config = config.clone();
    let mut changes_made = false;

    // Get the terminal width to wrap text accordingly
    let terminal_width = termion::terminal_size().unwrap().0 as usize;


    loop {
        // Create a table to display the commands
        let mut table = Table::new();
        table.add_row(row!["Number", "Display Name", "Command"]);

        for option in &config.commands {
            table.add_row(Row::new(vec![
                Cell::new(&option.number.to_string()),
                Cell::new(&fill(&option.display_name, terminal_width / 3)),
                Cell::new(&fill(&option.command, terminal_width / 3 * 2)),
            ]));
        }

        // Print the table
        println!("Current commands:");
        table.printstd();

        // Provide the options to add, edit, delete, or return to main menu

        let menu_options = vec![
            "a. ADD a new command",
            "e. EDIT a command",
            "d. DELETE a command",
            "q. Return to Main Menu",
        ];

        // Display the menu and prompt the user to select an option
        let menu_prompt = Select::new("Select an option: ", menu_options)
            .prompt()
            .expect("Failed to display menu");
        // Parse the selected option
        match menu_prompt {
            "a. ADD a new command" => add_command(&mut config, &mut changes_made),
            "e. EDIT a command" => edit_command(&mut config,  &mut changes_made),
            "d. DELETE a command" => delete_command(&mut config, &mut changes_made),
            "q. Return to Main Menu" => {
                if changes_made {
                    let save_prompt = Select::new("Save changes?", vec!["Yes", "No"])
                        .prompt()
                        .expect("Failed to display menu");
                    if save_prompt == "Yes" {
                        if validate_json(&config) {
                            save_config(config_path, &config);
                            println!("Changes saved.");
                        } else {
                            println!("Error: Invalid JSON format. Changes not saved.");
                        }
                    } else {
                        println!("Changes not saved.");
                    }
                }
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
        
    }




fn add_command(config: &mut Config, changes_made: &mut bool) {
    let new_number = config.commands.len() + 1;
    let display_name = prompt("Enter display name: ");
    let command = prompt("Enter command: ");

    config.commands.push(CommandOption {
        number: new_number,
        display_name,
        command,
    });
    *changes_made = true;
}


fn edit_command(config: &mut Config, changes_made: &mut bool) {
    let menu_options: Vec<String> = config.commands.iter().map(|c| c.display_name.clone()).collect();
    
    let selected_command = Select::new("Select a command to edit:", menu_options)
        .prompt()
        .expect("Failed to display menu");

    // Find the index of the selected command
    let index = config.commands.iter().position(|c| c.display_name == selected_command)
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



fn delete_command(config: &mut Config, changes_made: &mut bool) {
    let menu_options: Vec<String> = config.commands.iter().map(|c| c.display_name.clone()).collect();
    let selected_command = Select::new("Select a command to delete:", menu_options)
        .prompt()
        .expect("Failed to display menu");
    // Find the index of the selected command
    let index = config.commands.iter().position(|c| c.display_name == selected_command)
        .expect("Selected command not found");
    // Remove the command
    config.commands.remove(index);
    // Re-number the remaining commands
    for (i, command) in config.commands.iter_mut().enumerate() {
        command.number = i + 1;
    }
    *changes_made = true;
}

fn validate_json(config: &Config) -> bool {
    // Convert the config to a JSON string and check for errors
    serde_json::to_string(&config).is_ok()
}