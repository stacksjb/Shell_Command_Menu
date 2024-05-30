use anyhow::Context;
use serde::{Deserialize, Serialize};

/// A stored shell command.
#[derive(Clone, Deserialize, Serialize)]
pub struct CommandOption {
    /// The human-readable name for this command.
    pub display_name: String,
    /// The command to run.
    pub command: String,
}

/// A list of stored shell commands loaded from a file..
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Commands {
    pub commands: Vec<CommandOption>,
}

/// Loads stored commands from a file.
pub fn load_config(path: &str) -> anyhow::Result<Commands> {
    let config_data = std::fs::read_to_string(path).context("unable to read config file")?;
    serde_json::from_str(&config_data).context("unable to parse config file")
}

/// Saves stored commands to a file.
pub fn save_config(path: &str, config: &Commands) {
    let config_data = serde_json::to_string_pretty(config).expect("❌  Failed to serialize config");
    let mut file = std::fs::File::create(path).expect("❌  Unable to create config file");
    std::io::Write::write_all(&mut file, config_data.as_bytes())
        .expect("❌  Unable to write to config file");
}

fn default_config() -> Commands {
    let default_commands = vec![
        CommandOption {
            display_name: "Command 1".to_string(),
            command: "echo '1'".to_string(),
        },
    ];

    Commands {
        commands: default_commands,
    }
}

/// Creates a default list of stored commands.
pub fn create_default_config(path: &str) -> Commands {
    let default_config = default_config();

    save_config(path, &default_config);
    println!("✅  Creating new default config.");
    default_config
}
