use std::fs;

use crate::{
    compiler::ast::{Ast, Node},
    compiler::lexer::{export_tokens, tokenize},
    compiler::parser::generate_ast,
    utils::command_line::Args,
};

use super::ast::export_ast;

/// The main compile function. Takes care of the overall logic of compilation while handing out the
/// details to helper functions.
pub fn compile(args: &Args) -> String {
    let output_name: String = args.filename.to_string().replace(".nid", ".ass");
    let source_code = read_file(&args.filename);

    let mut tokens = tokenize(source_code);

    if args.debug {
        export_tokens(&tokens);
    }

    let ast: Ast<dyn Node> = generate_ast(&mut tokens);

    if args.debug {
        export_ast(&ast);
    }

    output_name
}

/// Reads entire file as a long contious string.
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file contents!")
}
