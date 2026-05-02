use anyhow::Context; // Importing context from the anyhow crate
use directories::BaseDirs;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize}; // For serializing/deserializing config
use std::fs;
use std::path::{Path, PathBuf};

// Define the Config struct with multiple sections
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(default)] // Default values for the struct
#[serde(rename_all = "camelCase")] // Rename fields to camelCase in JSON
pub struct Config {
    pub commands: Vec<CommandOption>, // The commands section
    pub cmd_sound: Option<PathBuf>,   // The command sound section
    pub window_title_support: bool,   // The window title support - disabled by default
    pub window_title: Option<String>, // The window title section
}

// Define the CommandOption struct
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct CommandOption {
    pub display_name: String,
    pub command: String,
}

// Function to get the path of the config file; else create it
pub fn get_config_file_path() -> Result<PathBuf, String> {
    let base_dirs = BaseDirs::new().ok_or("Could not get base directories")?;

    // Get the config directory and append the file name
    let config_file = base_dirs.config_dir().join("cli_menu_cmd.json");

    if config_file.exists() {
        // Load the config for validation
        let config = crate::config::load_config(&config_file)
            .map_err(|e| format!("Failed to load config for validation: {e}"))?;

        // Validate the JSON structure
        if validate_json(&config) {
            println!("✅  Config file loaded successfully from path: {config_file:?}");
        }
    } else {
        println!("⚠️  Config file not found. Creating new default config at: {config_file:?}");
        create_default_config(&config_file)
            .map_err(|e| format!("Failed to create default config file: {e}"))?;
    }

    Ok(config_file)
}

// Loads config (multiple sections, including commands) from a file.
pub fn load_config(path: &Path) -> anyhow::Result<Config> {
    let config_data = std::fs::read_to_string(path)
        .with_context(|| format!("unable to load config file located at {}", path.display()))?;
    let config: Config = serde_json::from_str(&config_data).context("unable to parse config")?;
    Ok(config)
}

// Save the entire configuration (including commands and other sections) to a file.
pub fn save_config(path: &Path, config: &Config) -> anyhow::Result<()> {
    let config_data = serde_json::to_string_pretty(config).context("failed to serialize config")?;
    fs::write(path, config_data)
        .with_context(|| format!("unable to write config file at {}", path.display()))?;
    Ok(())
}

// Saves a default config.
fn create_default_config(path: &Path) -> anyhow::Result<Config> {
    let default_config = Config::default();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "unable to create config directory located at {}",
                parent.display()
            )
        })?;
    }
    save_config(path, &default_config)?;
    println!("✅  Successfully created and saved new default config.");
    Ok(default_config)
}

// Function to validate JSON config file
pub fn validate_json(config: &Config) -> bool {
    serde_json::to_string(config).is_ok()
}

// Function to edit the cmd_sound path
pub fn edit_cmd_sound(config: &mut Config, changes_made: &mut bool) {
    let current_sound = config
        .cmd_sound
        .as_ref()
        .map_or(String::new(), |path| path.display().to_string());

    println!("Current sound file: {current_sound}");

    let sound_path = Text::new("Enter the new path for cmd_sound (leave empty to clear):")
        .with_initial_value(&current_sound)
        .prompt()
        .expect("Failed to read input");

    if sound_path.is_empty() {
        config.cmd_sound = None; // Clear the cmd_sound path
        println!("✅ cmd_sound cleared.");
    } else {
        let sound_path = sound_path.trim();
        config.cmd_sound = Some(PathBuf::from(sound_path));
        println!(
            "✅ cmd_sound updated to: {}",
            config.cmd_sound.as_ref().unwrap().display()
        );
    }

    *changes_made = true; // Mark changes as made
}

// Function to edit the window title
pub fn edit_window_title(config: &mut Config, changes_made: &mut bool) {
    let enable_title_support = Select::new("Enable window title support?", vec!["Yes", "No"])
        .prompt()
        .expect("Failed to read input");

    if enable_title_support == "Yes" {
        println!("✅ Window title support enabled.");
    } else {
        apply_window_title_settings(config, false, None, changes_made);
        println!("✅ Window title support disabled.");
        return;
    }

    let current_title = config
        .window_title
        .as_ref()
        .map_or(String::new(), |title| title.clone());

    println!("Current window title: {current_title}");

    let new_title = Text::new("Enter the new window title (leave empty to clear):")
        .with_initial_value(&current_title)
        .prompt()
        .expect("Failed to read input");

    apply_window_title_settings(config, true, Some(&new_title), changes_made);

    if let Some(title) = &config.window_title {
        println!("✅ Window title updated to: {title}");
    } else {
        println!("✅ Window title cleared, set to default.");
    }
}

