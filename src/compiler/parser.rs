use std::collections::VecDeque;

use super::ast::{Ast, Node, NodeType};
use super::lexer::{Token, TokenType};

/// Builds an AST from a queue of tokens.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Ast {
    let mut ast: Ast = Ast { body: Vec::new() };

    while !tokens.is_empty() {
        let token: Token = tokens.pop_front().unwrap(); // Assume error won't happen due to
                                                        // while-loop condition
        let node: Node;

        /*
         * End of file
         */
        if token.token_type == TokenType::Eof {
            break;

        /*
         * End of logical line
         */
        } else if token.token_type == TokenType::Eol {
            node = Node {
                kind: NodeType::Eol,
                value: String::from(";"),
            };

        /*
         * Debugging node
         */
        } else {
            node = Node {
                kind: NodeType::Debug,
                value: String::from("DEBUG"),
            };
        }

        ast.body.push(node);
    }
    ast
}
