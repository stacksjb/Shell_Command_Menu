use crate::{
    config::{CommandOption, Config}, // Importing Config struct
    menu_edit::print_commands,
    utils::pause,
};
use inquire::Select; // Importing Select prompt from inquire crate
use std::{env, fs, path::Path}; // Importing necessary modules from standard library // Importing functions and structs from other modules

// Function to import commands from CSV
pub fn import_commands(config: &mut Config, changes_made: &mut bool) {
    let dir = env::current_dir().unwrap_or_else(|_| ".".into());
    let files = match list_csv_files(&dir) {
        Ok(files) => files,
        Err(e) => {
            println!("⚠️ Could not list files: {e}");
            return;
        }
    };

    if files.is_empty() {
        println!("⚠️ No files found in {}", dir.display());
        return;
    }

    let path = Select::new("Select a file to import: ", files)
        .prompt()
        .expect("Failed to display menu");

    let commands = match read_commands_from_csv(&path) {
        Ok(commands) => commands,
        Err(e) => {
            println!("❌  Could not import file: {e}");
            return;
        }
    };

    let num_commands = commands.len();
    println!("Found {num_commands} commands from {path}");
    print_commands(&commands);

    let menu_options = vec![
        "a. APPEND to current commands",
        "o. OVERWRITE current commands",
        "c. CANCEL import",
    ];

    let menu_prompt = Select::new("Select an option: ", menu_options)
        .prompt()
        .expect("❌ Failed to display menu");

    let strategy = match menu_prompt {
        "a. APPEND to current commands" => "append",
        "o. OVERWRITE current commands" => "overwrite",
        _ => "cancel",
    };

    merge_imported_commands(config, commands, strategy, changes_made);

    match strategy {
        "append" => println!("{num_commands} commands appended."),
        "overwrite" => println!("Replaced config with {num_commands} commands from {path}"),
        _ => {
            println!("❌  Canceled import");
            pause();
        }
    }
}

// Function to read commands from a CSV file
pub fn read_commands_from_csv<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<CommandOption>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut commands = Vec::new();

    for result in reader.deserialize() {
        let command: CommandOption = result?;
        commands.push(command);
    }

    Ok(commands)
}

// Function to list CSV files in a directory
fn list_csv_files(dir_path: &Path) -> anyhow::Result<Vec<String>> {
    let dir_reader = fs::read_dir(dir_path)?; // Creating directory reader
    let mut files = Vec::new(); // Initializing vector to hold file names
    for entry in dir_reader {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue; // Skipping directories
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue; // Skipping invalid file names
        };
        if name.ends_with(".csv") {
            files.push(name); // Adding CSV file names to the vector
        }
    }
    Ok(files)
}

// Extracted for testable merging logic
fn merge_imported_commands(
    config: &mut Config,
    mut new_commands: Vec<CommandOption>,
    strategy: &str,
    changes_made: &mut bool,
) {
    match strategy {
        "append" => {
            config.commands.append(&mut new_commands);
            *changes_made = true;
        }
        "overwrite" => {
            config.commands = new_commands;
            *changes_made = true;
        }
        _ => {
            *changes_made = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_list_csv_files_returns_csvs_only() {
        use std::fs::{self, File};
        use std::path::PathBuf;

        let test_dir = PathBuf::from("tests/tmp_csvs");
        let _ = fs::create_dir_all(&test_dir);
        File::create(test_dir.join("file1.csv")).unwrap();
        File::create(test_dir.join("file2.txt")).unwrap();

        let csvs = list_csv_files(&test_dir).unwrap();
        assert_eq!(csvs.len(), 1);
        assert_eq!(csvs[0], "file1.csv");

        fs::remove_dir_all(&test_dir).unwrap();
    }
    #[test]
    fn test_merge_imported_commands_append() {
        let mut config = Config {
            commands: vec![CommandOption {
                display_name: "Existing".into(),
                command: "echo existing".into(),
            }],
            ..Default::default()
        };
        let new = vec![CommandOption {
            display_name: "New".into(),
            command: "echo new".into(),
        }];
        let mut changed = false;

        merge_imported_commands(&mut config, new, "append", &mut changed);

        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.commands[1].display_name, "New");
        assert!(changed);
    }

    #[test]
    fn test_merge_imported_commands_overwrite() {
        let mut config = Config {
            commands: vec![CommandOption {
                display_name: "Old".into(),
                command: "echo old".into(),
            }],
            ..Default::default()
        };
        let new = vec![CommandOption {
            display_name: "Overwrite".into(),
            command: "echo overwrite".into(),
        }];
        let mut changed = false;

        merge_imported_commands(&mut config, new, "overwrite", &mut changed);

        assert_eq!(config.commands.len(), 1);
        assert_eq!(config.commands[0].display_name, "Overwrite");
        assert!(changed);
    }

    #[test]
    fn test_merge_imported_commands_cancel() {
        let mut config = Config {
            commands: vec![CommandOption {
                display_name: "Keep".into(),
                command: "echo keep".into(),
            }],
            ..Default::default()
        };
        let new = vec![CommandOption {
            display_name: "ShouldNotAdd".into(),
            command: "echo nope".into(),
        }];
        let mut changed = false;

        merge_imported_commands(&mut config, new, "cancel", &mut changed);

        assert_eq!(config.commands.len(), 1);
        assert_eq!(config.commands[0].display_name, "Keep");
        assert!(!changed);
    }
    #[test]
    fn test_read_commands_from_csv_valid_csv() {
        let path = "tests/fixtures/commands.csv"; // Must be a real file with no prompts
        let commands = read_commands_from_csv(path).expect("Should parse CSV");

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].display_name, "List Files");
        assert_eq!(commands[0].command, "ls -la");
    }

    #[test]
    fn test_read_commands_from_invalid_csv() {
        let path = "tests/fixtures/invalid.csv"; // Create this with garbage data
        let result = read_commands_from_csv(path);
        assert!(result.is_err());
    }
}
