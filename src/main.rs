mod assembler;
mod compiler;
mod utils;

use crate::utils::command_line::print_help;
use crate::utils::hardware_conf::Hardware;
use compiler::compile::compile;
use std::env;
use std::process::Command;
use std::time::{Duration, Instant};
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

    if args.verbose {
        println!("Running in debug (verbose) mode!");
    }

    // Generate a hardware conf struct, that will be sent to compiler as
    // a read-only refrence.
    let hardware_conf: Hardware = if args.hardware_conf.exists() {
        Hardware::from(&args.hardware_conf)
    } else {
        println!("No valid hardware config file passed! Using default config.");
        Hardware::default()
    };

    // Run compiler
    let output_file: String = compile(&args, &hardware_conf);
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
    if !assembler.exists() {
        panic!("Assembler not found at: {}", assembler.display());
    }
    println!("Found assembler: {}", assembler.display());

    // Check if assembler should run with debug/verbose flag
    let assembler_args: Vec<String> = if args.verbose {
        vec![output_file, String::from("-d")]
    } else {
        vec![output_file]
    };

    // Run assembler
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
