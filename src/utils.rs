use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::path::PathBuf;
use std::process::Command; // Importing Command struct for executing shell commands // Importing types for audio playback
use termion::{input::TermRead, raw::IntoRawMode}; // Importing IntoRawMode trait for entering raw mode
use tokio::task; // Importing task module from Tokio for asynchronous task handling // Importing stdout, stdin, and Write traits for I/O operations

//This file contains the utility functions used in the project to run shell commands and other misc functions.
// Function to run a shell command
pub fn run_command(command: &str) {
    println!("Running command: {}", command); // Printing the command being executed
    let mut child = Command::new("sh") // Starting a new shell command
        .arg("-c") // Passing a command to the shell
        .arg(command) // The command to execute
        .spawn() // Starting the command asynchronously
        .expect("❌ Failed to execute command"); // Handling any errors

    let status = child.wait().expect("Command wasn't running"); // Waiting for the command to finish

    if status.success() {
        // Checking if the command was successful
        println!("✅ Command executed successfully."); // Printing success message
    } else {
        println!("\x07\x1b[31mError\x1b[0m: Command returned a non-zero exit status.");
        // Printing error message
    }
}

// Function to pause execution until user input is received
pub fn pause() {
    let mut stdout = stdout().into_raw_mode().unwrap(); // Entering raw mode for stdout
    stdout.flush().unwrap(); // Flushing stdout
    stdin().events().next(); // Waiting for user input
}

// Function to play a sound asynchronously from filepath
pub async fn play_sound(file_path: PathBuf) {
    let file_path = file_path.clone(); // Cloning the PathBuf to be owned by the closure
    task::spawn_blocking(move || {
        // Spawning a blocking task
        match OutputStream::try_default() {
            Ok((_stream, stream_handle)) => {
                // Trying to get the default audio output stream
                match File::open(&file_path) {
                    Ok(file) => {
                        // Trying to open the audio file
                        match Decoder::new(BufReader::new(file)) {
                            Ok(source) => {
                                // Trying to decode the audio file
                                let sink = Sink::try_new(&stream_handle).unwrap(); // Creating a sink for the audio stream
                                sink.append(source); // Appending the audio source to the sink
                                sink.sleep_until_end(); // Sleeping until the audio playback ends
                            }
                            _ => {
                                println!("❌ Failed to decode audio file: {:?}", file_path);
                                // Printing error message if decoding fails
                            }
                        }
                    }
                    _ => {
                        println!("❌ Failed to open audio file: {:?}", file_path);
                        // Printing error message if file opening fails
                    }
                }
            }
            _ => {
                println!("❌ Failed to initialize audio output stream"); // Printing error message if audio stream initialization fails
            }
        }
    })
    .await
    .unwrap(); // Waiting for the task to finish and handling any errors
}
