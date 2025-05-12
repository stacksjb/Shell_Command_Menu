# Shell_Command_Menu (aka CLI_Menu)

A simple Rust App to run stored shell commands from a config (Commands.json) file via a menu with options.

This is a simple app I used ChatGPT to write to help learn Rust and also to give me a framework for my daily CLI workflow. It tracks completion of each command and execution.

Commands are stored in a cli_menu_cmd.json JSON file located at your OS-Appropriate data folder using rust Directories config_dir(<https://docs.rs/directories/latest/directories/struct.BaseDirs.html>)

*Most of the credit goes to ChatGPT which wrote most of the code. Thanks to @Scott Pack for his talk at BSides 2023 (<https://www.youtube.com/watch?v=b_pkz4kDfq0>) which helped set the stage

Sound Effects credit Pixabay
<https://pixabay.com/sound-effects/whoosh-6316/>
<https://pixabay.com/sound-effects/message-incoming-132126/>



## Iterm2 Usage

I use this script to run my Todo list (CLI Menu) and Zoom the window, so I have it setup and mapped to a single `td` command which does the following:
1) Runs the Applescript `zoom_iterm.scpt` with `osascript zoom_iterm.scpt` - this script opens a new iTerm window with saved profile `CLI_Large` which is a saved profile that resizes the window and moves it to the bottom of my screen automatically
2) Executes the CLI_Menu program

You can configure custom window position/size in here if needed but it's not necessary as iTerm saves profile window locations for me.

However, in most cases I just have iTerm reopen saved windows, so even that isn't always necessary :)
