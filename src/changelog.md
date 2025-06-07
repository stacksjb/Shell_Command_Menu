# Changelog

6/6/25 - v0.2.4
Addded "reorder" option for menu config.
Split out edit to another module and rewrote for error handling of escape/cntrl-C.

05/26/25
Adding tests and codecov for future cleanup/implementation - not yet active.

05/11/25
Updated Cargo.tml and dependencies for Rust/Cargo 2024 Edition
Added testcases

10.15.24
Removed duplicate printline

9.23.24
Complete rewrite with 200% more fluff and garbage with updated config struct and additional prompts to allow for future expansion

9.11.24
Added Directories crate and updated to store commands.json in a standard location across all OS for consistent experience with running on OS per user instead of per location
Updated dependencies
Renamed config.rs to commands.rs
Updated vnum to 0.2.0 for prework to publish for full build on target OS
Updated edits to use PathBuf instead of String for config path

6.11.24
Added auto comments and menu digit padding
Fixed menu parsing to handle terminal size correctly

5.30.24
Added CSV Import Function (append/overwrite) - see exampleimport.csv for format
Added pause after import and edit so you can easily read what is imported
Reformatted edit and import to remove duplicate printing of table
Split out printing of table and count functions
Clarified wording

5.29.24
Fixed to handle flat JSON without need for Command Number variable (credit to @bluebear94 Thx!)
Added purge command to delete all entries
Added logic to automatically edit if file does not exist after creating the default
Added command count(s) to edit list; only print table if command count >0
Added termion back for specific functions

5.28.24
<https://github.com/stacksjb/CLI_Shell_Command_Menu/commit/428599c33755c0028791529587540d6a8d97de09>
Massive rewrite of code to remove unused items, remove termion and stdio, use Inquire.

5.26.24
Cleaned up and refactored code into modules

5.23.24
Added Sound

5.22.24
Initial Commit, first functional version
