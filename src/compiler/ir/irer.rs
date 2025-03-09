/*
* IR:er, is a stupid name, meant to represent that this file handles
* conversion from AST to the NID compilers Intermediate Representation (IR).
*/

use super::super::parsing::ast;
use super::stack_code::ast_to_stack_code;

/// Generates stack code IR from an AST
pub fn generate_ir(tree: &ast::Ast<dyn ast::Node>) -> Vec<String> {
    let ir: Vec<String> = ast_to_stack_code(tree);

    ir
}
