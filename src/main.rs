mod commands; // Importing commands module from separate file
mod config; // Importing config module from separate file
mod edit; // Importing edit module from separate file
mod import; // Importing import module from separate file
mod menu; // Importing menu module from separate file
mod utils; // Importing utils module from separate file // Importing menu module from separate file

fn main() {
    let config_path = match config::get_config_file_path() {
        Ok(path) => path, // If the function returns Ok(path), use the path
        Err(e) => {
            let _ = &eprintln!("{}", e); // Print the error message
            std::process::exit(1); // Exit if unable to get the config path
        }
    };
    menu::display_menu(&config_path);
}
