mod config; // Importing config module from separate file
mod edit; // Importing edit module from separate file
mod import; // Importing import module from separate file
mod menu; // Importing menu module from separate file
mod utils; // Importing utils module from separate file

fn main() {
    // Path to the commands.json file
    let commands_path = "commands.json"; // Setting path to commands.json file

    menu::display_menu(commands_path); // Displaying menu with provided commands file path
}
