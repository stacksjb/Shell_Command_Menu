use crate::edit::edit_json_menu;
use crate::utils::run_command;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process::exit;
use std::io::Write; // Import std::io::Write

pub fn display_menu(config_path: &str) {
    let config = crate::config::load_config(config_path);

    let mut last_choice: Option<usize> = None;
    // Clear the screen before printing the menu
    print!("{}[2J", 27 as char);
    println!("Welcome to the JSON Command Shortcut Menu!");
    loop {
        println!("Menu:");
        for option in &config.commands {
            println!("{}. {}", option.number, option.display_name);
        }
        println!("q. Exit");

        print!("Please enter your choice: ");
        std::io::stdout().flush().unwrap();

        let stdin = std::io::stdin();
        let _stdout = std::io::stdout().into_raw_mode().unwrap();
        let choice = stdin.keys().next().unwrap().unwrap();

        match choice {
            termion::event::Key::Char('q') | termion::event::Key::Esc => {
                drop(_stdout); // Ensure the terminal is restored before printing the exit message
                println!("Exiting...");
                exit(0);
            }
            termion::event::Key::Ctrl('e') => {
                drop(_stdout); // Ensure the terminal is restored before switching menus
                edit_json_menu(config_path);
            }
            termion::event::Key::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap() as usize;
                if num > 0 && num <= config.commands.len() {
                    last_choice = Some(num);
                    let command = &config.commands[num - 1].command;
                    drop(_stdout); // Ensure the terminal is restored before running the command
                    run_command(command);
                } else {
                    drop(_stdout); // Ensure the terminal is restored before printing the invalid choice message
                    println!("Invalid choice, please try again.");
                }
            }
            termion::event::Key::Char('\n') => {
                if let Some(last) = last_choice {
                    if last <= config.commands.len() {
                        let command = &config.commands[last - 1].command;
                        drop(_stdout); // Ensure the terminal is restored before running the command
                        println!("Re-running last command: {}", command);
                        run_command(command);
                    } else {
                        drop(_stdout); // Ensure the terminal is restored before printing the invalid last choice message
                        println!("Invalid last choice.");
                    }
                } else {
                    drop(_stdout); // Ensure the terminal is restored before printing the no previous choice message
                    println!("No previous choice. Please make a selection.");
                }
            }
            _ => {
                drop(_stdout); // Ensure the terminal is restored before printing the invalid choice message
                println!("Invalid choice, please try again.");
            }
        }
    }
}
