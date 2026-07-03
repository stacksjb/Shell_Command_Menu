use crate::{
    config::{CommandOption, Config}, // Importing Config struct
    menu_edit::print_commands,
    utils::pause,
};
use inquire::{Select, Text}; // Importing prompts from inquire crate
use std::{env, fs, path::Path}; // Importing necessary modules from standard library // Importing functions and structs from other modules

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ImportStrategy {
    Append,
    Overwrite,
    Cancel,
}

impl ImportStrategy {
    fn from_label(label: &str) -> Self {
        match label {
            "a. APPEND to current commands" => Self::Append,
            "o. OVERWRITE current commands" => Self::Overwrite,
            _ => Self::Cancel,
        }
    }
}

/// Prompts for a CSV file and imports commands into the current config.
///
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

    let path = match Select::new("Select a file to import: ", files).prompt() {
        Ok(path) => path,
        Err(e) => {
            println!("⚠️ Import canceled: {e}");
            return;
        }
    };

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

    let menu_prompt = match Select::new("Select an option: ", menu_options).prompt() {
        Ok(choice) => choice,
        Err(e) => {
            println!("⚠️ Import canceled: {e}");
            return;
        }
    };
    let strategy = ImportStrategy::from_label(menu_prompt);

    merge_imported_commands(config, commands, strategy, changes_made);

    match strategy {
        ImportStrategy::Append => println!("{num_commands} commands appended."),
        ImportStrategy::Overwrite => {
            println!("Replaced config with {num_commands} commands from {path}");
        }
        ImportStrategy::Cancel => {
            println!("❌  Canceled import");
            pause();
        }
    }
}

/// Prompts for a CSV destination and exports the current commands.
pub fn export_commands(config: &Config) {
    let path = match Text::new("Enter the CSV file path to export to:")
        .with_initial_value("commands.csv")
        .prompt()
    {
        Ok(path) => path,
        Err(e) => {
            println!("⚠️ Export canceled: {e}");
            return;
        }
    };

    let path = path.trim();
    if path.is_empty() {
        println!("⚠️ Export canceled: no path provided.");
        return;
    }

    match write_commands_to_csv(path, &config.commands) {
        Ok(()) => println!("✅ Exported {} commands to {path}.", config.commands.len()),
        Err(e) => println!("❌ Could not export commands: {e}"),
    }
}

/// Reads command entries from a CSV file.
///
/// # Errors
///
/// Returns an error when the CSV file cannot be opened or a record cannot be
/// deserialized as a [`CommandOption`].
pub fn read_commands_from_csv<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<CommandOption>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut commands = Vec::new();

    for result in reader.deserialize() {
        let command: CommandOption = result?;
        commands.push(command);
    }

    Ok(commands)
}

/// Writes command entries to a CSV file.
///
/// # Errors
///
/// Returns an error when the CSV file cannot be created, written, or flushed.
pub fn write_commands_to_csv<P: AsRef<Path>>(
    path: P,
    commands: &[CommandOption],
) -> anyhow::Result<()> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    writer.write_record(["display_name", "command"])?;
    for command in commands {
        writer.serialize(command)?;
    }
    writer.flush()?;
    Ok(())
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
        if Path::new(&name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csv"))
        {
            files.push(name); // Adding CSV file names to the vector
        }
    }
    files.sort();
    Ok(files)
}

// Extracted for testable merging logic
fn merge_imported_commands(
    config: &mut Config,
    mut new_commands: Vec<CommandOption>,
    strategy: ImportStrategy,
    changes_made: &mut bool,
) {
    match strategy {
        ImportStrategy::Append => {
            config.commands.append(&mut new_commands);
            *changes_made = true;
        }
        ImportStrategy::Overwrite => {
            config.commands = new_commands;
            *changes_made = true;
        }
        ImportStrategy::Cancel => {}
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

        merge_imported_commands(&mut config, new, ImportStrategy::Append, &mut changed);

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

        merge_imported_commands(&mut config, new, ImportStrategy::Overwrite, &mut changed);

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
        let mut changed = true;

        merge_imported_commands(&mut config, new, ImportStrategy::Cancel, &mut changed);

        assert_eq!(config.commands.len(), 1);
        assert_eq!(config.commands[0].display_name, "Keep");
        assert!(changed);
    }

    #[test]
    fn test_merge_imported_commands_cancel_without_prior_changes_stays_clean() {
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

        merge_imported_commands(&mut config, new, ImportStrategy::Cancel, &mut changed);

        assert_eq!(config.commands.len(), 1);
        assert_eq!(config.commands[0].display_name, "Keep");
        assert!(!changed);
    }

    #[test]
    fn test_read_commands_from_csv_valid_csv() {
        let commands =
            read_commands_from_csv("tests/fixtures/commands.csv").expect("Should parse CSV");

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].display_name, "List Files");
        assert_eq!(commands[0].command, "ls -la");
    }

    #[test]
    fn test_read_commands_from_invalid_csv() {
        let result = read_commands_from_csv("tests/fixtures/invalid.csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_commands_from_empty_csv() {
        let commands =
            read_commands_from_csv("tests/fixtures/empty.csv").expect("Should parse empty CSV");
        assert!(commands.is_empty());
    }

    #[test]
    fn test_write_commands_to_csv_roundtrip() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        let commands = vec![CommandOption {
            display_name: "List Files".into(),
            command: "ls -la".into(),
        }];

        write_commands_to_csv(file.path(), &commands).expect("Should write CSV");
        let loaded = read_commands_from_csv(file.path()).expect("Should parse written CSV");

        assert_eq!(loaded, commands);
    }
}
