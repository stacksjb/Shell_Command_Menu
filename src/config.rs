//Module to find common config path & load (or create config if needed)
use crate::commands::{CommandOption, Commands};
use anyhow::Context; // Importing context from the anyhow crate
use directories::BaseDirs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// Function to get the path of the config file; else create it
pub(crate) fn get_config_file_path() -> Result<PathBuf, String> {
    // Get the base directories for the current user
    let base_dirs = BaseDirs::new().ok_or("Could not get base directories")?;

    // Get the config directory and append the file name
    let config_file = base_dirs.config_dir().join("cli_menu_cmd.json");

    // Check if the config file exists
    if config_file.exists() {
        // If the file exists, print that it was loaded successfully
        println!(
            "✅  Config file loaded successfully from path: {:?}",
            config_file
        );
    } else {
        // If the file doesn't exist, create the default config
        println!(
            "⚠️  Config file not found. Creating new default config at: {:?}",
            config_file
        );
        create_default_config(&config_file)
            .map_err(|e| format!("Failed to create default config file: {}", e))?;
    }

    Ok(config_file)
}
// Loads commands from a file.
pub fn load_config(path: &PathBuf) -> anyhow::Result<Commands> {
    let config_data = std::fs::read_to_string(path).context("unable to read config file")?; // Loading config data from file
    serde_json::from_str(&config_data).context("unable to parse config file") // Parsing config data
}

// Save configuration to a file.
pub fn save_config(path: &Path, config: &Commands) {
    let config_data = serde_json::to_string_pretty(config).expect("❌  Failed to serialize config"); // Serializing config data
    let mut file = File::create(path).expect("❌  Unable to create config file"); // Creating or overwriting config file
    file.write_all(config_data.as_bytes())
        .expect("❌  Unable to write to config file"); // Writing config data to file
}

// Create a default list of stored commands.
fn default_config() -> Commands {
    let default_commands = vec![CommandOption {
        display_name: "Command 1".to_string(),
        command: "echo '1'".to_string(),
    }];

    Commands {
        commands: default_commands,
    }
}

// Creates a default list of stored commands.
fn create_default_config(path: &Path) -> anyhow::Result<Commands> {
    let default_config = default_config(); // Creating default commands

    save_config(path, &default_config); // Saving default commands to file
    println!("✅  Sucessfully created and saved new default config."); // Printing success message
    Ok(default_config) // Returning default commands
}
