use crate::edit::edit_menu;
use crate::utils::{generate_menu, get_page_size, play_sound, run_command};
use inquire::Select;
use std::process::exit;

#[tokio::main]
pub async fn display_menu(config_path: &str) {
    let mut selected_commands: Vec<usize> = vec![];
    let mut last_selected: Option<usize> = None;
    loop {
        let config = crate::config::load_config(config_path);

        // Determine the terminal height and set the page size accordingly
        let page_size = get_page_size();
        // Create a list of menu options
        let mut menu_options = generate_menu(&config, &selected_commands);

        menu_options.push("e. EDIT commands".to_string());
        menu_options.push("q. EXIT".to_string());

        // Display the menu and prompt the user to select an option
        let menu_prompt = if let Some(last) = last_selected {
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select an option:",
                menu_options,
            )
            .with_starting_cursor(last)
            .with_page_size(page_size)
        } else {
            Select::new(
                "Welcome to the CLI Command Shortcut Menu! Select an option:",
                menu_options,
            )
            .with_page_size(page_size)
        };

        match menu_prompt.prompt() {
            Ok(choice) => {
                if choice == "q. EXIT" {
                    println!("Exiting...");
                    exit(0);
                } else if choice == "e. EDIT commands" {
                    edit_menu(config_path);
                    //Clear the selected commands and last selected index after editing
                    selected_commands.clear();
                    last_selected = None;
                    // Reload the config after editing
                    continue;
                } else {
                    let num: usize = choice.split('.').next().unwrap().parse().unwrap();
                    if let Some(index) = num.checked_sub(1) {
                        if let Some(command) = config.commands.get(index) {
                            tokio::spawn(play_sound("whoosh-6316.mp3"));
                            run_command(&command.command);
                            selected_commands.push(num);
                            last_selected = Some(index);
                        } else {
                            println!("Invalid choice, please try again.");
                        }
                    } else {
                        println!("Invalid choice, please try again.");
                    }
                }
            }
            Err(_) => println!("Error reading input. Please try again."),
        }
    }
}
