mod compiler;
mod utils;

use std::time::{Duration, Instant};

use crate::utils::command_line::print_help;
use compiler::compile::compile;
use std::env;
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

    // Retrieve the path to the root directory of the Rust project
    let cwd = env::current_exe().expect("Failed to get path of compiler binary!");
    let bin_folder = cwd
        .parent()
        .expect("Failed to get parent directory of compiler binary!");

    // Get path to assembler with path relative to compiler
    let assembler = bin_folder.join("assembler");

    // Check if the C++ library path exists (optional)
    if !assembler.exists() {
        panic!("Assembler not found at: {}", assembler.display());
    }
    println!("Found assembler: {}", assembler.display());

    let assembler_args: Vec<String> = if args.debug {
        vec![output_file, String::from("-d")]
    } else {
        vec![output_file]
    };

    start = time_now();
    let output = Command::new(format!("{}", assembler.display()))
        .args(assembler_args)
        .output()
        .unwrap();
    exec_time = calc_total_time(&start);
    println!("Total assembly time: {:?}", exec_time);
    println!(
        "Assembler result: {}",
        String::from_utf8(output.stdout).unwrap()
    );
}
