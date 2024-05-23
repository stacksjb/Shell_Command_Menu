use std::io::{self, Write};
use std::process::Command;

pub fn run_command(command: &str) {
    println!("Running command: {}", command);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");

    let _ = child.wait().expect("Command wasn't running");
}

pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
