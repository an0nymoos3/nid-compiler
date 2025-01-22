/// Abstraction of a line that the user would see in the code editor.
/// Used for better error output.
#[derive(Debug, Clone)]
pub struct Line {
    pub line_num: u32,
    pub code: String,
    pub filename: String,
}

impl Line {
    pub fn new(line_num: u32, code: String, filename: String) -> Self {
        Self {
            line_num,
            code,
            filename,
        }
    }
}

/// Takes in source code in form of a single long string slice.
/// Returns a vector or Lines, useful for generating errors for
/// developer.
pub fn generate_lines(code: &str, filename: &str) -> Vec<Box<Line>> {
    let mut lines: Vec<Box<Line>> = Vec::new();

    let mut line_content: String = String::new();
    let mut cur_line: u32 = 1;

    for char in code.chars() {
        if char == '\n' || char == '\r' {
            lines.push(Box::new(Line::new(
                cur_line,
                line_content.to_string(),
                filename.to_string(),
            )));
            line_content.clear();
            cur_line += 1;
        }

        line_content.push(char);
    }

    lines.push(Box::new(Line::new(
        cur_line,
        line_content.to_string(),
        filename.to_string(),
    )));

    lines
}
