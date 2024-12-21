use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes to file as binary (as a series of bytes).
pub fn write_as_bin(filename: &Path, binary: &[u32]) {
    let mut file: File = File::create(filename).unwrap();

    for inst in binary {
        let bytes: [u8; 4] = inst.to_le_bytes(); // Converts 32-bit integer into 4 bytes
        file.write_all(&bytes).unwrap();
    }
}

/// Writes binary code as a string file. Easier for humans to read or even copy over
/// to other files if needed.
pub fn write_as_str(filename: &Path, binary: &[u32]) {
    let bin_str: String = binary
        .iter()
        .map(|inst| format!("{:032b}", inst))
        .collect::<String>();

    let mut file: File = File::create(filename).unwrap();
    file.write_all(bin_str.as_bytes()).unwrap();
}
