use std::{
    env,
    fs::{File, exists},
    path::PathBuf,
};

use crate::todo::{add, list};
mod todo;

// checks if the file exists, if not create it
fn check_file() -> std::io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => {
            let mut new_path = PathBuf::from(path);
            new_path.push("todo");
            match exists(&new_path).expect("Couldn't check for file") {
                true => {
                    println!("File path exists");
                    Ok(new_path)
                }
                // Create path/file if it does not exist
                false => {
                    let _ = File::create(&new_path)?;
                    println!("File path created");
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
    if args.len() == 2 {
        let operation = &args[1];
        match operation.as_str() {
            "list" => {
                let path = check_file();
                match path {
                    Ok(path_file) => {
                        let _ = list(path_file);
                    }
                    Err(_) => todo!(),
                }
            }
            _ => todo!(),
        }
    }
    if args.len() == 3 {
        let operation = &args[1];
        let arg = &args[2];
        match operation.as_str() {
            "add" => {
                let path = check_file();
                match path {
                    Ok(path_file) => {
                        let _ = add(path_file, arg.as_bytes());
                    }
                    Err(_) => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
