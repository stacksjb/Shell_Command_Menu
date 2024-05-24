use crate::config::{Config, CommandOption, save_config};
use crate::utils::prompt;
use prettytable::{Table, Row, Cell};
use prettytable::row; // Import the row macro
use termion::input::TermRead;
use std::io::Write; // Import std::io::Write
use textwrap::fill; // Import textwrap for wrapping text

pub fn edit_json_menu(config_path: &str) {
    let mut config = crate::config::load_config(config_path);
    let _original_config = config.clone();
    let mut changes_made = false;

    // Get the terminal width to wrap text accordingly
    let terminal_width = termion::terminal_size().map(|(w, _)| w as usize).unwrap_or(80);

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

        // Prompt for input
        println!("Enter 'a' to add a new command, a number to edit a command, 'd' to delete a command, or 'q' to return to the main menu.");
        print!("Your choice: ");
        std::io::stdout().flush().unwrap(); // Ensure std::io::Write is imported

        let stdin = std::io::stdin();
        let choice = stdin.keys().next().unwrap().unwrap();

        match choice {
            termion::event::Key::Char('a') => {
                add_command(&mut config);
                changes_made = true;
            }
            termion::event::Key::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap() as usize;
                if num > 0 && num <= config.commands.len() {
                    edit_command(&mut config, num - 1);
                    changes_made = true;
                } else {
                    println!("Invalid choice, please try again.");
                }
            }
            termion::event::Key::Char('d') => {
                println!("Enter the number of the command to delete: ");
                std::io::stdout().flush().unwrap(); // Ensure std::io::Write is imported
                let stdin = std::io::stdin(); // Create a new instance of stdin
                let del_choice = stdin.keys().next().unwrap().unwrap();
                if let termion::event::Key::Char(c) = del_choice {
                    let num = c.to_digit(10).unwrap_or(0) as usize;
                    if num > 0 && num <= config.commands.len() {
                        delete_command(&mut config, num - 1);
                        changes_made = true;
                    } else {
                        println!("Invalid number, please try again.");
                    }
                }
            }
            termion::event::Key::Char('q') | termion::event::Key::Esc => {
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }

    if changes_made {
        // Ask if the user wants to save changes
        let save_changes = prompt("Do you want to save changes? (y/n): ");
        if save_changes.trim().to_lowercase() == "y" {
            if validate_json(&config) {
                save_config(config_path, &config);
                println!("Changes saved successfully.");
            } else {
                println!("Invalid configuration. Changes not saved.");
            }
        } else {
            println!("Changes discarded.");
        }
    }
}

fn add_command(config: &mut Config) {
    let new_number = config.commands.len() + 1;
    let display_name = prompt("Enter display name: ");
    let command = prompt("Enter command: ");

    config.commands.push(CommandOption {
        number: new_number,
        display_name,
        command,
    });
}

fn edit_command(config: &mut Config, index: usize) {
    let current_display_name = &config.commands[index].display_name;
    let current_command = &config.commands[index].command;

    let display_name = prompt(&format!("Enter new display name (current: {}): ", current_display_name));
    let command = prompt(&format!("Enter new command (current: {}): ", current_command));

    if !display_name.trim().is_empty() {
        config.commands[index].display_name = display_name;
    }
    if !command.trim().is_empty() {
        config.commands[index].command = command;
    }
}

fn delete_command(config: &mut Config, index: usize) {
    config.commands.remove(index);
    // Re-number the remaining commands
    for (i, command) in config.commands.iter_mut().enumerate() {
        command.number = i + 1;
    }
}

fn validate_json(config: &Config) -> bool {
    // Convert the config to a JSON string and check for errors
    serde_json::to_string(&config).is_ok()
}

pub fn reset_strikeout_state(selected_commands: &mut Vec<usize>, last_selected: &mut Option<usize>) {
    selected_commands.clear();
    *last_selected = None;
}