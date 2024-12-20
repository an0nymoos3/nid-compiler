/*
* This file handles command line arguments sent to the compiler.
*
* TODO: Add a --output | -o flag to specify an ouput file.
*/

use std::env;
use std::path::PathBuf;

#[derive(Debug)]
/// Possible args that can be used when running compiler.
pub struct Args {
    pub filename: String,
    pub verbose: bool,
    pub help: bool,
    pub hardware_conf: PathBuf,
    pub compile_only: bool,
    pub assemble_only: bool,
    pub string_output: bool,
}

/// Reads and returns the correct command line args passed by user.
pub fn build_args() -> Args {
    let mut args: Args = Args {
        filename: String::new(),
        verbose: false,
        help: false,
        hardware_conf: PathBuf::new(),
        string_output: false,
        compile_only: false,
        assemble_only: false,
    };

    let cmd_line: Vec<String> = env::args().collect();

    for (i, arg) in cmd_line.iter().enumerate() {
        if arg.contains(".nid") || arg.contains(".ass") {
            args.filename = arg.to_owned();
        }
        if arg == "--verbose" || arg == "-v" {
            args.verbose = true;
        }
        if arg == "--help" || arg == "-h" {
            args.help = true;
        }
        if arg == "--hardware-conf" || arg == "-hc" {
            args.hardware_conf = PathBuf::from(
                cmd_line
                    .get(i + 1)
                    .expect("Error getting path from --hardware-conf!"),
            );
        }
        if arg == "--string-output" || arg == "-s" {
            args.string_output = true;
        }
        if arg == "--compile-only" || arg == "-c" {
            args.compile_only = true;
        }
        if arg == "--assemble-only" || arg == "-a" {
            args.assemble_only = true;
        }
    }

    args
}

/// Prints this not so helpful help message.
pub fn print_help() {
    let mut message: String = String::new();
    message.push_str("nidc [options] [target].nid\n");
    message.push_str("Options:\n");
    message.push_str("-h  | --help                  Prints this message.\n");
    message.push_str("-v  | --verbose               Run compiler in verbose mode.\n");
    message.push_str("-hc | --hardware-conf         Specify custom hardware configuration.\n");
    message.push_str("-s  | --string-output         Output binary as a text file, rather than actual binary file.\n");
    message
        .push_str("-c  | --compile-only          Compile to ASS, without assembling to binary.\n");
    message.push_str("-a  | --assemble-only         Only assemble a .ass file.\n");

    println!("{}", message);
}
