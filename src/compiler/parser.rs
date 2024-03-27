use std::collections::VecDeque;

use super::ast;
use super::lexer::{Token, TokenType};

/// Builds an AST from a queue of tokens.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> ast::Ast<dyn ast::Node> {
    let mut ast: ast::Ast<dyn ast::Node> = ast::Ast { body: Vec::new() };
    while !tokens.is_empty() && tokens.front().unwrap().token_type != TokenType::Eof {
        let token: Token = tokens.pop_front().unwrap();

        if token.token_type == TokenType::OpenScope {
            let body: Vec<Box<dyn ast::Node>> = parse_body(tokens);
            let block: Box<ast::Block> = Box::new(ast::Block { body });
            ast.body.push(block);
        } else {
            ast.body.push(parse_token(&token));
        }
    }

    ast
}

/// Function for being able to recursively parsing the body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Vec<Box<dyn ast::Node>> {
    let mut code_body: Vec<Box<dyn ast::Node>> = Vec::new();

    while tokens.front().unwrap().token_type != TokenType::CloseScope {
        let token: Token = tokens.pop_front().unwrap(); // Assume error won't happen due to while-loop condition

        // Either recursively call parse_body on a new scope or parse token normally
        if token.token_type == TokenType::OpenScope {
        } else {
            code_body.push(parse_token(&token));
        }
    }

    code_body
}

fn parse_token(token: &Token) -> Box<dyn ast::Node> {
    /*
     * End of file
     */
    if token.token_type == TokenType::Eof {
        panic!("End of file while parsing token!");
    } else {
        Box::new(ast::Debug {})
    }
}
