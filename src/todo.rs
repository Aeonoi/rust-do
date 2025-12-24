use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

/// Appends the string or todo to the file path
pub fn add(path: PathBuf, todo_str: &[u8]) -> std::io::Result<()> {
    let mut buffer = File::options().append(true).open(&path)?;
    buffer.write(b"* ")?;
    buffer.write(todo_str)?;
    buffer.write(b"\n")?;
    Ok(())
}

// Read in the file contents of all other lines
// Copy them into a different file and then truncate and then copy over all of the other lines
pub fn remove(path: PathBuf, index: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::options().read(true).write(true).open(&path)?;
    let mut buf = Vec::new();
    // TODO: Optimize by not reading to end but instead read char by char
    // and appending to this buf vec for every char that we want
    f.read_to_end(&mut buf)?;

    // handle incorrect index
    if index < 1 {
        return Err("Index must be greater than 0".into());
    }
    if index > buf.len() as i64 {
        // Use template literals to add the size of the buffer
        return Err("Index must be smaller than size".into());
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
    println!("Start: {start}, End: {end}");

    buf.drain(start..end);

    // Truncate file and then write into
    let mut new_buffer = File::create(&path)?;
    new_buffer.write_all(&buf)?;

    Ok(())
}

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
pub fn clear(path: PathBuf) -> std::io::Result<()> {
    println!("Calling clear");
    let _ = OpenOptions::new().write(true).truncate(true).open(path);
    Ok(())
}

// pub fn help() {
// }

// pub fn history(path: PathBuf) {
//     todo!()
// }
