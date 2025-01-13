use std::path::{Path, PathBuf};

use super::parser::parse_tokens;
use crate::assembler::exporter::{write_as_bin, write_as_str};
use crate::utils::command_line::Args;
use crate::{assembler::lexer::tokenize, utils::nid_fs::read_file};

pub fn assemble_program(args: &Args, program: &Path) -> PathBuf {
    // Generate the correct filename
    let output_name: PathBuf = PathBuf::from(program.to_str().unwrap().replace(".ass", ".out"));

    // Read ASS
    let code = read_file(program);

    // Tokenize
    let mut tokens = tokenize(code);

    // Parse
    let binary_rep: Vec<u32> = parse_tokens(&mut tokens);

    if args.verbose {
        println!("Writing to {} ...", output_name.display())
    }

    // Write to file
    if args.string_output {
        write_as_str(&output_name, &binary_rep)
    } else {
        write_as_bin(&output_name, &binary_rep)
    }

    // Return binary program name
    output_name
}
