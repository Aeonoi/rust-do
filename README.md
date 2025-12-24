# Rust-do

Tested with only Linux systems. File path may be different on other OSes.

## Arguments:

* [X] 'help': Displays help information about the program.
* [X] 'add <string>': Appends the specified string to the ``todo`` file.
* [X] 'remove <index>': Removes the todo item at the specified index. 
* [X] 'list': Lists all the current todo items.
* [X] 'clear': Clears all todo items from the ``todo`` file.
* [X] 'clear-history': Clears the history log of operations.
* [X] 'history': Loads the history indexed and tells what the operation and the string was

Improved basic functionality.
* Use shortened names for operations/commands such as 'rm' for remove, 'ls' for list, etc and add support for flags such as -h for help, -a for add, -r for remove, etc.
* [ ] 'revert': Reverts the last removal operation, restoring the most recently removed todo item. Will only work if a removal has occurred since the last revert.
* [ ] 'revert <index>': Reverts the operation done with the index in the history log.
* [ ] 'finish <index>': Marks the todo item at the specified index as completed.
* [ ] 'remove <array of indexes>': Removes multiple todo items at the specified indexes.
* [ ] 'list': Lists all the current todo items in a index and checkmark format.
* [ ] 'remove --search <string>": Removes the first todo item that matches the specified string (fuzzy finds and will displays the result for user to select again with index.
* [ ] 'save <filename>': Saves the current todo list to the specified filename.
* [ ] 'load <filename>': Loads todo items from the specified filename, replacing the current todo list.
* [ ] 'undo clear': Reverts the last clear operation, restoring all todo items that were removed.

## Installation


## Testing

Use Docker.
