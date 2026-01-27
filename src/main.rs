use shell_command_menu::{config, menu_main, utils};

fn main() {
    // Print the version
    let version = utils::get_version();
    println!("Welcome to CLI_Menu v{version}!");
    // Execute the config::get_config_file_path function to get the config file path and load it; else create it
    let config_path = match config::get_config_file_path() {
        Ok(path) => {
            path // Return the path
        }
        Err(e) => {
            let _ = &eprintln!("{e}"); // Print the error message
            std::process::exit(1); // Exit if unable to get the config path
        }
    };
    //Execute the display_menu function from the menu module with the config file from previous function
    menu_main::display_menu(&config_path);
}
