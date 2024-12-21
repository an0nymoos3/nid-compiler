/*
* This is the entry point of the actual compiler which converts
* human-readable NID code into ASS code.
*/

use std::{path::PathBuf, process::exit};

use crate::{
    compiler::{
        ast::{Ast, Node},
        lexer::{export_tokens, tokenize},
        parser::generate_ast,
    },
    utils::{
        command_line::Args,
        hardware_conf::Hardware,
        nid_fs::{read_file, write_to_file},
    },
};

use super::{ass_gen::program_generator::generate_ass, ast::export_ast, lexer::remove_comments};

/// The main compile function. Takes care of the overall logic of compilation while handing out the
/// details to helper functions.
pub fn compile(args: &Args, hardware_conf: &Hardware) -> PathBuf {
    let output_name: PathBuf = PathBuf::from(args.filename.to_string().replace(".nid", ".ass"));
    let source_code = read_file(&PathBuf::from(&args.filename));
    let source_code_no_comments = remove_comments(&source_code);

    // Generate Tokens from the source code.
    let mut tokens = tokenize(source_code_no_comments);
    if args.verbose {
        export_tokens(&tokens);
    }

    // Use the Tokens to create an AST of the NID program.
    let ast: Ast<dyn Node> = generate_ast(&mut tokens);
    if args.verbose {
        export_ast(&ast);
    }

    // Convert the AST into ASS code.
    let ass_program: Vec<String> = generate_ass(&ast.body, ast.entry_point, hardware_conf);
    if args.verbose {
        println!("Generated ASS code:");
        for (line, inst) in ass_program.iter().enumerate() {
            println!("{} | {}", line + 1, inst);
        }
    }

    // Output the ASS code into a .ass file of the same name.
    if let Err(e) = write_to_file(&ass_program, &output_name) {
        println!(
            "Something went wrong while writing generated ASS to: {err_path} | Err: {e}",
            err_path = output_name.display()
        );
        exit(1);
    }

    output_name
}
