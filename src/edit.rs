use crate::config::{Config, CommandOption, save_config};
use crate::utils::prompt;
use prettytable::{Table, Row, Cell};
use prettytable::row; // Import the row macro
use termion::input::TermRead;
use std::io::Write; // Import std::io::Write

pub fn edit_json_menu(config_path: &str) {
    let mut config = crate::config::load_config(config_path);

    loop {
        // Create a table to display the commands
        let mut table = Table::new();
        table.add_row(row!["Number", "Display Name", "Command"]);

        for option in &config.commands {
            table.add_row(Row::new(vec![
                Cell::new(&option.number.to_string()),
                Cell::new(&option.display_name),
                Cell::new(&option.command),
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
            }
            termion::event::Key::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap() as usize;
                if num > 0 && num <= config.commands.len() {
                    edit_command(&mut config, num - 1);
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
                    } else {
                        println!("Invalid number, please try again.");
                    }
                }
            }
            termion::event::Key::Char('q') | termion::event::Key::Esc => {
                save_config(config_path, &config);
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
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
    let display_name = prompt(&format!("Enter new display name (current: {}): ", config.commands[index].display_name));
    let command = prompt(&format!("Enter new command (current: {}): ", config.commands[index].command));

    config.commands[index].display_name = display_name;
    config.commands[index].command = command;
}

fn delete_command(config: &mut Config, index: usize) {
    config.commands.remove(index);
    // Re-number the remaining commands
    for (i, command) in config.commands.iter_mut().enumerate() {
        command.number = i + 1;
    }
}
