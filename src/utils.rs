use crate::config::Commands; // Importing Commands struct from the config module
use inquire::Text; // Importing Text prompt from the inquire crate
use rodio::{Decoder, OutputStream, Sink}; // Importing types for audio playback
use std::fs::File; // Importing File struct for file operations
use std::io::BufReader; // Importing BufReader for buffered reading from files
use std::process::Command; // Importing Command struct for executing shell commands
use termion::input::TermRead; // Importing TermRead trait for reading input events
use tokio::task; // Importing task module from Tokio for asynchronous task handling
use termion::raw::IntoRawMode; // Importing IntoRawMode trait for entering raw mode
use std::io::{stdout, stdin, Write}; // Importing stdout, stdin, and Write traits for I/O operations

// Function to run a shell command
pub fn run_command(command: &str) {
    println!("Running command: {}", command); // Printing the command being executed
    let mut child = Command::new("sh") // Starting a new shell command
        .arg("-c") // Passing a command to the shell
        .arg(command) // The command to execute
        .spawn() // Starting the command asynchronously
        .expect("❌ Failed to execute command"); // Handling any errors

    let status = child.wait().expect("Command wasn't running"); // Waiting for the command to finish

    if status.success() { // Checking if the command was successful
        println!("✅ Command executed successfully."); // Printing success message
    } else {
        println!("\x07\x1b[31mError\x1b[0m: Command returned a non-zero exit status."); // Printing error message
    }
}

// Function to prompt the user for input
pub fn prompt(message: &str) -> String {
    Text::new(message) // Creating a new Text prompt with the provided message
        .prompt() // Displaying the prompt and waiting for user input
        .expect("❌ Failed to display prompt") // Handling any errors
}

// Function to pause execution until user input is received
pub fn pause() {
    let mut stdout = stdout().into_raw_mode().unwrap(); // Entering raw mode for stdout
    stdout.flush().unwrap(); // Flushing stdout
    stdin().events().next(); // Waiting for user input
}

// Function to play a sound asynchronously
pub async fn play_sound(file_path: &str) {
    let file_path = file_path.to_string(); // Cloning the file_path to be owned by the closure
    task::spawn_blocking(move || { // Spawning a blocking task
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() { // Trying to get the default audio output stream
            if let Ok(file) = File::open(&file_path) { // Trying to open the audio file
                if let Ok(source) = Decoder::new(BufReader::new(file)) { // Trying to decode the audio file
                    let sink = Sink::try_new(&stream_handle).unwrap(); // Creating a sink for the audio stream
                    sink.append(source); // Appending the audio source to the sink
                    sink.sleep_until_end(); // Sleeping until the audio playback ends
                } else {
                    println!("❌ Failed to decode audio file: {}", file_path); // Printing error message if decoding fails
                }
            } else {
                println!("❌ Failed to open audio file: {}", file_path); // Printing error message if file opening fails
            }
        } else {
            println!("❌ Failed to initialize audio output stream"); // Printing error message if audio stream initialization fails
        }
    })
    .await
    .unwrap(); // Waiting for the task to finish and handling any errors
}

// Function to generate a menu based on provided commands and selected commands
pub fn generate_menu(config: &Commands, selected_commands: &[usize]) -> Vec<String> {
    let max_number_width = config.commands.len().to_string().len(); // Calculating the width of the maximum number
    let menu_options: Vec<String> = config // Generating menu options
        .commands
        .iter()
        .enumerate()
        .map(|(index, cmd)| {
            let number = index + 1; // Getting the number of the command
            let padded_number = format!("{: >width$}", number, width = max_number_width); // Padding the number with spaces
            if selected_commands.contains(&number) { // Checking if the command is selected
                format!("{}. {}", padded_number, strike_through(&cmd.display_name)) // Striking through the command name if selected
            } else {
                format!("{}. {}", padded_number, cmd.display_name) // Otherwise, displaying the command name
            }
        })
        .collect(); // Collecting menu options into a vector
    menu_options // Returning the generated menu options
}

// Function to strike through text
fn strike_through(text: &str) -> String {
    let mut result = String::new(); // Initializing an empty string to hold the result
    for c in text.chars() { // Iterating over each character in the text
        result.push(c); // Appending the character to the result string
        result.push('\u{0336}'); // Adding a Unicode character for strike-through
    }
    result // Returning the resulting string with strike-through
}

// Function to get the page size for menu display
pub fn get_page_size() -> usize {
    if let Some((_, height)) = term_size::dimensions() { // Getting terminal dimensions
        height - 2 // Subtracting 2 for leaving space for the prompt
    } else {
        10 // Fallback page size if terminal dimensions cannot be determined
    }
}
