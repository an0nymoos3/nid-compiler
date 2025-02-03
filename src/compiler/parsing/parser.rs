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
    Pointer,     // Same as in C and C++, points to a memory address
    Reference,   // -- || --
    Return,      // Return statement
    Asm,         // Allows for inline assembly code
    Eol,         // End of line, basically ; representing end of line.
    Eof, // Represents the end of the code (EOF all caps appears to be a reserved word of some kind)
    Macro, // Basic macro functionality, such as allocating memory that the compiler is not allowed
         // to touch
*/

use crate::utils::error::print_err;

use super::ast;
use super::lexer::{Token, TokenType};
use std::collections::VecDeque;

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    let nodes = match generate_nodes(tokens) {
        Some(n) => n,
        None => return None,
    };

    let ast: ast::Ast<dyn ast::Node> = ast::Ast::new(nodes);

    Some(ast)
}

/// First step of building AST, create nodes from Tokens.
/// These nodes are stored in the order they were parsed by the
/// tokenizer. They get grouped and sorted into a proper
/// AST in later steps.
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
                        return_type: None,
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
            TokenType::Assignment => Box::new(ast::Assignment {
                type_dec: None,
                var: None,
                expression: None,
                line: token.line.clone(),
            }),
            TokenType::OpenParen => continue,
            TokenType::CloseParen => continue,
            TokenType::OpenScope => Box::new(ast::Block { body: None }),
            TokenType::CloseScope => Box::new(ast::EmptyNode {
                token_type: token.token_type,
                line: token.line.clone(),
            }),
            TokenType::ArrayAccessOpen => todo!(),
            TokenType::ArrayAccessClose => todo!(),
            TokenType::BinaryOperator => {
                let operator: ast::BinaryOperator = match token.value.as_str() {
                    "+" => ast::BinaryOperator::Add,
                    "-" => ast::BinaryOperator::Sub,
                    "*" => ast::BinaryOperator::Mul,
                    "/" => ast::BinaryOperator::Div,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) as an operator (+, -, *, /)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Box::new(ast::BinaryExpression {
                    left: None,
                    op: Some(operator),
                    right: None,
                    line: token.line.clone(),
                })
            }
            TokenType::Comparison => {
                let operator: ast::ConditionalOperator = match token.value.as_str() {
                    "==" => ast::ConditionalOperator::Eq,
                    "!=" => ast::ConditionalOperator::NotEq,
                    ">" => ast::ConditionalOperator::GreatThan,
                    "<" => ast::ConditionalOperator::LessThan,
                    ">=" => ast::ConditionalOperator::GreatEq,
                    "<=" => ast::ConditionalOperator::LessEq,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) as an comparison (==, <=, ...)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Box::new(ast::Condition {
                    left: None,
                    operator: Some(operator),
                    right: None,
                    line: token.line.clone(),
                })
            }
            TokenType::LogicOperator => {
                let operator: ast::ConditionalOperator = match token.value.as_str() {
                    "!" => ast::ConditionalOperator::Not,
                    "&&" => ast::ConditionalOperator::And,
                    "||" => ast::ConditionalOperator::Or,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) when expected (!, && or ||)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Box::new(ast::Condition {
                    left: None,
                    operator: Some(operator),
                    right: None,
                    line: token.line.clone(),
                })
            }
            TokenType::TypeIndicator => {
                let var_type: ast::TypeEnum = match token.value.as_str() {
                    "int" => ast::TypeEnum::Int,
                    "float" => ast::TypeEnum::Float,
                    "string" => ast::TypeEnum::String,
                    "char" => ast::TypeEnum::Char,
                    "bool" => ast::TypeEnum::Bool,
                    "void" => ast::TypeEnum::Void,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) when expected a type (int, float, string, ...)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Box::new(ast::Indicator {
                    var_type,
                    line: token.line.clone(),
                })
            }
            TokenType::Loop => Box::new(ast::Loop {
                condition: None,
                body: None,
                line: token.line.clone(),
            }),
            TokenType::Branch => Box::new(ast::Branch {
                condition: None,
                true_body: None,
                false_body: None,
                line: token.line.clone(),
            }),
            TokenType::Seperator => Box::new(ast::EmptyNode {
                token_type: TokenType::Seperator,
                line: token.line.clone(),
            }),
            TokenType::Return => Box::new(ast::Return {
                return_value: None,
                line: token.line.clone(),
            }),
            TokenType::Eol => Box::new(ast::EmptyNode {
                token_type: TokenType::Eol,
                line: token.line.clone(),
            }),
            TokenType::Eof => Box::new(ast::EmptyNode {
                token_type: TokenType::Eof,
                line: token.line.clone(),
            }),
            _ => {
                print_err(
                    &token.line,
                    &format!(
                        "Failed to parse a token: {} of type: {:?}",
                        token.value, token.token_type
                    ),
                    None,
                );
                continue;
            }
        };
        nodes.push(new_node);
    }

    Some(nodes)
}

/// Links nodes together that are related to variable assignments.
fn link_assignment_nodes(tree: &mut ast::Ast<dyn ast::Node>) {
    for (i, node) in tree.body.iter_mut().enumerate() {
        if node.get_type() == ast::AstType::Assignment {
            let assign: &ast::Assignment = node.as_any().downcast_ref::<ast::Assignment>().unwrap();
        }
    }
}
