mod assembler;
mod compiler;
mod utils;

use crate::utils::command_line::print_help;
use crate::utils::hardware_conf::Hardware;
use assembler::assemble::assemble_program;
use compiler::compile::compile;
use std::time::{Duration, Instant};
use utils::command_line::{build_args, Args};
use utils::compile_times::{calc_total_time, time_now};

use std::path::PathBuf;

/// main()
fn main() {
    let args: Args = build_args();
    if args.help {
        print_help();
        return;
    }

    if args.compile_only && args.assemble_only {
        panic!("Can't run compiler in compile only and assembly only modes at the same time!")
    }

    if args.verbose {
        println!("Running in debug (verbose) mode!");
    }

    let start: Instant = time_now();

    // Compile NID program
    let ass_file: PathBuf = if !args.assemble_only {
        println!("Compiling...");

        // Generate a hardware conf struct, that will be sent to compiler as
        // a read-only refrence.
        let hardware_conf: Hardware = if args.hardware_conf.exists() {
            Hardware::from(&args.hardware_conf)
        } else {
            println!("No valid hardware config file passed! Using default config.");
            Hardware::default()
        };

    // Run compiler
    let ass_out_file: PathBuf = compile(&args, &hardware_conf);

    // Run assembler
    let bin_out_file: PathBuf = assemble_program(&args, &ass_out_file);

    // Print time
    let exec_time: Duration = calc_total_time(&start);
    println!("Total compilation time: {:?}", exec_time);
}
