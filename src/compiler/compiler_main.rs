use std::{any::Any, collections::VecDeque, fs};

use crate::{
    compiler::ast::Ast,
    compiler::lexer::{tokenize, Token},
    compiler::parser::generate_ast,
    utils::command_line::Args,
};

use super::ast::{BlockStatement, NodeType};

/// The main compile function. Takes care of the overall logic of compilation while handing out the
/// details to helper functions.
pub fn compile(args: &Args) -> String {
    let output_name: String = args.filename.to_string().replace(".nid", ".ass");
    let source_code = read_file(&args.filename);

    let mut tokens = tokenize(source_code);

    if args.debug {
        export_tokens(&tokens);
    }

    let ast: Ast = generate_ast(&mut tokens);

    if args.debug {
        export_ast(&ast);
    }

    output_name
}

/// Reads entire file as a long contious string.
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file contents!")
}

/// Debugging function. Prints all tokens to terminal. TODO: Export to file instead of printing.
fn export_tokens(tokens: &VecDeque<Token>) {
    for token in tokens {
        println!("Token: {:?}", token);
    }
}

/// Debugging function. Prints all nodes in AST to terminal. TODO: Export to file instead of printing.
fn export_ast(ast: &Ast) {
    traverse_ast_body(&ast.body, 0)
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body(body: &[NodeType], depth: i32) {
    print_branch(depth);

    for node in body.iter() {
        if let NodeType::BlockStatement(code_block) = node {
            println!();
            traverse_ast_body(&code_block.body, depth + 1);
        } else if let NodeType::Eol = node {
            println!();
            print_branch(depth);
        } else {
            print!("{} ", node);
        }
    }
}

/// Pretty printing function for drawing an AST
fn print_branch(depth: i32) {
    let mut branch: String = String::from("|");
    for _ in 0..depth {
        branch.push('-');
    }
    print!("{} ", branch);
}
