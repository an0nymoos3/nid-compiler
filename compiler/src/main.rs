mod compiler;
mod utils;

use std::time::{Duration, Instant};

use crate::utils::command_line::print_help;
use compiler::compile::compile;
use std::process::Command;
use utils::command_line::{build_args, Args};
use utils::compile_times::{calc_total_time, time_now};

/// main()
fn main() {
    let args: Args = build_args();

    if args.help {
        print_help();
        return;
    }

    println!("Compiling...");
    let mut start: Instant = time_now();

    if args.debug {
        println!("Running in debug (verbose) mode!");
    }

    let output_file: String = compile(&args);
    let mut exec_time: Duration = calc_total_time(&start);
    println!("Total compilation time: {:?}", exec_time);
    println!("Assembly written to: {}", output_file);

    start = time_now();
    let _ = Command::new("bin/assembler")
        .arg(&output_file)
        .output()
        .unwrap();
    exec_time = calc_total_time(&start);
    println!("Total assembly time: {:?}", exec_time);
}
