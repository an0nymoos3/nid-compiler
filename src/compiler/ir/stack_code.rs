/*
* While ir.rs is responsible for the overall process of converting an AST into IR, this file
* handles the details of converting an AST.
*
* TODO:
* - Keep track of when new variables are created and give them a new %tmp{} register.
* - Implement assignments.
* - Implement comparisons / branching.
*/

use crate::compiler::parsing::ast::{self, AstType};
use std::collections::VecDeque;

pub fn ast_to_stack_code(tree: &ast::Ast) -> Vec<String> {
    let mut ir: Vec<String> = Vec::new();

    let mut body = VecDeque::from(tree.body.body.clone());

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
