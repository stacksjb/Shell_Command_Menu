use std::fs::{self, File};
use std::process::{Command, exit};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Deserialize, Serialize)]
struct Config {
    commands: Vec<(String, String)>,
}

fn main() {
    let config_path = "commands.json";
    if !std::path::Path::new(config_path).exists() {
        create_default_config(config_path);
    }

    let config_data = fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("Unable to parse config file");

    let mut last_choice: Option<usize> = None;

    loop {
        // Clear the screen before printing the menu
        print!("{}[2J", 27 as char);
        println!("Menu:");
        for (i, (key, _)) in config.commands.iter().enumerate() {
            println!("{}. Run command for {}", i + 1, key);
        }
        println!("q. Exit");

        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let _stdout = io::stdout().into_raw_mode().unwrap();
        let choice = stdin.keys().next().unwrap().unwrap();

        match choice {
            termion::event::Key::Char('q') => {
                drop(_stdout); // Ensure the terminal is restored before printing the exit message
                println!("Exiting...");
                exit(0);
            }
            termion::event::Key::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap() as usize;
                if num > 0 && num <= config.commands.len() {
                    last_choice = Some(num);
                    let command = &config.commands[num - 1].1;
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
                        let command = &config.commands[last - 1].1;
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

fn run_command(command: &str) {
    println!("Running command: {}", command);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");

    let _ = child.wait().expect("Command wasn't running");
}

fn create_default_config(path: &str) {
    let default_commands = vec![
        ("default_option".to_string(), "echo 'Default command'".to_string())
    ];

    let default_config = Config {
        commands: default_commands,
    };

    let config_data = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default config");
    let mut file = File::create(path).expect("Unable to create config file");
    file.write_all(config_data.as_bytes()).expect("Unable to write to config file");
}
