#[allow(dead_code)] // TODO: Remove later
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[allow(dead_code)] // TODO: Remove later
pub enum ConditionalOperator {
    And,
    Or,
    Not,
}

#[allow(dead_code)] // TODO: Remove later
pub enum VarOrValue {
    Variable(Variable),
    Value(Value),
}

#[derive(Debug)]
pub struct Ast<T: Node + ?Sized> {
    pub body: Vec<Box<T>>,
}

pub trait Node {
    fn display(&self);
}

/*
* Structs used as the Nodes in the AST
*/

/// Code block, essentially scopes ({...})
pub struct Block {
    pub body: Vec<Box<dyn Node>>,
}

/// Branches, (if-statements)
pub struct Branch {
    pub condition: Condition,
    pub true_body: Block,  // If block
    pub false_body: Block, // Else block
}

/// Condition, used by branches and loops
pub struct Condition {
    pub operator: ConditionalOperator,
    pub left_operand: VarOrValue,
    pub right_operand: VarOrValue,
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: Condition,
    pub body: Block,
}

/// Return statement, can either contain a return value or not.
pub struct Return {
    pub return_value: Option<VarOrValue>,
}

/// Variable Node
pub struct Variable {
    pub identifier: String,
    pub value: Option<Value>, // Option allows for uninitialized value. Up to compiler to later verify
                              // that uninitialized variable isn't read.
}

/// Debug trait. TODO: Remove this
pub struct DebugNode;

/*
* Impl the Node trait on all Nodes
*/
impl Node for Block {
    fn display(&self) {
        print!("Code block");
    }
}
impl Node for Branch {
    fn display(&self) {
        print!("Branch statement");
    }
}
impl Node for Condition {
    fn display(&self) {
        print!("Condition");
    }
}
impl Node for Loop {
    fn display(&self) {
        print!("Loop");
    }
}
impl Node for Return {
    fn display(&self) {
        print!("Return");
    }
}
impl Node for Variable {
    fn display(&self) {
        print!("Variable");
    }
}
impl Node for DebugNode {
    fn display(&self) {
        print!("Debugging node!");
    }
}

/// Debugging function. Prints all nodes in AST to terminal. TODO: Export to file instead of printing.
pub fn export_ast<T: Node + ?Sized>(ast: &Ast<T>) {
    println!();
    traverse_ast_body(ast.body.as_slice(), 0);
    println!("\n");
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body<T: Node + ?Sized>(body: &[Box<T>], depth: i32) {
    print_branch(depth);

    for node in body.iter() {
        node.display();
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
