# Rust-do

Tested with only Linux systems. File path may be different on other OSes.

## Possible Arguments:

* [ ] '--help or -h': Displays help information about the program.
* [X] 'add <string>': Appends the specified string to the ``todo`` file.
* [ ] 'remove <index>': Removes the todo item at the specified index. 
* [ ] 'remove --search <string>": Removes the first todo item that matches the specified string (fuzzy finds and will displays the result for user to select again with index.
* [ ] 'list': Lists all the current todo items.
* [ ] 'clear': Clears all todo items from the ``todo`` file.
* [ ] 'revert': Reverts the last removal operation, restoring the most recently removed todo item. Will only work if a removal has occurred since the last revert.
* [ ] 'history': Loads the history indexed and tells what the operation and the string was
* [ ] 'save <filename>': Saves the current todo list to the specified filename.
* [ ] 'load <filename>': Loads todo items from the specified filename, replacing the current todo list.
* [ ] 'finish <index>': Marks the todo item at the specified index as completed.
* [ ] 'list': Lists all the current todo items in a index and checkmark format.

## Installation


