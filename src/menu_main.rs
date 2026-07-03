use crate::{
    config::CommandOption,
    utils::{get_version, play_sound, run_command},
};
use inquire::Select;
use std::{
    io::{Write, stdout},
    path::Path,
    process::exit,
};
use termion::{clear, cursor, terminal_size};

use crate::menu_edit::edit_menu;

use inquire::error::InquireError;

const EDIT_MENU_LABEL: &str = "e. EDIT Commands";
const EXIT_LABEL: &str = "q. EXIT";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MainMenuChoice {
    Command(usize),
    Edit,
    Quit,
}

pub fn prompt_or_return<T>(prompt: impl FnOnce() -> Result<T, InquireError>) -> Option<T> {
    match prompt() {
        Ok(val) => Some(val),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
            println!("⚠️  Canceled. Returning to Edit Menu...");
            None
        }
        Err(e) => {
            eprintln!("❌  Error: {e}");
            None
        }
    }
}

/// Displays the main interactive command menu.
///
pub async fn display_menu(config_path: &Path) {
    let mut selected_commands: Vec<usize> = vec![];
    let mut last_selected: Option<usize> = None;

    loop {
        let Ok(config) = crate::config::load_config(config_path) else {
            println!("⚠️ Config does not exist or is invalid; editing new config");
            edit_menu(config_path);
            selected_commands.clear();
            last_selected = None;
            continue;
        };

        let term_height = get_terminal_height() as usize;
        let display_height = term_height.saturating_sub(3);

        if config.window_title_support
            && let Some(title) = &config.window_title
        {
            set_window_title(title);
        }

        clear_screen();
        let mut menu_options = generate_menu(&config.commands, &selected_commands);

        menu_options.push(EDIT_MENU_LABEL.to_string());
        menu_options.push(EXIT_LABEL.to_string());

        let menu_prompt = if let Some(last) = last_selected {
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:",
                menu_options,
            )
            .with_starting_cursor(last)
            .with_page_size(display_height)
        } else {
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select a command to execute:",
                menu_options,
            )
            .with_page_size(display_height)
        };

        match menu_prompt.prompt() {
            Ok(choice) => match parse_main_menu_choice(&choice) {
                Some(MainMenuChoice::Quit) => {
                    println!("Exiting CLI Menu v{}...", get_version());
                    exit(0);
                }
                Some(MainMenuChoice::Edit) => {
                    edit_menu(config_path);
                    selected_commands.clear();
                    last_selected = None;
                }
                Some(MainMenuChoice::Command(num)) => {
                    let Some(index) = num.checked_sub(1) else {
                        println!("❌  Invalid choice, please try again.");
                        continue;
                    };
                    if let Some(command) = config.commands.get(index) {
                        if let Some(cmd_sound) = &config.cmd_sound {
                            tokio::spawn(play_sound(cmd_sound.clone()));
                        }
                        if config.window_title_support {
                            set_window_title(&choice);
                        }
                        if let Err(e) = run_command(&command.command) {
                            eprintln!("❌  Failed to run command: {e}");
                        }
                        selected_commands.push(num);
                        last_selected = Some(index);
                    } else {
                        println!("❌  Invalid choice, please try again.");
                    }
                }
                None => {
                    println!("❌  Invalid choice, please try again.");
                }
            },
            Err(_) => println!("❌  Error reading input. Please try again."),
        }
    }
}

fn parse_main_menu_choice(choice: &str) -> Option<MainMenuChoice> {
    match choice {
        EXIT_LABEL => Some(MainMenuChoice::Quit),
        EDIT_MENU_LABEL => Some(MainMenuChoice::Edit),
        _ => choice
            .split('.')
            .next()?
            .trim()
            .parse::<usize>()
            .ok()
            .map(MainMenuChoice::Command),
    }
}

#[must_use]
pub fn generate_menu(commands: &[CommandOption], selected_commands: &[usize]) -> Vec<String> {
    let max_number_width = commands.len().to_string().len();
    commands
        .iter()
        .enumerate()
        .map(|(index, cmd)| {
            let number = index + 1;
            let padded_number = format!("{number: >max_number_width$}");
            if selected_commands.contains(&number) {
                format!("{}. {}", padded_number, strike_through(&cmd.display_name))
            } else {
                format!("{}. {}", padded_number, &cmd.display_name)
            }
        })
        .collect()
}

#[must_use]
pub fn get_terminal_height() -> u16 {
    terminal_size().map_or(24, |(_, height)| height)
}

pub fn clear_screen() {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    if let Err(e) = stdout().flush() {
        eprintln!("❌  Failed to flush terminal output: {e}");
    }
}

fn strike_through(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(c);
        result.push('\u{0336}');
    }
    result
}

pub fn set_window_title(title: &str) {
    print!("\x1b]0;{title}\x07");
}
