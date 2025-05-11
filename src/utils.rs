use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Write, stdin, stdout};
use std::path::PathBuf;
use std::process::Command; // Importing Command struct for executing shell commands // Importing types for audio playback
use termion::{input::TermRead, raw::IntoRawMode}; // Importing IntoRawMode trait for entering raw mode
use tokio::task; // Importing task module from Tokio for asynchronous task handling // Importing stdout, stdin, and Write traits for I/O operations

//This file contains the utility functions used in the project to run shell commands and other misc functions.
// Function to run a shell command
/// Runs a shell command and prints the result, capturing stdout/stderr for cleaner output.
pub fn run_command(command: &str) {
    println!("Running command: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("❌ Failed to execute command");

    if output.status.success() {
        println!("✅ Command executed successfully.");
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("\x07\x1b[31mError\x1b[0m: {}", stderr.trim());
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    // Importing serial_test crate for running tests serially for command execution tests

    #[test]
    #[serial]
    fn test_run_command_success() {
        run_command("echo 'Hello'");
        println!();
    }

    #[test]
    #[serial]
    fn test_run_command_failure() {
        run_command("non_existent_command_hopefully");
        println!();
    }

    #[tokio::test]
    #[serial]
    async fn test_play_sound() {
        let sound_path = PathBuf::from("path/to/sound/file.wav");
        play_sound(sound_path).await;
        println!();
    }

    #[tokio::test]
    #[serial]
    async fn test_play_sound_invalid() {
        let sound_path = PathBuf::from("invalid/path/to/sound/file.wav");
        play_sound(sound_path).await;
        println!();
    }

    #[tokio::test]
    #[serial]
    async fn test_play_sound_invalid_path() {
        let fake_path = PathBuf::from("nonexistent_audio_file.mp3");
        play_sound(fake_path).await;
        println!();
    }

    #[tokio::test]
    #[serial]
    async fn test_play_sound_valid_file() {
        let path = PathBuf::from("assets/silence.wav"); // Blank Sound File for testing
        play_sound(path).await;
        println!();
    }
}
