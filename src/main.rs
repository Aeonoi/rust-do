use std::env;
use std::str::FromStr;

use crate::todo::TodoCreator;
mod todo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("No arguments provided".into());
    }

    let mut todo_creator = TodoCreator::new();
    todo_creator.check_files()?;

    if args.len() == 2 {
        match args[1].as_str() {
            "clean_list" => todo_creator.clean_list()?,
            "list" => todo_creator.list(false)?,
            "history" => todo_creator.list(true)?,
            "clear" => todo_creator.clear(false)?,
            "clear-history" => todo_creator.clear(true)?,
            "help" => todo_creator.help(),
            _ => todo!(),
        }
    }

    if args.len() == 3 {
        match args[1].as_str() {
            "add" => todo_creator.add(args[2].as_bytes())?,
            "remove" => {
                let index = i64::from_str(&args[2])?;
                todo_creator.remove(index)?;
            }
            _ => todo!(),
        }
    }

    Ok(())
}
