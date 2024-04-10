mod compiler;
mod utils;

use std::time::{Duration, Instant};

use crate::utils::command_line::print_help;
use compiler::compile::compile;
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
    let start: Instant = time_now();

    if args.debug {
        println!("Running in debug (verbose) mode!");
    }

    let output_file: String = compile(&args);
    let exec_time: Duration = calc_total_time(&start);

    println!("Assembly file: {}", output_file);
    println!("Total compilation time: {:?}", exec_time);
}
