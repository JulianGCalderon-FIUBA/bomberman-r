use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::MyResult;

pub fn write_to_file(path: &Path, content: &str) -> MyResult<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn read_from_file(path: &Path) -> MyResult<String> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
