/*
* This is the main file that goes through the entire syntax tree and generates ASS code.
*/

use super::{
    instruction_parser::{
        parse_assignment, parse_branch_statement, parse_builtin_functions, parse_loop_statement,
    },
    memory_manager::{remove_mem_from_compiler, set_max_addr, set_max_regs},
};
use crate::{compiler::ast, utils::hardware_conf::Hardware};

/// Converts AST to ASS code, which is represented as a vector of strings (each string being an ASS
/// instruction)
pub fn generate_ass(
    program_body: &[Box<dyn ast::Node>],
    entry_point: usize,
    hardware_conf: &Hardware,
) -> Vec<String> {
    let mut preallocstart: Option<u16> = None;
    let mut preallocend: Option<u16> = None;

    set_max_addr(hardware_conf.mem_addresses - 20); // Hard coded 20 last positions in memory as a
                                                    // call stack
    set_max_regs(hardware_conf.registers);

    // First look for certain global things in the code. Currently only looks for macros
    for inst in program_body {
        // Look for macros
        if let Some(nid_macro) = inst.as_any().downcast_ref::<ast::Macro>() {
            if nid_macro.macro_type == ast::MacroType::PreAllocStart {
                preallocstart = Some(nid_macro.macro_value);
            } else if nid_macro.macro_type == ast::MacroType::PreAllocEnd {
                preallocend = Some(nid_macro.macro_value);
            }
        }
    }

    // Tell compiler to not touch certain memory addresses
    remove_mem_from_compiler(preallocstart, preallocend);
    generate_body_ass(program_body[entry_point].get_body())
}

/// Parses a body of NID AST nodes. Helper function as it can be used for recursive parsing.
pub fn generate_body_ass(program_body: &[Box<dyn ast::Node>]) -> Vec<String> {
    let mut ass_prog: Vec<String> = Vec::new();

    // Iterate over the list of nodes
    for inst in program_body.iter() {
        // Match the correct node type
        match inst.get_type() {
            ast::AstType::Asm => {
                if let Some(asm_inst) = inst.as_any().downcast_ref::<ast::Asm>() {
                    for asm_line in &asm_inst.code {
                        ass_prog.push(asm_line.value.clone()); // Push the line of code to program.
                    }
                } else {
                    panic!("Downcasting from {:?} to Asm failed!", inst.get_type());
                }
            }
            ast::AstType::Assignment => {
                if let Some(assign_inst) = inst.as_any().downcast_ref::<ast::Assignment>() {
                    for ass_line in parse_assignment(assign_inst) {
                        ass_prog.push(ass_line); // Push the line of code to program.
                    }
                } else {
                    panic!(
                        "Downcasting from {:?} to Assignment failed!",
                        inst.get_type()
                    );
                }
            }
            ast::AstType::Branch => {
                if let Some(branch_inst) = inst.as_any().downcast_ref::<ast::Branch>() {
                    for ass_line in parse_branch_statement(branch_inst) {
                        ass_prog.push(ass_line);
                    }
                } else {
                    panic!("Downcasting from {:?} to Branch failed!", inst.get_type());
                }
            }
            ast::AstType::Builtin => {
                if let Some(builtin) = inst.as_any().downcast_ref::<ast::Builtin>() {
                    for ass_line in parse_builtin_functions(builtin) {
                        ass_prog.push(ass_line);
                    }
                } else {
                    panic!("Downcasting from {:?} to Builtin failed!", inst.get_type())
                }
            }
            ast::AstType::Loop => {
                if let Some(loop_inst) = inst.as_any().downcast_ref::<ast::Loop>() {
                    for ass_line in parse_loop_statement(loop_inst) {
                        ass_prog.push(ass_line);
                    }
                } else {
                    panic!("Downcasting from {:?} to Loop failed!", inst.get_type());
                }
            }
            ast::AstType::Return => {
                // TODO: Handle return statements once functions are implemented.
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
