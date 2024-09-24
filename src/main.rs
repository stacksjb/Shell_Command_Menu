mod config;
mod csv;
mod menu;
mod utils;

fn main() {
    // Attempt to read the Config at the default path
    let config_path = match config::get_config_file_path() {
        Ok(path) => {
            // Print loaded successfully
            println!("✅  Config file loaded successfully from path: {:?}", path);
            path // Return the path
        }
        Err(e) => {
            let _ = &eprintln!("{}", e); // Print the error message
            std::process::exit(1); // Exit if unable to get the config path
        }
    };
    //Execute the display_menu function from the menu module with the config file
    menu::display_menu(&config_path);
}
