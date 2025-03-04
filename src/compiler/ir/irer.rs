/*
* IR:er, is a stupid name, meant to represent that this file handles
* conversion from AST to the NID compilers Intermediate Representation (IR).
*
* TODO:
* - Keep track of when new variables are created and give them a new %tmp{} register.
* - Implement assignments.
* - Implement comparisons / branching.
*/
use std::collections::VecDeque;

use super::stack_code::ast_to_stack_code;
use crate::compiler::parsing::ast::{self, AstType};

/// Generates stack code IR from an AST
pub fn generate_ir(tree: &ast::Ast<dyn ast::Node>) -> Vec<String> {
    let mut ir: Vec<String> = ast_to_stack_code(tree);

    let mut body = VecDeque::from(tree.body.clone());

    while !body.is_empty() {
        let node = body.pop_front().unwrap();

        unsafe {
            if (*node).get_type() == AstType::Function {
                ir.push(format!(
                    "define {}:",
                    (*(node as *mut ast::Function)).identifier
                ))
            }
        }
    }

    ir
}
