# Shell_Command_Menu (aka CLI_Menu) :)

A simple Rust App to run stored shell commands from a config (Commands.json) file via a menu with options.

This is a simple app I wrote to help learn Rust and also to give me a framework for my daily CLI workflow. It tracks completion of each command and execution.

Commands are stored in a cli_menu_cmd.json JSON file located at your OS-Appropriate data folder using rust [Directories](<https://docs.rs/directories/latest/directories/struct.BaseDirs.html>) config_dir.

*Most of the initial credit goes to ChatGPT which wrote most of the code. Thanks to @Scott Pack for his talk at [BSides 2023](<https://www.youtube.com/watch?v=b_pkz4kDfq0>) which helped set the stage

Sound Effects credit Pixabay [whoosh](<https://pixabay.com/sound-effects/whoosh-6316/>) [message](<https://pixabay.com/sound-effects/message-incoming-132126/>)

## Example of my daily Usage with [Tod](https://github.com/tod-org/tod)

I have each of my daily [task contexts](https://gettingthingsdone.com/2010/09/david-allen-on-why-sorting-your-lists-by-contexts-even-matters/#:~:text=There%20is%20never%20a%20moment,you%20simply%20can't%20do.&text=Some%20suggested%20contexts%20to%20get,for%20Projects%20and%20Someday%20Maybe) grouped into a query command, that I then work through one at a time, starting from the top, as a daily routie.

![image](https://github.com/user-attachments/assets/7631747a-ad86-4100-82ef-584372a32006)

## Iterm2 Usage

I use this script to run my Todo list (CLI Menu) and Zoom the window, so I have it setup and mapped to a single `td` command which does the following:

1) Runs the Applescript `zoom_iterm.scpt` with `osascript zoom_iterm.scpt` - this script opens a new iTerm window with saved profile `CLI_Large` which is a saved profile that resizes the window and moves it to the bottom of my screen automatically
2) Executes the CLI_Menu program

You can configure custom window position/size in here if needed but it's not necessary as iTerm saves profile window locations for me.

However, in most cases I just have iTerm reopen saved windows, so even that isn't always necessary :)
