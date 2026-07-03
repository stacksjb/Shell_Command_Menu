use crate::config::{
    CommandOption, Config, edit_cmd_sound, edit_window_title, save_config, validate_config,
};
use crate::csv::{export_commands, import_commands};
use crate::menu_main::prompt_or_return;
use crate::utils::pause;
use inquire::Select;
use prettytable::{Cell, Row, Table, row};
use std::path::Path;
use std::process;
use textwrap::fill;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EditMenuChoice {
    Add,
    Edit,
    Reorder,
    Delete,
    Reset,
    Import,
    Export,
    Sound,
    WindowTitle,
    Quit,
}

impl EditMenuChoice {
    const ADD: &'static str = "a. ADD a new command";
    const EDIT: &'static str = "e. EDIT a command";
    const REORDER: &'static str = "o. REORDER a command";
    const DELETE: &'static str = "d. DELETE a command";
    const RESET: &'static str = "r. RESET (clear all commands)";
    const IMPORT: &'static str = "i. IMPORT from .csv";
    const EXPORT: &'static str = "x. EXPORT to .csv";
    const SOUND: &'static str = "s. SET sound file path";
    const WINDOW_TITLE: &'static str = "t. SET Window Title settings";
    const QUIT: &'static str = "q. Return to Main Menu (prompt to save changes)";

    fn labels() -> Vec<&'static str> {
        vec![
            Self::ADD,
            Self::EDIT,
            Self::REORDER,
            Self::DELETE,
            Self::RESET,
            Self::IMPORT,
            Self::EXPORT,
            Self::SOUND,
            Self::WINDOW_TITLE,
            Self::QUIT,
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            Self::ADD => Some(Self::Add),
            Self::EDIT => Some(Self::Edit),
            Self::REORDER => Some(Self::Reorder),
            Self::DELETE => Some(Self::Delete),
            Self::RESET => Some(Self::Reset),
            Self::IMPORT => Some(Self::Import),
            Self::EXPORT => Some(Self::Export),
            Self::SOUND => Some(Self::Sound),
            Self::WINDOW_TITLE => Some(Self::WindowTitle),
            Self::QUIT => Some(Self::Quit),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SaveChoice {
    Yes,
    No,
}

impl SaveChoice {
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Yes" => Some(Self::Yes),
            "No" => Some(Self::No),
            _ => None,
        }
    }
}

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

        let menu_prompt = prompt_or_return(|| {
            Select::new("Select an option: ", EditMenuChoice::labels()).prompt()
        });
        let Some(choice) = menu_prompt else {
            continue;
        };

        let Some(choice) = EditMenuChoice::from_label(choice) else {
            println!("❌  Invalid choice, please try again.");
            continue;
        };

        match choice {
            EditMenuChoice::Add => add_command(&mut config, &mut changes_made),
            EditMenuChoice::Edit => edit_command(&mut config, &mut changes_made),
            EditMenuChoice::Reorder => reorder_command(&mut config, &mut changes_made),
            EditMenuChoice::Delete => delete_command(&mut config, &mut changes_made),
            EditMenuChoice::Reset => clear_all_commands(&mut config, &mut changes_made),
            EditMenuChoice::Sound => edit_cmd_sound(&mut config, &mut changes_made),
            EditMenuChoice::WindowTitle => edit_window_title(&mut config, &mut changes_made),
            EditMenuChoice::Import => {
                import_commands(&mut config, &mut changes_made);
                print!("Press any key to return to Edit Command Menu...");
                pause();
            }
            EditMenuChoice::Export => {
                export_commands(&config);
                print!("Press any key to return to Edit Command Menu...");
                pause();
            }
            EditMenuChoice::Quit => {
                if changes_made {
                    let save_prompt = prompt_or_return(|| {
                        Select::new("Save changes?", vec!["Yes", "No"]).prompt()
                    });
                    let save_choice = save_prompt.and_then(SaveChoice::from_label);
                    match save_choice {
                        Some(SaveChoice::Yes) => {
                            if !save_current_config(config_path, &config) {
                                continue;
                            }
                        }
                        Some(SaveChoice::No) => {
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
        }
    }
}

fn save_current_config(config_path: &Path, config: &Config) -> bool {
    match validate_config(config) {
        Ok(()) => match save_config(config_path, config) {
            Ok(()) => {
                println!("✅  Changes Saved. Press any key to return to Main Menu...");
                pause();
                true
            }
            Err(e) => {
                println!("❌  Error saving config: {e}");
                false
            }
        },
        Err(errors) => {
            println!("❌  Config validation failed. Changes not saved:");
            for error in errors {
                println!("  - {error}");
            }
            print!("Press any key to return to Edit Command Menu...");
            pause();
            false
        }
    }
}

pub fn add_command(config: &mut Config, changes_made: &mut bool) {
    let Some(display_name) = prompt_or_return(|| {
        inquire::Text::new("Enter the display name for the command:")
            .with_help_message("This name will be displayed in the menu")
            .prompt()
    }) else {
        return;
    };

    let Some(command) = prompt_or_return(|| {
        inquire::Text::new("Enter the command to execute:")
            .with_help_message("This command will be executed in the shell")
            .prompt()
    }) else {
        return;
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

    let Some(command_index) =
        prompt_or_return(|| Select::new("Select a command to edit:", command_names).prompt())
    else {
        return;
    };
    let Some(command_number) = selected_command_index(&command_index) else {
        println!("❌  Invalid choice, please try again.");
        return;
    };
    let Some(existing_command) = config.commands.get(command_number) else {
        println!("❌  Invalid choice, please try again.");
        return;
    };
    let current_display_name = existing_command.display_name.clone();
    let current_command = existing_command.command.clone();

    let Some(display_name) = prompt_or_return(|| {
        inquire::Text::new("Enter the new display name for the command:")
            .with_initial_value(&current_display_name)
            .prompt()
    }) else {
        return;
    };

    let Some(command) = prompt_or_return(|| {
        inquire::Text::new("Enter the new command to execute:")
            .with_initial_value(&current_command)
            .prompt()
    }) else {
        return;
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

    let Some(command_index) =
        prompt_or_return(|| Select::new("Select a command to reorder:", command_names).prompt())
    else {
        return;
    };

    let Some(command_number) = selected_command_index(&command_index) else {
        println!("❌  Invalid choice, please try again.");
        return;
    };

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

    let Some(command_index) =
        prompt_or_return(|| Select::new("Select a command to delete:", command_names).prompt())
    else {
        return;
    };

    let Some(command_number) = selected_command_index(&command_index) else {
        println!("❌  Invalid choice, please try again.");
        return;
    };

    if let Some(deleted) = delete_command_at(config, command_number, changes_made) {
        println!(
            "✅  Command '{}' deleted successfully.",
            deleted.display_name
        );
    }
}

fn selected_command_index(selection: &str) -> Option<usize> {
    selection
        .split('.')
        .next()?
        .trim()
        .parse::<usize>()
        .ok()?
        .checked_sub(1)
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
