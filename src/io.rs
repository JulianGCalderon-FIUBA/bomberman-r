use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
