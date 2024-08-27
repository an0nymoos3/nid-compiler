/*
* This file will will be responsible for outputing human readable
* errors on screen.
*
* TODO: Implement some sort of Line struct to store data about what
* line of NID code that caused error.
* After that this file will be functional.
*/

/*
/// Pretty print error
fn print_err(line: &Line, err: &str, solution: Option<&str>) {
    println!(); // Newline

    println!("{err}");
    println!("{} | ---------- ", line.line_nr - 1);
    println!(
        "{} | {}",
        line.line_nr,
        line.line_content.iter().collect::<String>()
    );
    println!("{} | ---------- ", line.line_nr + 1);

    if let Some(fix) = solution {
        println!("Possible fix: {}", fix);
    }

    println!();
    println!("----------------------------------------");
}
*/
