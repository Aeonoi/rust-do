use std::env;
use std::str::FromStr;

use crate::todo::TodoCreator;
mod todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Accepts at least 1 argument
    if args.len() == 1 {
        panic!("No arguments provided.");
    }
    let mut todo_creator = TodoCreator::new();
    match todo_creator.check_files() {
        Ok(_) => println!("All good!"),
        Err(err) => panic!("Error happened: {}", err),
    }
    // check the number of arguments
    if args.len() == 2 {
        let operation = &args[1];
        match operation.as_str() {
            "list" => match todo_creator.list(false) {
                Ok(_) => println!("All good!"),
                Err(err) => panic!("Error: {err}"),
            },
            "history" => match todo_creator.list(true) {
                Ok(_) => println!("All good!"),
                Err(err) => panic!("Error: {err}"),
            },
            "clear" => match todo_creator.clear(false) {
                Ok(_) => println!("All good!"),
                Err(err) => panic!("Error: {err}"),
            },
            "clear-history" => match todo_creator.clear(true) {
                Ok(_) => println!("All good!"),
                Err(err) => panic!("Error: {err}"),
            },
            "help" => todo_creator.help(),
            _ => todo!(),
        }
    }
    if args.len() == 3 {
        let operation = &args[1];
        let arg = &args[2];
        match operation.as_str() {
            "add" => match todo_creator.add(arg.as_bytes()) {
                Ok(_) => println!("All good!"),
                Err(err) => panic!("Error: {err}"),
            },
            "remove" => {
                let index = i64::from_str(arg);
                // not convertable to index
                if let Err(err) = index {
                    panic!("Error: {err}")
                }
                match todo_creator.remove(index.unwrap()) {
                    Ok(_) => println!("All good!"),
                    Err(err) => panic!("Error: {err}"),
                }
            }
            _ => todo!(),
        }
    }
}
