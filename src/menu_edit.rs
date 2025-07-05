use crate::config::{
    CommandOption, Config, edit_cmd_sound, edit_window_title, save_config, validate_json,
};
use crate::csv::import_commands;
use crate::menu_main::prompt_or_return;
use crate::utils::pause;
use inquire::Select;
use prettytable::{Cell, Row, Table, row};
use std::path::PathBuf;
use std::process;
use textwrap::fill;

pub fn edit_menu(config_path: &PathBuf) {
    let mut config = crate::config::load_config(config_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });
    let _original_config = config.clone();
    let mut changes_made = false;

    loop {
        println!("\n🛠️ Welcome to the Edit Menu 🛠️");
        print_commands(&config.commands);

        let menu_options = vec![
            "a. ADD a new command",
            "e. EDIT a command",
            "o. REORDER a command",
            "d. DELETE a command",
            "r. RESET (clear all commands)",
            "i. IMPORT from .csv",
            "s. SET sound file path",
            "t. SET Window Title settings",
            "q. Return to Main Menu (prompt to save changes)",
        ];

        let menu_prompt =
            prompt_or_return(|| Select::new("Select an option: ", menu_options).prompt());
        let choice = match menu_prompt {
            Some(choice) => choice,
            None => continue,
        };

        match choice {
            "a. ADD a new command" => add_command(&mut config, &mut changes_made),
            "e. EDIT a command" => edit_command(&mut config, &mut changes_made),
            "o. REORDER a command" => reorder_command(&mut config, &mut changes_made),
            "d. DELETE a command" => delete_command(&mut config, &mut changes_made),
            "r. RESET (clear all commands)" => clear_all_commands(&mut config, &mut changes_made),
            "s. SET sound file path" => edit_cmd_sound(&mut config, &mut changes_made),
            "t. SET Window Title settings" => edit_window_title(&mut config, &mut changes_made),
            "i. IMPORT from .csv" => {
                import_commands(&mut config, &mut changes_made);
                print!("Press any key to return to Edit Command Menu...");
                pause();
            }
            "q. Return to Main Menu (prompt to save changes)" => {
                if changes_made {
                    let save_prompt = prompt_or_return(|| {
                        Select::new("Save changes?", vec!["Yes", "No"]).prompt()
                    });
                    match save_prompt {
                        Some("Yes") => {
                            if validate_json(&config) {
                                save_config(config_path, &config);
                                println!(
                                    "✅  Changes Saved. Press any key to return to Main Menu..."
                                );
                                pause();
                            } else {
                                println!("❌  Error: Invalid JSON format. Changes not saved.");
                            }
                        }
                        Some("No") => {
                            println!(
                                "❌  Changes not saved. Press any key to return to Main Menu..."
                            );
                            pause();
                        }
                        _ => continue,
                    }
                }
                break;
            }
            _ => println!("❌  Invalid choice, please try again."),
        }
    }
}

pub fn add_command(config: &mut Config, changes_made: &mut bool) {
    let display_name = match prompt_or_return(|| {
        inquire::Text::new("Enter the display name for the command:")
            .with_help_message("This name will be displayed in the menu")
            .prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    let command = match prompt_or_return(|| {
        inquire::Text::new("Enter the command to execute:")
            .with_help_message("This command will be executed in the shell")
            .prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    config.commands.push(CommandOption {
        display_name,
        command,
    });
    *changes_made = true;
}

pub fn edit_command(config: &mut Config, changes_made: &mut bool) {
    if config.commands.is_empty() {
        println!("❌  No commands to edit. Please add a command first.");
        return;
    }

    let command_names: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}. {}", i + 1, cmd.display_name))
        .collect();

    let command_index =
        match prompt_or_return(|| Select::new("Select a command to edit:", command_names).prompt())
        {
            Some(value) => value,
            None => return,
        };

    let command_number = command_index
        .split('.')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let display_name = match prompt_or_return(|| {
        inquire::Text::new("Enter the new display name for the command:")
            .with_initial_value(&config.commands[command_number].display_name)
            .prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    let command = match prompt_or_return(|| {
        inquire::Text::new("Enter the new command to execute:")
            .with_initial_value(&config.commands[command_number].command)
            .prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    config.commands[command_number] = CommandOption {
        display_name,
        command,
    };
    *changes_made = true;
}

pub fn reorder_command(config: &mut Config, changes_made: &mut bool) {
    if config.commands.is_empty() {
        println!("❌  No commands to reorder. Please add a command first.");
        return;
    }

    let command_names: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}. {}", i + 1, cmd.display_name))
        .collect();

    let command_index = match prompt_or_return(|| {
        Select::new("Select a command to reorder:", command_names).prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    let command_number = command_index
        .split('.')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let new_position: usize = match prompt_or_return(|| {
        inquire::Text::new("Enter the new position for this command:")
            .with_help_message("Enter a number between 1 and the number of commands.")
            .prompt()
    }) {
        Some(value) => value.parse().unwrap_or(command_number + 1),
        None => return,
    };

    if new_position > 0 && new_position <= config.commands.len() {
        let command_to_move = config.commands.remove(command_number);
        config.commands.insert(new_position - 1, command_to_move);
        println!("✅  Command moved to position {new_position}.");
        *changes_made = true;
    } else {
        println!(
            "❌  Invalid position. Please enter a number between 1 and {}.",
            config.commands.len()
        );
    }
}

pub fn delete_command(config: &mut Config, changes_made: &mut bool) {
    if config.commands.is_empty() {
        println!("❌  No commands to delete. Please add a command first.");
        return;
    }

    let command_names: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}. {}", i + 1, cmd.display_name))
        .collect();

    let command_index = match prompt_or_return(|| {
        Select::new("Select a command to delete:", command_names).prompt()
    }) {
        Some(value) => value,
        None => return,
    };

    let command_number = command_index
        .split('.')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let deleted = config.commands.remove(command_number);
    println!(
        "✅  Command '{}' deleted successfully.",
        deleted.display_name
    );
    *changes_made = true;
}

pub fn clear_all_commands(config: &mut Config, changes_made: &mut bool) {
    config.commands.clear();
    println!("✅  All commands cleared successfully.");
    *changes_made = true;
}

pub fn print_commands(commands: &[CommandOption]) {
    println!("{} total commands:", commands.len());

    let terminal_width = termion::terminal_size().unwrap().0 as usize;
    if !commands.is_empty() {
        let mut table = Table::new();
        table.add_row(row!["Number", "Display Name", "Command"]);

        for (i, option) in commands.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(&fill(&option.display_name, terminal_width / 3)),
                Cell::new(&fill(&option.command, terminal_width / 3 * 2)),
            ]));
        }

        table.printstd();
    }
}
