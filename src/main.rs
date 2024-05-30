mod config;
mod edit;
mod import;
mod menu;
mod utils;

fn main() {
    // Path to the commands.json file
    let commands_path = "commands.json";
    menu::display_menu(commands_path);
}
