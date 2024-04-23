use super::ast;

/// Traverses through AST and checks if function can be inlined. If it can, it replaces function
/// call node with a body node.
pub fn inline(ast: &mut ast::Ast<dyn ast::Node>) {
    for i in 0..ast.body.len() {
        // For now, assume all functions can be inlined.
        if ast.body[i].get_type() == ast::AstType::Function {
            let body = ast.body[i].get_body();

            // To get around the borrowing issues i'll manually copy everything over to a new vec
            // for now.
            let inlined_body: Vec<Box<dyn ast::Node>> = body.iter().map(|x| x).collect();

            ast.body[i] = Box::new(ast::Block { body: inlined_body });
        }
    }
}
