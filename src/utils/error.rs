/*
* This file is responsible for outputting human-readable
* errors on screen.
*/

use super::lines::Line;
use std::cmp::max;

/// Pretty print error
pub fn print_err(line: &Line, err: &str, solution: Option<&str>) {
    let l2: usize = (line.line_num).to_string().len();
    let l3: usize = (line.line_num.wrapping_add(1)).to_string().len();

    let offset: usize = max(l2, l3); // Compare n2 and n3 in case of n3 overflowing

    println!("\nERROR:");
    println!("{err}");
    println!("=> {}:{}\n", line.filename, line.line_num);

    println!("{:<offset$} | ... ", line.line_num - 1);
    println!("{:<offset$} | {}", line.line_num, line.code.trim());
    println!("{:<offset$} | ... ", line.line_num + 1);

    if let Some(fix) = solution {
        println!("\nPossible fix: {}", fix);
    }

    println!();
    println!("--------------------------------------------------------------------------------");
}
