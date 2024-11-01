use std::path::Path;

use crate::{assembler::lexer::tokenize, utils::nid_fs::read_file};

pub fn assemble_program(program: &Path) {
    let code = read_file(program);
    let tokens = tokenize(code);
}
