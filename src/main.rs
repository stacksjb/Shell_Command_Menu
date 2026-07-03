use shell_command_menu::{config, menu_main, utils};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Print the version
    let version = utils::get_version();
    let mut args = std::env::args().skip(1);
    let mut config_override: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("{version}");
                return;
            }
            "--run-once" => {
                let Some(command) = args.next() else {
                    eprintln!("Missing command for --run-once");
                    std::process::exit(2);
                };
                match utils::run_command(&command) {
                    Ok(status) => std::process::exit(status.code().unwrap_or(1)),
                    Err(e) => {
                        eprintln!("Failed to run command: {e}");
                        std::process::exit(1);
                    }
                }
            }
            "--config" | "-c" => {
                let Some(path) = args.next() else {
                    eprintln!("Missing path for {arg}");
                    std::process::exit(2);
                };
                config_override = Some(PathBuf::from(path));
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                std::process::exit(2);
            }
        }
    }

    println!("Welcome to CLI_Menu v{version}!");
    // Execute the config::get_config_file_path function to get the config file path and load it; else create it
    let config_path_result = match config_override {
        Some(path) => config::ensure_config_file_path(path),
        None => config::get_config_file_path(),
    };
    let config_path = match config_path_result {
        Ok(path) => {
            path // Return the path
        }
        Err(e) => {
            eprintln!("{e}"); // Print the error message
            std::process::exit(1); // Exit if unable to get the config path
        }
    };
    //Execute the display_menu function from the menu module with the config file from previous function
    menu_main::display_menu(&config_path).await;
}
