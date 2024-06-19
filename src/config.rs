use anyhow::Context; // Importing context from the anyhow crate
use prettytable::{row, Cell, Row, Table}; // Importing types for creating tables
use serde::{Deserialize, Serialize}; // Importing serialization and deserialization traits
use textwrap::fill; // Importing fill function for text wrapping

/// A stored shell command.
#[derive(Clone, Deserialize, Serialize)]
pub struct CommandOption {
    /// The human-readable name for this command.
    pub display_name: String,
    /// The command to run.
    pub command: String,
}

/// A list of stored shell commands loaded from a file.
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Commands {
    pub commands: Vec<CommandOption>,
}

/// Loads stored commands from a file.
pub fn load_config(path: &str) -> anyhow::Result<Commands> {
    let config_data = std::fs::read_to_string(path).context("unable to read config file")?; // Loading config data from file
    serde_json::from_str(&config_data).context("unable to parse config file") // Parsing config data
}

/// Saves stored commands to a file.
pub fn save_config(path: &str, config: &Commands) {
    let config_data = serde_json::to_string_pretty(config).expect("❌  Failed to serialize config"); // Serializing config data
    let mut file = std::fs::File::create(path).expect("❌  Unable to create config file"); // Creating or overwriting config file
    std::io::Write::write_all(&mut file, config_data.as_bytes()) // Writing config data to file
        .expect("❌  Unable to write to config file");
}

fn default_config() -> Commands {
    let default_commands = vec![CommandOption {
        display_name: "Command 1".to_string(),
        command: "echo '1'".to_string(),
    }];

    Commands {
        commands: default_commands,
    }
}

/// Creates a default list of stored commands.
pub fn create_default_config(path: &str) -> Commands {
    let default_config = default_config(); // Creating default commands

    save_config(path, &default_config); // Saving default commands to file
    println!("✅  Creating new default config."); // Printing success message
    default_config // Returning default commands
}

// Print the number of commands
pub fn print_num_commands(commands: &[CommandOption]) {
    println!("{} total commands:", commands.len()); // Printing the count of commands
}

// Print the command table
pub fn print_commands(commands: &[CommandOption]) {
    print_num_commands(commands); // Printing the count of commands
    print_command_table(commands); // Printing the command table
}

pub fn print_command_table(commands: &[CommandOption]) {
    let terminal_width = termion::terminal_size().unwrap().0 as usize; // Getting terminal width
    if !commands.is_empty() {
        // Checking if there are any commands
        let mut table = Table::new(); // Creating a new table
        table.add_row(row!["Number", "Display Name", "Command"]); // Adding table headers

        for (i, option) in commands.iter().enumerate() {
            // Iterating over commands
            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()), // Adding cell for command number
                Cell::new(&fill(&option.display_name, terminal_width / 3)), // Adding cell for display name with text wrapping
                Cell::new(&fill(&option.command, terminal_width / 3 * 2)), // Adding cell for command with text wrapping
            ]));
        }

        table.printstd(); // Printing the table to stdout
    }
}
