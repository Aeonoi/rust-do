use std::{
    env,
    fs::{File, OpenOptions, exists},
    io::{Read, Write},
    path::PathBuf,
};

pub struct TodoCreator {
    todo_path: PathBuf,
    todo_history_path: PathBuf,
}

impl TodoCreator {
    pub fn new() -> TodoCreator {
        TodoCreator {
            todo_path: PathBuf::new(),
            todo_history_path: PathBuf::new(),
        }
    }

    pub fn check_files(&mut self) -> std::io::Result<()> {
        match env::home_dir() {
            Some(path) => {
                let mut todo_path = PathBuf::from(&path);
                let mut todo_history_path = PathBuf::from(&path);
                todo_path.push("todo");
                todo_history_path.push("todo_history");
                match exists(&todo_path).expect("Could not check for todo file") {
                    true => {
                        self.todo_path = todo_path;
                    }
                    // Create path/file if it does not exist
                    false => {
                        let _ = File::create(&todo_path)?;
                        self.todo_path = todo_path;
                    }
                }
                match exists(&todo_history_path).expect("Could not check for todo file") {
                    true => {
                        self.todo_history_path = todo_history_path;
                        Ok(())
                    }
                    // Create path/file if it does not exist
                    false => {
                        let _ = File::create(&todo_history_path)?;
                        self.todo_history_path = todo_history_path;
                        Ok(())
                    }
                }
            }
            None => panic!("Impossible to get your home dir! Please set HOME directory"),
        }
    }

    /// Appends the string or todo to the file path
    pub fn add(&mut self, todo_str: &[u8]) -> std::io::Result<()> {
        let mut buffer = File::options().append(true).open(&self.todo_path)?;
        buffer.write(b"* ")?;
        buffer.write(todo_str)?;
        buffer.write(b"\n")?;
        let u = str::from_utf8(todo_str);
        if let Ok(todo_item) = u {
            let _ = self.add_to_history("add", todo_item);
        } else {
            todo!()
        }
        Ok(())
    }

    // Read in the file contents of all other lines
    // Copy them into a different file and then truncate and then copy over all of the other lines
    pub fn remove(&mut self, index: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::options()
            .read(true)
            .write(true)
            .open(&self.todo_path)?;
        let mut buf = Vec::new();
        // TODO: Optimize by not reading to end but instead read char by char
        // and appending to this buf vec for every char that we want
        f.read_to_end(&mut buf)?;

        // handle incorrect index
        if index < 1 {
            return Err("Index must be greater than 0".into());
        }

        let mut lines = 0;

        // tracks the start and end of the line to be removed
        let mut start = 0;
        let mut end = 0;

        for i in 0..buf.len() {
            if buf[i] == b'\n' {
                if lines + 1 == index {
                    end = i + 1
                }
                lines += 1;
                if lines + 1 == index {
                    start = i + 1;
                }
            }
        }
        // Assume that we can never have end to be less than start otherwise there is an error
        // i.e. EOF
        if end < start {
            end = buf.len();
        }

        if index > lines as i64 {
            // Use template literals to add the size of the buffer
            return Err("Index must be smaller than size".into());
        }

        let removed: Vec<_> = buf.drain(start..end).collect();

        // Truncate file and then write into
        let mut new_buffer = File::create(&self.todo_path)?;
        new_buffer.write_all(&buf)?;

        let u = str::from_utf8(&removed);
        if let Ok(todo_item) = u {
            let _ = self.add_to_history("remove", todo_item);
        } else {
            todo!()
        }

        Ok(())
    }

    pub fn clean_list(&mut self) -> std::io::Result<()> {
        let mut f = File::open(&self.todo_path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        for (_, line) in buf.split(|&b| b == b'\n').enumerate() {
            if !line.is_empty() {
                println!("{}", String::from_utf8_lossy(line));
            }
        }
        Ok(())
    }

    pub fn list(&mut self, history: bool) -> std::io::Result<()> {
        let mut f = File::open(&self.todo_path)?;
        if history {
            f = File::open(&self.todo_history_path)?;
        }
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        for (i, line) in buf.split(|&b| b == b'\n').enumerate() {
            if !line.is_empty() {
                println!("[{}]: {}", i + 1, String::from_utf8_lossy(line));
            }
        }
        Ok(())
    }

    pub fn clear(&mut self, history: bool) -> std::io::Result<()> {
        if history {
            let _ = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.todo_history_path);
        } else {
            let _ = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.todo_path);
        }
        Ok(())
    }

    pub fn help(&mut self) {
        println!(
            "- 'help': Displays help information about the program.
- 'add <string>': Appends the specified string to the ``todo`` file.
- 'remove <index>': Removes the todo item at the specified index.
- 'list': Lists all the current todo items.
- 'history': Loads the history indexed and tells what the operation and the todo list was.
- 'clear': Clears all todo items from the ``todo`` file.
- 'clear-history': Clears the history log of operations."
        )
    }

    pub fn add_to_history(&mut self, operation: &str, todo_item: &str) -> std::io::Result<()> {
        // All todo items should contain the star(*)
        // TODO: Add timestamp w/o chrono
        // let now = SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();
        let mut todo = operation.to_string() + "," + todo_item;
        if operation == "add" {
            todo = operation.to_string() + ",* " + todo_item;
        }
        let mut buffer = File::options().append(true).open(&self.todo_history_path)?;
        buffer.write(todo.as_bytes())?;
        buffer.write(b"\n")?;
        Ok(())
    }
}

impl Default for TodoCreator {
    fn default() -> Self {
        Self::new()
    }
}
