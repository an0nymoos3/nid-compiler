/*
* This file exports the generated ASS code to a file of a given name.
*/

use std::{fs, io::Write};

/// Writes assembly program to file
pub fn write_to_file(program: &Vec<String>, filename: &str) -> std::io::Result<()> {
    let mut file: fs::File = fs::File::create(filename)?;

    for inst in program {
        file.write_all(format!("{inst}\n").as_bytes())?;
    }

    Ok(())
}
