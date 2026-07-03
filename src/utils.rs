use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::{BufReader, Write, stdin, stdout};
use std::path::PathBuf;
use std::process::{Command, ExitStatus}; // Importing Command struct for executing shell commands // Importing types for audio playback
use termion::{input::TermRead, raw::IntoRawMode}; // Importing IntoRawMode trait for entering raw mode
use tokio::task; // Importing task module from Tokio for asynchronous task handling // Importing stdout, stdin, and Write traits for I/O operations

//This file contains the utility functions used in the project to run shell commands and other misc functions.
pub trait CommandExecutor {
    /// Executes a shell command and returns its exit status.
    ///
    /// # Errors
    ///
    /// Returns an error when the command cannot be spawned or waited on.
    fn execute(&mut self, command: &str) -> anyhow::Result<ExitStatus>;
}

pub struct ShellCommandExecutor;

impl CommandExecutor for ShellCommandExecutor {
    fn execute(&mut self, command: &str) -> anyhow::Result<ExitStatus> {
        execute_command(command)
    }
}

/// Runs a shell command and prints the result, capturing stdout/stderr for cleaner output.
///
/// # Errors
///
/// Returns an error when the shell cannot be spawned or the command status
/// cannot be collected.
pub fn run_command(command: &str) -> anyhow::Result<ExitStatus> {
    let mut executor = ShellCommandExecutor;
    run_command_with(command, &mut executor)
}

/// Runs a shell command through the supplied executor.
///
/// # Errors
///
/// Returns an error when the executor cannot run the command.
pub fn run_command_with(
    command: &str,
    executor: &mut impl CommandExecutor,
) -> anyhow::Result<ExitStatus> {
    println!("Running command: {command}"); // Printing the command being executed
    let status = executor.execute(command)?;

    if status.success() {
        // Checking if the command was successful
        println!("✅ Command executed successfully."); // Printing success message
    } else {
        println!("\x07\x1b[31mError\x1b[0m: Command returned a non-zero exit status.");
        // Printing error message
    }

    Ok(status)
}

/// Executes a shell command and returns its exit status.
///
/// # Errors
///
/// Returns an error when the shell cannot be spawned or the command status
/// cannot be collected.
pub fn execute_command(command: &str) -> anyhow::Result<ExitStatus> {
    let mut child = Command::new("sh").arg("-c").arg(command).spawn()?;

    Ok(child.wait()?)
}

// Function to pause execution until user input is received
pub fn pause() {
    let Ok(mut stdout) = stdout().into_raw_mode() else {
        eprintln!("❌  Failed to enter raw terminal mode.");
        return;
    };
    if let Err(e) = stdout.flush() {
        eprintln!("❌  Failed to flush terminal output: {e}");
    }
    stdin().events().next(); // Waiting for user input
}

// Function to play a sound asynchronously from filepath
pub async fn play_sound(file_path: PathBuf) {
    if let Err(e) = task::spawn_blocking(move || {
        // Spawning a blocking task

        match DeviceSinkBuilder::open_default_sink() {
            Ok(mut stream_handle) => {
                // Trying to get the default audio output stream
                stream_handle.log_on_drop(false); // Set log_on_drop to false
                match File::open(&file_path) {
                    Ok(file) => {
                        // Trying to open the audio file
                        match Decoder::new(BufReader::new(file)) {
                            Ok(source) => {
                                // Trying to decode the audio file
                                let sink = Player::connect_new(stream_handle.mixer()); // Creating a sink for the audio stream
                                sink.append(source); // Appending the audio source to the sink
                                sink.sleep_until_end(); // Sleeping until the audio playback ends
                            }
                            _ => {
                                println!("❌ Failed to decode audio file: {}", file_path.display());
                                // Printing error message if decoding fails
                            }
                        }
                    }
                    _ => {
                        println!("❌ Failed to open audio file: {}", file_path.display());
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
    {
        eprintln!("❌ Audio playback task failed: {e}");
    }
}

// Function to return the current version
#[must_use]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    // Importing serial_test crate for running tests serially for command execution tests

    #[cfg(unix)]
    struct FakeExecutor {
        status_code: i32,
        commands: Vec<String>,
    }

    #[cfg(unix)]
    impl CommandExecutor for FakeExecutor {
        fn execute(&mut self, command: &str) -> anyhow::Result<ExitStatus> {
            use std::os::unix::process::ExitStatusExt;

            self.commands.push(command.to_string());
            Ok(ExitStatus::from_raw(self.status_code << 8))
        }
    }

    #[test]
    #[serial]
    fn test_run_command_success() {
        let status = run_command("echo 'Hello'").expect("command should run");
        assert!(status.success());
    }

    #[test]
    #[serial]
    fn test_run_command_failure() {
        let status = run_command("non_existent_command_hopefully").expect("shell should run");
        assert!(!status.success());
    }

    #[cfg(unix)]
    #[test]
    fn test_run_command_with_uses_injected_executor() {
        let mut executor = FakeExecutor {
            status_code: 0,
            commands: Vec::new(),
        };

        let status = run_command_with("echo fake", &mut executor).expect("command should run");

        assert!(status.success());
        assert_eq!(executor.commands, vec!["echo fake"]);
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
