mod config;
mod edit;
mod menu;
mod utils;

fn main() {
    let config_path = "commands.json";
    if !std::path::Path::new(config_path).exists() {
        config::create_default_config(config_path);
    }

    menu::display_menu(config_path);
}
