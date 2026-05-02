use crate::config::{
    CommandOption, Config, edit_cmd_sound, edit_window_title, save_config, validate_json,
};
use crate::csv::import_commands;
use crate::menu_main::prompt_or_return;
use crate::utils::pause;
use inquire::Select;
use prettytable::{Cell, Row, Table, row};
use std::path::Path;
use std::process;
use textwrap::fill;

pub fn edit_menu(config_path: &Path) {
    let mut config = crate::config::load_config(config_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });
    let original_config = config.clone();
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
                                match save_config(config_path, &config) {
                                    Ok(()) => {
                                        println!(
                                            "✅  Changes Saved. Press any key to return to Main Menu..."
                                        );
                                        pause();
                                    }
                                    Err(e) => println!("❌  Error saving config: {e}"),
                                }
                            } else {
                                println!("❌  Error: Invalid JSON format. Changes not saved.");
                            }
                        }
                        Some("No") => {
                            discard_edit_session(&mut config, &original_config, &mut changes_made);
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

    add_command_to_config(config, display_name, command, changes_made);
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

    let _ = edit_command_at(config, command_number, display_name, command, changes_made);
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

    if reorder_command_to_position(config, command_number, new_position, changes_made) {
        println!("✅  Command moved to position {new_position}.");
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

    if let Some(deleted) = delete_command_at(config, command_number, changes_made) {
        println!(
            "✅  Command '{}' deleted successfully.",
            deleted.display_name
        );
    }
}

pub fn clear_all_commands(config: &mut Config, changes_made: &mut bool) {
    if clear_commands(config, changes_made) {
        println!("✅  All commands cleared successfully.");
    } else {
        println!("✅  No commands to clear.");
    }
}

pub fn add_command_to_config(
    config: &mut Config,
    display_name: String,
    command: String,
    changes_made: &mut bool,
) {
    config.commands.push(CommandOption {
        display_name,
        command,
    });
    *changes_made = true;
}

pub fn edit_command_at(
    config: &mut Config,
    index: usize,
    display_name: String,
    command: String,
    changes_made: &mut bool,
) -> bool {
    let Some(existing) = config.commands.get_mut(index) else {
        return false;
    };

    let updated = CommandOption {
        display_name,
        command,
    };
    if *existing != updated {
        *existing = updated;
        *changes_made = true;
    }
    true
}

pub fn reorder_command_to_position(
    config: &mut Config,
    index: usize,
    new_position: usize,
    changes_made: &mut bool,
) -> bool {
    if index >= config.commands.len() || new_position == 0 || new_position > config.commands.len() {
        return false;
    }

    let new_index = new_position - 1;
    if index == new_index {
        return true;
    }

    let command_to_move = config.commands.remove(index);
    config.commands.insert(new_index, command_to_move);
    *changes_made = true;
    true
}

pub fn delete_command_at(
    config: &mut Config,
    index: usize,
    changes_made: &mut bool,
) -> Option<CommandOption> {
    if index >= config.commands.len() {
        return None;
    }

    *changes_made = true;
    Some(config.commands.remove(index))
}

pub fn clear_commands(config: &mut Config, changes_made: &mut bool) -> bool {
    if config.commands.is_empty() {
        return false;
    }

    config.commands.clear();
    *changes_made = true;
    true
}

pub fn discard_edit_session(
    config: &mut Config,
    original_config: &Config,
    changes_made: &mut bool,
) {
    *config = original_config.clone();
    *changes_made = false;
}

pub fn print_commands(commands: &[CommandOption]) {
    println!("{} total commands:", commands.len());

    let terminal_width = termion::terminal_size().map_or(80, |(width, _)| width as usize);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with_commands(names: &[&str]) -> Config {
        Config {
            commands: names
                .iter()
                .map(|name| CommandOption {
                    display_name: (*name).into(),
                    command: format!("echo {name}"),
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn add_command_to_config_marks_changed() {
        let mut config = Config::default();
        let mut changed = false;

        add_command_to_config(&mut config, "List".into(), "ls".into(), &mut changed);

        assert_eq!(config.commands.len(), 1);
        assert_eq!(config.commands[0].display_name, "List");
        assert!(changed);
    }

    #[test]
    fn edit_command_at_updates_existing_command() {
        let mut config = config_with_commands(&["Old"]);
        let mut changed = false;

        let edited = edit_command_at(&mut config, 0, "New".into(), "date".into(), &mut changed);

        assert!(edited);
        assert_eq!(config.commands[0].display_name, "New");
        assert_eq!(config.commands[0].command, "date");
        assert!(changed);
    }

    #[test]
    fn edit_command_at_same_value_stays_clean() {
        let mut config = config_with_commands(&["Same"]);
        let mut changed = false;

        let edited = edit_command_at(
            &mut config,
            0,
            "Same".into(),
            "echo Same".into(),
            &mut changed,
        );

        assert!(edited);
        assert!(!changed);
    }

    #[test]
    fn reorder_command_to_position_moves_first_to_last() {
        let mut config = config_with_commands(&["A", "B", "C"]);
        let mut changed = false;

        let moved = reorder_command_to_position(&mut config, 0, 3, &mut changed);

        assert!(moved);
        assert_eq!(
            config
                .commands
                .iter()
                .map(|command| command.display_name.as_str())
                .collect::<Vec<_>>(),
            vec!["B", "C", "A"]
        );
        assert!(changed);
    }

    #[test]
    fn reorder_command_to_same_position_stays_clean() {
        let mut config = config_with_commands(&["A", "B"]);
        let mut changed = false;

        let moved = reorder_command_to_position(&mut config, 1, 2, &mut changed);

        assert!(moved);
        assert_eq!(config.commands[1].display_name, "B");
        assert!(!changed);
    }

    #[test]
    fn reorder_command_to_invalid_position_does_not_change_config() {
        let mut config = config_with_commands(&["A", "B"]);
        let original = config.clone();
        let mut changed = false;

        let moved = reorder_command_to_position(&mut config, 0, 3, &mut changed);

        assert!(!moved);
        assert_eq!(config, original);
        assert!(!changed);
    }

    #[test]
    fn delete_command_at_removes_existing_command() {
        let mut config = config_with_commands(&["A", "B"]);
        let mut changed = false;

        let deleted = delete_command_at(&mut config, 0, &mut changed).expect("deleted command");

        assert_eq!(deleted.display_name, "A");
        assert_eq!(config.commands[0].display_name, "B");
        assert!(changed);
    }

    #[test]
    fn clear_commands_empty_config_stays_clean() {
        let mut config = Config::default();
        let mut changed = false;

        let cleared = clear_commands(&mut config, &mut changed);

        assert!(!cleared);
        assert!(!changed);
    }

    #[test]
    fn discard_edit_session_restores_original_config_and_clears_dirty_flag() {
        let original = config_with_commands(&["Original"]);
        let mut config = config_with_commands(&["Changed"]);
        let mut changed = true;

        discard_edit_session(&mut config, &original, &mut changed);

        assert_eq!(config, original);
        assert!(!changed);
    }
}
