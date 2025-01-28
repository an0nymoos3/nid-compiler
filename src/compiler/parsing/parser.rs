/*
* This file handles the logic of parsing the lexed Tokens into some sort of
* Abstract Syntax Tree.
*
* If the main parser function "generate_ast()" fails at some point it will attempt
* to continue compiling the rest of the program to find more potential errors
* before returning None to compile(), which then knows to terminate the compiler.
*
*   Integer, // A value such as "45"
    Floating,
    String,
    Char,
    Bool,
    Identifier,       // Human readable identifier, such as variable name
    Assignment,       // Assigning operator
    OpenParen,        // (
    CloseParen,       // )
    OpenScope,        // {
    CloseScope,       // }
    ArrayAccessOpen,  // [
    ArrayAccessClose, // ]
    BinaryOperator,   // +, -, *, /
    Comparison,       // ==, <=, >=
    LogicOperator,    // !, &&, ||
    TypeIndicator,    // Used to declare variable type and function return
    Loop,
    Branch,      // If conditions etc...
    Seperator,   // for identifying seperations for things like parameters (,)
    Punctuation, // . (used for accessing fields or structs)
    Pointer,     // Same as ptrs in C and C++, points to a memory address
    Reference,   // -- || --
    Return,      // Return statement
    Asm,         // Allows for inline assembly code
    Eol,         // End of line, basically ; representing end of line.
    Eof, // Represents the end of the code (EOF all caps appears to be a reserved word of some kind)
    Macro, // Basic macro functionality, such as allocating memory that the compiler is not allowed
         // to touch
*/

use crate::utils::error::print_err;

use super::ast::{self};
use super::lexer::{Token, TokenType};
use std::collections::VecDeque;

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    let nodes = match generate_nodes(tokens) {
        Some(n) => n,
        None => return None,
    };

    for node in nodes.iter() {
        println!("{:?}", node.display());
    }

    // Empty AST while rebuilding compiler
    let ast = ast::Ast {
        entry_point: 0,
        body: nodes,
    };

    Some(ast)
}

/// First step of building AST, create nodes from Tokens
fn generate_nodes(tokens: &mut VecDeque<Token>) -> Option<Vec<Box<dyn ast::Node>>> {
    let mut nodes: Vec<Box<dyn ast::Node>> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        let new_node: Box<dyn ast::Node> = match token.token_type {
            TokenType::Integer => Box::new(ast::Value {
                value: Some(ast::ValueEnum::Int(token.value.parse::<i16>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Floating => Box::new(ast::Value {
                value: Some(ast::ValueEnum::Float(token.value.parse::<f32>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::String => Box::new(ast::Value {
                value: Some(ast::ValueEnum::String(token.value.clone())),
                line: token.line.clone(),
            }),
            TokenType::Char => Box::new(ast::Value {
                value: Some(ast::ValueEnum::Char(token.value.parse::<char>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Bool => Box::new(ast::Value {
                value: Some(ast::ValueEnum::Bool(token.value.parse::<bool>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Identifier => {
                if tokens.get(i + 1).unwrap().token_type == TokenType::OpenParen {
                    Box::new(ast::Function {
                        identifier: token.value.clone(),
                        params: None,
                        body: None,
                        line: token.line.clone(),
                    })
                } else {
                    Box::new(ast::Variable {
                        identifier: token.value.clone(),
                        var_type: None,
                        line: token.line.clone(),
                    })
                }
            }
            _ => {
                print_err(
                    &token.line,
                    &format!("Failed to parse a token: {}", token.value),
                    None,
                );
                continue;
            }
        };
        nodes.push(new_node);
    }

    Some(nodes)
}
