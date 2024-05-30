use edit::edit_menu;

mod config;
mod edit;
mod utils;
mod menu;

fn main() {
    let config_path = "commands.json";
    if !std::path::Path::new(config_path).exists() {
        // If the config file does not exist, create a new one and open up edit_menu
    println!("⚠️  Config does not exist or is invalid; editing new config!");
       edit_menu(config_path);

    }

    // Display the menu
    menu::display_menu(config_path);
}
