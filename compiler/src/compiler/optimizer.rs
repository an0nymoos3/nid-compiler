use super::ast;

/// Traverses through AST and checks if function can be inlined. If it can, it replaces function
/// call node with a body node.
pub fn _inline(ast: &mut ast::Ast<dyn ast::Node>) {
    for i in 0..ast.body.len() {
        // For now, assume all functions can be inlined.
        if ast.body[i].get_type() == ast::AstType::Function {
            let _func_body: &[Box<dyn ast::Node>] = ast.body[i].get_body();

            // TODO: Go through each line of the function body and write it to a Block
            // in place instead of function call.
        }
    }
}
