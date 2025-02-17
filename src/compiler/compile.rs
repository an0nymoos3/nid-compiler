/*
* This is the entry point of the actual compiler which converts
* human-readable NID code into ASS code.
*/

use std::{path::PathBuf, process::exit};

use super::ir::symbol_table::{generate_table, type_check};
use super::parsing::ast::export_ast;
use super::parsing::lexer::remove_comments;
use crate::utils::lines::{generate_lines, Line};
use crate::{
    compiler::{
        parsing::ast::{Ast, Node},
        parsing::lexer::{export_tokens, tokenize},
        parsing::parser::generate_ast,
    },
    utils::{command_line::Args, hardware_conf::Hardware, nid_fs::read_file},
};

/// The main compile function. Takes care of the overall logic of compilation while handing out the
/// details to helper functions.
pub fn compile(args: &Args, hardware_conf: &Hardware) -> PathBuf {
    let output_name: PathBuf = PathBuf::from(args.filename.to_string().replace(".nid", ".ass"));
    let source_code = read_file(&PathBuf::from(&args.filename));
    let source_code_no_comments = remove_comments(&source_code);

    let lines: Vec<Line> = generate_lines(&source_code_no_comments, &args.filename);

    // Generate Tokens from the source code.
    let tokens = match tokenize(lines) {
        Some(t) => t,
        None => {
            println!("Failed building tokenization! Exiting early...\nCompilation failed.");
            exit(1)
        }
    };
    if args.output_tokens {
        export_tokens(&tokens);
    }

    // Use the Tokens to create an AST of the NID program.
    let ast: Ast<dyn Node> = match generate_ast(tokens) {
        Some(tree) => tree,
        None => {
            println!("Failed building AST! Exiting early...\nCompilation failed.");
            exit(1)
        }
    };
    if args.output_ast {
        export_ast(&ast);
    }

    // Perform a type check of the program using a symbol table
    let sym_table = generate_table(&ast);
    if type_check(&sym_table).is_err() {
        println!("Program failed type check! See errors above.");
        exit(1);
    }

    /*

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
    */

    output_name
}
