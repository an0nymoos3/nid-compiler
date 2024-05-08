/*
* This is the main file that goes through the entire syntax tree and generates the correct ASS
* instructions in order.
*/

use std::process::exit;

use crate::compiler::ast;

use super::instruction_parser::parse_assignment;

/// Converts AST to ASS code, which is represented as a vector of strings (each string being an ASS
/// instruction)
pub fn generate_ass(program: &ast::Ast<dyn ast::Node>) -> Vec<String> {
    let mut ass_prog: Vec<String> = Vec::new();

    for node in program.body.iter() {
        match node.get_type() {
            ast::AstType::Assignment => {}
            _ => {
                println!("Invalid type while parsing ast!");
                exit(1);
            }
        }
    }

    ass_prog
}
