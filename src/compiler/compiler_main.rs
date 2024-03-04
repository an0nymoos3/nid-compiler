use std::{collections::VecDeque, fs};

use crate::{
    compiler::lexer::{tokenize, Token},
    utils::command_line::Args,
};

pub fn compile(args: &Args) -> String {
    let output_name: String = args.filename.to_string().replace(".nid", ".ass");
    let source_code = read_file(&args.filename);

    let tokens = tokenize(source_code);

    if args.debug {
        export_tokens(&tokens);
    }

    output_name
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file contents!")
}

fn export_tokens(tokens: &VecDeque<Token>) {
    for token in tokens {
        println!("Token: {:?}", token);
    }
}
