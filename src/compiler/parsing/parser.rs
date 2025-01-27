/*
* This file handles the logic of parsing the lexed Tokens into some sort of
* Abstract Syntax Tree.
*
* If the main parser function "generate_ast()" fails at some point it will attempt
* to continue compiling the rest of the program to find more potential errors
* before returning None to compile(), which then knows to terminate the compiler.
*/

use super::ast::{self};
use super::lexer::Token;
use std::collections::VecDeque;

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    None
}
