use std::collections::VecDeque;

use super::ast::{self, Value, VarOrValue, Variable};
use super::lexer::{Token, TokenType};

/// Builds an AST from a queue of tokens.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> ast::Ast<dyn ast::Node> {
    let body: Vec<Box<dyn ast::Node>> = parse_body(tokens);
    let ast: ast::Ast<dyn ast::Node> = ast::Ast { body };
    ast
}

/// Function for being able to recursively parsing the body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Vec<Box<dyn ast::Node>> {
    let mut code_body: Vec<Box<dyn ast::Node>> = Vec::new();

    while !tokens.is_empty()
        && tokens.front().unwrap().token_type != TokenType::Eof
        && tokens.front().unwrap().token_type != TokenType::CloseScope
    {
        let token: Token = tokens.pop_front().unwrap(); // Assume no error because of while loop
                                                        // above.

        /*
         * End of file
         */
        if token.token_type == TokenType::Eof {
            panic!("End of file while parsing token!");
        }

        // Create a new Node.
        let new_node: Box<dyn ast::Node> = match token.token_type {
            // Inner block, traversed via recursion
            TokenType::OpenScope => Box::new(ast::Block {
                body: parse_body(tokens),
            }),
            // A branch instruction
            TokenType::Branch => Box::new(ast::DebugNode {}),
            // While loops
            TokenType::Loop => Box::new(ast::DebugNode {}),
            // Return statement
            TokenType::Return => build_return(tokens),

            // Not really sure what to do with EOL rn...
            TokenType::Eol => Box::new(ast::DebugNode {}),

            // Anything else turns into Debug rn
            _ => panic!(
                "Unknown TokenType supplied! TokenType: {:?}",
                token.token_type
            ),
        };

        // Push to body of current scope.
        code_body.push(new_node);
    }

    code_body
}

/// Returns the token after current.
fn next_token() {}

/// Returns the token before current.
fn prev_token() {}

/*
* Helper functions for building the different Node types.
*/

/// Builds a branch Node at current position in tokens.
fn build_branch() {}

/// Build a loop Node at current position in tokens.
fn build_loop() {}

/// Builds a return Node at current position in tokens.
fn build_return(tokens: &mut VecDeque<Token>) -> Box<ast::Return> {
    let token = tokens.pop_front().unwrap();
    let return_value: Option<VarOrValue>;

    if token.token_type == TokenType::Identifier {
        return_value = Some(VarOrValue::Variable(Variable {
            identifier: token.value,
            value: None,
        }));
    } else if token.token_type == TokenType::Integer {
        return_value = Some(VarOrValue::Value(Value::Int(
            token.value.parse::<i32>().unwrap(),
        )));
    } else if token.token_type == TokenType::Floating {
        return_value = Some(VarOrValue::Value(Value::Float(
            token.value.parse::<f32>().unwrap(),
        )));
    } else if token.token_type == TokenType::String {
        return_value = Some(VarOrValue::Value(Value::String(token.value)))
    } else if token.token_type == TokenType::Char {
        return_value = Some(VarOrValue::Value(Value::Char(
            token.value.parse::<char>().unwrap(),
        )))
    } else if token.token_type == TokenType::Eol {
        return_value = None;
    } else {
        panic!("Invalid return token!")
    }

    // Make sure user doesn't try to return anything else, and didn't forget about ';'
    if return_value.is_some() && tokens.pop_front().unwrap().token_type != TokenType::Eol {
        panic!("Missing ;")
    }

    Box::new(ast::Return { return_value })
}
