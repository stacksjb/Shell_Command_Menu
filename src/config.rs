use anyhow::Context; // Importing context from the anyhow crate
use directories::BaseDirs;
use inquire::Text;
use serde::{Deserialize, Serialize}; // For serializing/deserializing config
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// Define the Config struct with multiple sections
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub commands: Vec<CommandOption>, // The commands section
    pub cmd_sound: Option<PathBuf>,   // The command sound section
    pub window_title: Option<String>, // The window title section
}

// Define the CommandOption struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandOption {
    pub display_name: String,
    pub command: String,
}

// Function to get the path of the config file; else create it
pub(crate) fn get_config_file_path() -> Result<PathBuf, String> {
    let base_dirs = BaseDirs::new().ok_or("Could not get base directories")?;

    // Get the config directory and append the file name
    let config_file = base_dirs.config_dir().join("cli_menu_cmd.json");

    if config_file.exists() {
        // Load the config for validation
        let config = crate::config::load_config(&config_file)
            .map_err(|e| format!("Failed to load config for validation: {}", e))?;

        // Validate the JSON structure
        if validate_json(&config) {
            println!(
                "✅  Config file loaded successfully from path: {:?}",
                config_file
            );
        }
    } else {
        println!(
            "⚠️  Config file not found. Creating new default config at: {:?}",
            config_file
        );
        create_default_config(&config_file)
            .map_err(|e| format!("Failed to create default config file: {}", e))?;
    }

    Ok(config_file)
}

// Loads config (multiple sections, including commands) from a file.
pub fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    let config_data = std::fs::read_to_string(path).context("unable to read config file")?; // Loading config data
    serde_json::from_str(&config_data).context("unable to parse config file") // Parsing config into the Config struct
}

// Save the entire configuration (including commands and other sections) to a file.
pub fn save_config(path: &Path, config: &Config) {
    let config_data = serde_json::to_string_pretty(config).expect("❌  Failed to serialize config"); // Serializing config data
    let mut file = File::create(path).expect("❌  Unable to create config file"); // Creating or overwriting the config file
    file.write_all(config_data.as_bytes())
        .expect("❌  Unable to write to config file"); // Writing config data to file
}

// Create a default configuration with stored commands.
fn default_config() -> Config {
    let default_commands = vec![CommandOption {
        display_name: "Command 1".to_string(),
        command: "echo '1'".to_string(),
    }];

    Config {
        commands: default_commands,
        cmd_sound: None,
        window_title: Some("CLI_Menu".to_string()),
        // Add default values for other sections here if needed
    }
}

// Creates and saves a default config.
fn create_default_config(path: &Path) -> anyhow::Result<Config> {
    let default_config = default_config(); // Creating default config
    save_config(path, &default_config); // Saving default config to file
    println!("✅  Successfully created and saved new default config.");
    Ok(default_config) // Returning default config
}

// Function to validate JSON config file
pub fn validate_json(config: &Config) -> bool {
    serde_json::to_string(config).is_ok() // Convert Config to JSON and check if it succeeds
}

// Function to edit the cmd_sound path
pub fn edit_cmd_sound(config: &mut Config, changes_made: &mut bool) {
    // Get the current value of cmd_sound
    let current_sound = config
        .cmd_sound
        .as_ref()
        .map_or(String::new(), |path| path.display().to_string());

    // Prompt user to enter a new path or clear the existing one
    println!("Current sound file: {}", current_sound);

    let sound_path = Text::new("Enter the new path for cmd_sound (leave empty to clear):")
        .with_initial_value(&current_sound)
        .prompt()
        .expect("Failed to read input");

    if sound_path.is_empty() {
        config.cmd_sound = None; // Clear the cmd_sound path
        println!("✅ cmd_sound cleared.");
    } else {
        //Trim the whitespace
        let sound_path = sound_path.trim();
        // Update the cmd_sound path
        config.cmd_sound = Some(PathBuf::from(sound_path)); // Update the cmd_sound path
        println!(
            "✅ cmd_sound updated to: {}",
            config.cmd_sound.as_ref().unwrap().display()
        );
    }

    *changes_made = true; // Mark changes as made
}

// Function to edit the window title
pub fn edit_window_title(config: &mut Config, changes_made: &mut bool) {
    // Get the current value of window_title
    let current_title = config
        .window_title
        .as_ref()
        .map_or(String::new(), |title| title.clone());

    // Prompt user to enter a new window title or clear the existing one
    println!("Current window title: {}", current_title);

    let new_title = Text::new("Enter the new window title (leave empty to clear):")
        .with_initial_value(&current_title)
        .prompt()
        .expect("Failed to read input");

    if new_title.is_empty() {
        config.window_title = None; // Clear the window title
        println!("✅ Window title cleared.");
    } else {
        //Trim the whitespace
        let new_title = new_title.trim();
        // Update the window title
        config.window_title = Some(new_title.to_string()); // Update the window title
        println!("✅ Window title updated to: {}", new_title);
    }

    *changes_made = true; // Mark changes as made
}
