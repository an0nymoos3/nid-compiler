/*
* This is the main file that goes through the entire syntax tree and generates the correct ASS
* instructions in order.
*/

use super::instruction_parser::parse_assignment;
use crate::compiler::ast;
use std::process::exit;

/// Converts AST to ASS code, which is represented as a vector of strings (each string being an ASS
/// instruction)
pub fn generate_ass(program_body: &[Box<dyn ast::Node>], entry_point: usize) -> Vec<String> {
    let mut ass_prog: Vec<String> = Vec::new();

    for inst in program_body[entry_point + 1].get_body().iter() {
        match inst.get_type() {
            ast::AstType::Asm => {
                if let Some(asm_inst) = inst.as_any().downcast_ref::<ast::Asm>() {
                    // Check if asm
                    // block
                    for asm_line in &asm_inst.code {
                        ass_prog.push(asm_line.value.clone()); // Push the line of code to program.
                    }
                } else {
                    panic!("Downcasting from {:?} to Asm failed!", inst.get_type());
                }
            }
            _ => {
                panic!(
                    "Unhandled Node: {:?} of type: {:?}",
                    inst.get_name(),
                    inst.get_type()
                );
            }
        }
    }

    ass_prog
}
