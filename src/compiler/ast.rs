#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[derive(Debug)]
pub struct Ast<T: Node + ?Sized> {
    pub body: Vec<Box<T>>,
}

pub trait Node {}

pub struct FuncDecleration;
pub struct VarDecleration;
pub struct ArrayDecleration;
pub struct StructDecleration;
pub struct EnumDecleration;
pub struct Assignment;
pub struct Branch;
pub struct Loop;
pub struct Block {
    pub body: Vec<Box<dyn Node>>,
}
pub struct Return;
pub struct FunctionCall;
pub struct UnaryExpression;
pub struct BinaryExpression;
pub struct TypeSpecifier;
pub struct ArrayAccess;
pub struct Eol;
pub struct Debug;

impl Node for FuncDecleration {}
impl Node for VarDecleration {}
impl Node for ArrayDecleration {}
impl Node for StructDecleration {}
impl Node for EnumDecleration {}
impl Node for Assignment {}
impl Node for Branch {}
impl Node for Loop {}
impl Node for Block {}
impl Node for Return {}
impl Node for FunctionCall {}
impl Node for UnaryExpression {}
impl Node for BinaryExpression {}
impl Node for TypeSpecifier {}
impl Node for ArrayAccess {}
impl Node for Eol {}
impl Node for Debug {}

/// Debugging function. Prints all nodes in AST to terminal. TODO: Export to file instead of printing.
pub fn export_ast<T: Node + ?Sized>(ast: &Ast<T>) {
    println!();
    //traverse_ast_body(&ast.body, 0);
    println!("\n");
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body(body: &[Box<dyn Node>], depth: i32) {
    print_branch(depth);

    for node in body.iter() {}
}

/// Pretty printing function for drawing an AST
fn print_branch(depth: i32) {
    let mut branch: String = String::from("|");
    for _ in 0..depth {
        branch.push('-');
    }
    print!("{} ", branch);
}
