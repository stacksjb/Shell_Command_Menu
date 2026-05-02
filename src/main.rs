use shell_command_menu::{config, menu_main, utils};

fn main() {
    // Print the version
    let version = utils::get_version();
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
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
            _ => {
                eprintln!("Unknown argument: {arg}");
                std::process::exit(2);
            }
        }
    }

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
