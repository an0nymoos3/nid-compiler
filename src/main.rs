mod assembler;
mod compiler;
mod utils;

use crate::utils::command_line::print_help;
use compiler::compiler_main::compile;
use utils::command_line::{build_args, Args};

fn main() {
    let args: Args = build_args();

    if args.help {
        print_help();
        return;
    }

    println!("Compiling...");
    if args.debug {
        println!("Running in debug (verbose) mode!");
    }

    let output_file: String = compile(&args);
    println!("Assembly file: {}", output_file);
}
