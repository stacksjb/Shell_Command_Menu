use crate::edit::{edit_json_menu, reset_strikeout_state};
use crate::utils::{run_command, play_sound};
use inquire::Select;
use std::process::exit;
use term_size;

#[tokio::main]
pub async fn display_menu(config_path: &str) {
    let config = crate::config::load_config(config_path);
    let mut selected_commands: Vec<usize> = vec![];
    let mut last_selected: Option<usize> = None;

    // Helper function to apply strike-through effect
    fn strike_through(text: &str) -> String {
        text.chars().map(|c| format!("{}\u{0336}", c)).collect()
    }

    loop {
        // Determine the terminal height and set the page size accordingly
        let page_size = if let Some((_, height)) = term_size::dimensions() {
            height as usize - 2 // Leave some space for the prompt
        } else {
            10 // Fallback page size
        };

        // Create a list of menu options
        let mut menu_options: Vec<String> = config.commands.iter().map(|cmd| {
            if selected_commands.contains(&cmd.number) {
                format!("{}. {}", cmd.number, strike_through(&cmd.display_name))
            } else {
                format!("{}. {}", cmd.number, cmd.display_name)
            }
        }).collect();

        menu_options.push("e. Edit commands".to_string());
        menu_options.push("q. Exit".to_string());

        // Display the menu and prompt the user to select an option
        let menu_prompt = if let Some(last) = last_selected {
            Select::new("Welcome to the JSON Command Shortcut Menu! Select an option:", menu_options)
                .with_starting_cursor(last)
                .with_page_size(page_size)
        } else {
            Select::new("Welcome to the JSON Command Shortcut Menu! Select an option:", menu_options)
                .with_page_size(page_size)
        };

        match menu_prompt.prompt() {
            Ok(choice) => {
                if choice == "q. Exit" {
                    println!("Exiting...");
                    exit(0);
                } else if choice == "e. Edit commands" {
                    edit_json_menu(config_path);
                    // Reset strike-out state after editing
                    reset_strikeout_state(&mut selected_commands, &mut last_selected);
                } else {
                    let num: usize = choice.split('.').next().unwrap().parse().unwrap();
                    if let Some(command) = config.commands.iter().find(|cmd| cmd.number == num) {
                        tokio::spawn(play_sound("whoosh-6316.mp3"));
                        run_command(&command.command);
                        if !selected_commands.contains(&num) {
                            selected_commands.push(num);
                        }
                        last_selected = Some(config.commands.iter().position(|cmd| cmd.number == num).unwrap());
                    } else {
                        println!("Invalid choice, please try again.");
                    }
                }
            }
            Err(_) => println!("Error reading input. Please try again."),
        }
    }
}
