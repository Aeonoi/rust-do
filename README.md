# Rust-do

Tested with only Linux systems. File path may be different on other OSes.

## Possible Arguments:

* [ ] '--help or -h': Displays help information about the program.
* [X] 'add <string>': Appends the specified string to the ``todo`` file.
* [ ] 'remove <index>': Removes the todo item at the specified index. 
* [X] 'list': Lists all the current todo items.
* [ ] 'clear': Clears all todo items from the ``todo`` file.
* [ ] 'revert': Reverts the last removal operation, restoring the most recently removed todo item. Will only work if a removal has occurred since the last revert.
* [ ] 'revert <index>': Reverts the operation done with the index in the history log.
* [ ] 'history': Loads the history indexed and tells what the operation and the string was

Improved basic functionality.
* [ ] 'finish <index>': Marks the todo item at the specified index as completed.
* [ ] 'remove <array of indexes>': Removes multiple todo items at the specified indexes.
* [ ] 'list': Lists all the current todo items in a index and checkmark format.
* [ ] 'remove --search <string>": Removes the first todo item that matches the specified string (fuzzy finds and will displays the result for user to select again with index.
* [ ] 'save <filename>': Saves the current todo list to the specified filename.
* [ ] 'load <filename>': Loads todo items from the specified filename, replacing the current todo list.
* [ ] 'undo clear': Reverts the last clear operation, restoring all todo items that were removed.

## Installation


## Considerations

* Use write locks and unlocks to ensure that when the application runs, we do not overwrite the file if another instance is using the file. We prioritize the first instance that opened the file.
