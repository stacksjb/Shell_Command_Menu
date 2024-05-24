mod config;
mod edit;
mod utils;
mod menu;

fn main() {
    let config_path = "commands.json";
    if !std::path::Path::new(config_path).exists() {
        config::create_default_config(config_path);
    }

    // Display the menu
    menu::display_menu(config_path);
}
