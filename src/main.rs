use std::{
    env,
    fs::{File, exists},
    path::PathBuf,
    str::FromStr,
};

use crate::todo::{add, clear, list, remove};
mod todo;

// checks if the todo file exists, if not create it
fn check_todo_file() -> std::io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => {
            let mut new_path = PathBuf::from(path);
            new_path.push("todo");
            match exists(&new_path).expect("Couldn't check for file") {
                true => Ok(new_path),
                // Create path/file if it does not exist
                false => {
                    let _ = File::create(&new_path)?;
                    Ok(new_path)
                }
            }
        }
        None => panic!("Impossible to get your home dir! Please set XDG directory"),
        // None =>
    }
}

fn check_todo_history_file() -> std::io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => {
            let mut new_path = PathBuf::from(path);
            new_path.push("todo_history");
            match exists(&new_path).expect("Couldn't check for file") {
                true => Ok(new_path),
                // Create path/file if it does not exist
                false => {
                    let _ = File::create(&new_path)?;
                    Ok(new_path)
                }
            }
        }
        None => panic!("Impossible to get your home dir! Please set XDG directory"),
        // None =>
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Accepts at least 1 argument
    if args.len() == 1 {
        panic!("No arguments provided.");
    }
    // check the number of arguments
    let todo_path = check_todo_file();
    let todo_history_path = check_todo_history_file();
    if args.len() == 2 {
        let operation = &args[1];
        match operation.as_str() {
            "list" => match todo_path {
                Ok(ref path_file) => {
                    let _ = list(path_file.to_path_buf());
                }
                Err(_) => todo!(),
            },
            "clear" => match todo_path {
                Ok(ref path_file) => {
                    let _ = clear(path_file.to_path_buf());
                }
                Err(ref err) => {
                    println!("Error clearing todo file: {}", err);
                }
            },
            _ => todo!(),
        }
    }
    if args.len() == 3 {
        let operation = &args[1];
        let arg = &args[2];
        match operation.as_str() {
            "add" => match todo_path {
                Ok(ref path_file) => {
                    let _ = add(path_file.to_path_buf(), arg.as_bytes());
                }
                Err(_) => todo!(),
            },
            "remove" => match todo_path {
                Ok(ref path_file) => {
                    let index = i64::from_str(arg);
                    match index {
                        Ok(index_val) => {
                            let _ = remove(path_file.to_path_buf(), index_val);
                        }
                        Err(error) => {
                            println!("Error: {}", error);
                        }
                    }
                }
                Err(_) => todo!(),
            },
            _ => todo!(),
        }
    }
}
