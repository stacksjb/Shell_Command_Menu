use crate::config::Commands;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::process::Command;
use tokio::task;

pub fn run_command(command: &str) {
    println!("Running command: {}", command);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Command wasn't running");

    if status.success() {
        println!("Command executed successfully.");
    } else {
        // Ring the terminal bell and print "Error" in red
        println!("\x07\x1b[31mError\x1b[0m: Command returned a non-zero exit status.");
    }
}

pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub async fn play_sound(file_path: &str) {
    let file_path = file_path.to_string(); // Clone the file_path to be owned by the closure
    task::spawn_blocking(move || {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(file) = File::open(&file_path) {
                if let Ok(source) = Decoder::new(BufReader::new(file)) {
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                } else {
                    println!("Failed to decode audio file: {}", file_path);
                }
            } else {
                println!("Failed to open audio file: {}", file_path);
            }
        } else {
            println!("Failed to initialize audio output stream");
        }
    })
    .await
    .unwrap();
}

pub fn generate_menu(config: &Commands, selected_commands: &[usize]) -> Vec<String> {
    let menu_options: Vec<String> = config
        .commands
        .iter()
        .enumerate()
        .map(|(index, cmd)| {
            let number = index + 1;
            if selected_commands.contains(&number) {
                format!("{}. {}", number, strike_through(&cmd.display_name))
            } else {
                format!("{}. {}", number, cmd.display_name)
            }
        })
        .collect();
    menu_options
}

fn strike_through(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(c);
        result.push('\u{0336}');
    }
    result
}

pub fn get_page_size() -> usize {
    if let Some((_, height)) = term_size::dimensions() {
        height - 2 // Leave some space for the prompt
    } else {
        10 // Fallback page size
    }
}
