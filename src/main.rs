use std::env;
mod todo;

// checks if the file exists, if not create it
fn check_file() {
    let path = "todo.txt";
    if !std::path::Path::new(path).exists() {
        std::fs::File::create
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // Accepts at least 1 argument
    if args.len() == 1 {
        panic!("No arguments provided.");
    }
    // check the number of arguments
}
