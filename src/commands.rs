use prettytable::{row, Cell, Row, Table}; // Importing types for creating tables
use serde::{Deserialize, Serialize}; // Importing serialization and deserialization traits
use textwrap::fill; // Importing fill function from textwrap crate
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
