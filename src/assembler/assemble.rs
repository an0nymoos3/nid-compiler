use std::path::{Path, PathBuf};

use crate::assembler::exporter::{write_as_bin, write_as_str};
use crate::utils::command_line::Args;
use crate::{assembler::lexer::tokenize, utils::nid_fs::read_file};
use super::parser::parse_tokens;

pub fn assemble_program(args: &Args, program: &Path) -> PathBuf {
    // Generate the correct filename
    let output_name: PathBuf = PathBuf::from(program.to_str().unwrap().replace(".ass", ".out"));

    // Generate assembly tokens
    let code = read_file(program);
    let tokens = tokenize(code);

    let test_byte: Vec<u32> = vec![947];
    if args.verbose {
        println!("Writing to {} ...", output_name.display())
    }
    if args.string_output {
        write_as_str(&output_name, &test_byte)
    } else {
        write_as_bin(&output_name, &test_byte)
    }

    // Return binary program name
    output_name
}