pub fn apply_window_title_settings(
    config: &mut Config,
    enable_title_support: bool,
    new_title: Option<&str>,
    changes_made: &mut bool,
) {
    let original_title_support = config.window_title_support;
    let original_title = config.window_title.clone();

    config.window_title_support = enable_title_support;
    if enable_title_support {
        config.window_title = new_title.and_then(|title| {
            let title = title.trim();
            if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            }
        });
    }

    if config.window_title_support != original_title_support
        || config.window_title != original_title
    {
        *changes_made = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_is_empty() {
        let config = Config::default();
        assert!(config.commands.is_empty());
        assert!(config.cmd_sound.is_none());
        assert!(!config.window_title_support);
        assert!(config.window_title.is_none());
    }

    #[test]
    fn test_save_and_load_config_roundtrip() {
        use tempfile::NamedTempFile;

        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let original = Config {
            commands: vec![CommandOption {
                display_name: "Test".into(),
                command: "echo test".into(),
            }],
            cmd_sound: Some("sound.mp3".into()),
            window_title_support: true,
            window_title: Some("My CLI Menu".into()),
        };

        save_config(&path, &original).expect("Should save config");

        let loaded = load_config(&path).expect("Should load config");
        assert_eq!(original.commands.len(), loaded.commands.len());
        assert_eq!(original.cmd_sound, loaded.cmd_sound);
        assert_eq!(original.window_title, loaded.window_title);
        assert_eq!(original.window_title_support, loaded.window_title_support);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_create_default_config_creates_parent_directory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("cli_menu_cmd.json");

        let config = create_default_config(&path).expect("Should create default config");

        assert_eq!(config, Config::default());
        assert!(path.exists());
    }
    #[test]
    fn test_validate_json_returns_true_for_valid_config() {
        let config = Config::default();
        assert!(validate_json(&config));
    }

    #[test]
    fn test_apply_window_title_disable_without_change_stays_clean() {
        let mut config = Config::default();
        let mut changed = false;

        apply_window_title_settings(&mut config, false, None, &mut changed);

        assert!(!config.window_title_support);
        assert!(!changed);
    }

    #[test]
    fn test_apply_window_title_disable_existing_support_marks_changed() {
        let mut config = Config {
            window_title_support: true,
            window_title: Some("CLI Menu".into()),
            ..Default::default()
        };
        let mut changed = false;

        apply_window_title_settings(&mut config, false, None, &mut changed);

        assert!(!config.window_title_support);
        assert_eq!(config.window_title.as_deref(), Some("CLI Menu"));
        assert!(changed);
    }

    #[test]
    fn test_apply_window_title_same_title_stays_clean() {
        let mut config = Config {
            window_title_support: true,
            window_title: Some("CLI Menu".into()),
            ..Default::default()
        };
        let mut changed = false;

        apply_window_title_settings(&mut config, true, Some("CLI Menu"), &mut changed);

        assert_eq!(config.window_title.as_deref(), Some("CLI Menu"));
        assert!(!changed);
    }

    #[test]
    fn test_apply_window_title_change_marks_changed() {
        let mut config = Config {
            window_title_support: true,
            window_title: Some("CLI Menu".into()),
            ..Default::default()
        };
        let mut changed = false;

        apply_window_title_settings(&mut config, true, Some("New Title"), &mut changed);

        assert_eq!(config.window_title.as_deref(), Some("New Title"));
        assert!(changed);
    }

    #[test]
    fn test_apply_window_title_empty_title_clears_title() {
        let mut config = Config {
            window_title_support: true,
            window_title: Some("CLI Menu".into()),
            ..Default::default()
        };
        let mut changed = false;

        apply_window_title_settings(&mut config, true, Some("  "), &mut changed);

        assert!(config.window_title.is_none());
        assert!(changed);
    }

    #[test]
    fn test_get_config_file_path_returns_path() {
        let path = get_config_file_path().expect("Should return a config path");
        assert!(path.ends_with("cli_menu_cmd.json"));
    }
}
