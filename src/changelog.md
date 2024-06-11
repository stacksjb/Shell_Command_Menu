# Changelog

6.11.24
Added auto comments and menu digit padding

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

5.28.24
<https://github.com/stacksjb/CLI_Shell_Command_Menu/commit/428599c33755c0028791529587540d6a8d97de09>
Massive rewrite of code to remove unused items, remove termion and stdio, use Inquire.

5.26.24
Cleaned up and refactored code into modules

5.23.24
Added Sound

5.22.24
Initial Commit, first functional version
