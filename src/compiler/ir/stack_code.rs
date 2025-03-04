/*
* While ir.rs is responsible for the overall process of converting an AST into IR, this file
* handles the details of converting an AST.
*/

use crate::compiler::parsing::ast;

pub fn ast_to_stack_code(tree: &ast::Ast<dyn ast::Node>) -> Vec<String> {
    let ir: Vec<String> = Vec::new();

    ir
}
