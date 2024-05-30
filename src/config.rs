use anyhow::Context;
use prettytable::{row, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use textwrap::fill;

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
    let default_config = default_config();

    save_config(path, &default_config);
    println!("✅  Creating new default config.");
    default_config
}
//Print the command count
pub fn print_num_commands(commands: &[CommandOption]) {
    println!("{} total commands:", commands.len());
}
//Print the command table
pub fn print_commands(commands: &[CommandOption]) {



    // Display the number of commands
    print_num_commands(commands);
   // print the command table
    print_command_table(commands);
}

pub fn print_command_table(commands: &[CommandOption]) {
    // Get the terminal width to wrap text accordingly
    let terminal_width = termion::terminal_size().unwrap().0 as usize;
    if !commands.is_empty() {
        let mut table = Table::new();
        // Create a table to display the commands if there are any
        table.add_row(row!["Number", "Display Name", "Command"]);

        for (i, option) in commands.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(&fill(&option.display_name, terminal_width / 3)),
                Cell::new(&fill(&option.command, terminal_width / 3 * 2)),
            ]));
        }

        table.printstd();
    }
}