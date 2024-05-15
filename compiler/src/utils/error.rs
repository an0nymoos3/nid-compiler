use crate::compiler::compile::Line;

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

