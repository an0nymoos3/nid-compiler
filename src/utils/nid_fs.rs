use std::path::Path;
use std::{fs, io::Write};

/// Reads entire file as a long contious string.
pub fn read_file(filename: &Path) -> String {
    fs::read_to_string(filename).expect("Failed to read file contents!")
}

/// Writes assembly program to file
pub fn write_to_file(program: &Vec<String>, filename: &Path) -> std::io::Result<()> {
    let mut file: fs::File = fs::File::create(filename)?;

    for inst in program {
        file.write_all(format!("{inst}\n").as_bytes())?;
    }

    Ok(())
}
