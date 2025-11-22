use std::{fs::File, io::Write, path::PathBuf};

/// Appends the string or todo to the file path
pub fn add(path: PathBuf, todo_str: &[u8]) -> std::io::Result<()> {
    let mut buffer = File::options().append(true).open(&path)?;
    buffer.write(todo_str)?;
    buffer.write(b"\n")?;
    Ok(())
}

// pub fn remove(path: PathBuf, index: i32) {
//     todo!()
// }
// pub fn list(path: PathBuf) {
//     todo!()
// }
// pub fn clear(path: PathBuf) {
//     todo!()
// }
// pub fn revert(path: PathBuf) {
//     todo!()
// }
// pub fn history(path: PathBuf) {
//     todo!()
// }
