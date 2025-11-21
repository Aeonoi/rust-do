use std::{
    env,
    fs::{File, exists},
    path::PathBuf,
};
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
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // Accepts at least 1 argument
    // if args.len() == 1 {
    //     panic!("No arguments provided.");
    // }
    // check the number of arguments

    let _ = check_file();
}
