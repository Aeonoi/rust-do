use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

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
pub fn list(path: PathBuf) -> std::io::Result<()> {
    let mut f = File::open(&path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    for (i, line) in buf.split(|&b| b == b'\n').enumerate() {
        if !line.is_empty() {
            println!("[{}]: {}", i + 1, String::from_utf8_lossy(line));
        }
    }
    Ok(())
}
// pub fn clear(path: PathBuf) {
//     todo!()
// }
// pub fn revert(path: PathBuf) {
//     todo!()
// }
// pub fn history(path: PathBuf) {
//     todo!()
// }
