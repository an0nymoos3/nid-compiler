use std::collections::VecDeque;

use super::ast::{Ast, BlockStatement, Node, NodeType, Value};
use super::lexer::{Token, TokenType};

/// Builds an AST from a queue of tokens.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Ast {
    let mut ast: Ast = Ast { body: Vec::new() };
    while !tokens.is_empty() && tokens.front().unwrap().token_type != TokenType::Eof {
        let token: Token = tokens.pop_front().unwrap();

        if token.token_type == TokenType::OpenScope {
            let body: Vec<NodeType> = parse_body(tokens);
            let code_block: BlockStatement = BlockStatement { body };
            ast.body.push(NodeType::BlockStatement(code_block));
        } else {
            ast.body.push(parse_token(&token));
        }
    }

    ast
}

/// Function for being able to recursively parsing the body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Vec<NodeType> {
    let mut code_body: Vec<NodeType> = Vec::new();

    while tokens.front().unwrap().token_type != TokenType::CloseScope {
        let token: Token = tokens.pop_front().unwrap(); // Assume error won't happen due to while-loop condition

        // Either recursively call parse_body on a new scope or parse token normally
        if token.token_type == TokenType::OpenScope {
            let body: Vec<NodeType> = parse_body(tokens);
            let code_block: BlockStatement = BlockStatement { body };
            code_body.push(NodeType::BlockStatement(code_block));
        } else {
            code_body.push(parse_token(&token));
        }
    }

    code_body
}

fn parse_token(token: &Token) -> NodeType {
    let node: Node;
    let nodetype: NodeType;

    /*
     * End of file
     */
    if token.token_type == TokenType::Eof {
        panic!("End of file while parsing token!");

    /*
     * End of logical line
     */
    } else if token.token_type == TokenType::Eol {
        nodetype = NodeType::Eol;

    /*
     * Parse number.
     */
    } else if token.token_type == TokenType::Number {
        nodetype = NodeType::Literal(Value::Int(token.value.parse::<i32>().unwrap()));

    /*
     * Parse identifiers
     */
    } else if token.token_type == TokenType::Identifier {
        nodetype = NodeType::VarIdentifier(token.value.to_owned());

    /*
     * Assignment token
     */
    } else if token.token_type == TokenType::Assignment {
        nodetype = NodeType::VarDecleration(token.value.to_owned());

    /*
     * Debugging node
     */
    } else {
        node = Node {
            value: String::from("DEBUG"),
        };
        nodetype = NodeType::Debug(node);
    }

    nodetype
}
