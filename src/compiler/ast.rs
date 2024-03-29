use std::fmt::{self, Display};

#[allow(dead_code)] // TODO: Remove later
pub enum ValueEnum {
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

#[derive(Debug)]
pub struct Ast<T: Node + ?Sized> {
    pub body: Vec<Box<T>>,
}

pub trait Node {}
impl Display for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/*
* Structs used as the Nodes in the AST.
*
* Wherever Box<dyn Node> is used, any type of Node can be used.
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
    pub left_operand: Box<dyn Node>,  // Variable or value
    pub right_operand: Box<dyn Node>, // Variable or value
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: Condition,
    pub body: Block,
}

/// Return statement, can either contain a return value or not.
pub struct Return {
    pub return_value: Option<Box<dyn Node>>, // Variable, Value or None
}

/// Variable Node
pub struct Variable {
    pub identifier: String,
    pub value: Option<Value>, // Option allows for uninitialized value. Up to compiler to later verify
                              // that uninitialized variable isn't read.
}

/// Value Node
pub struct Value {
    pub value: ValueEnum,
}

/// Debug trait. TODO: Remove this
pub struct DebugNode;

/*
* Impl the Node trait on all Nodes
*/
impl Node for Block {}
impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}
impl Node for Branch {}
impl Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Branch statement")
    }
}
impl Node for Condition {}
impl Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}
impl Node for Loop {}
impl Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loop")
    }
}
impl Node for Return {}
impl Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(return_val) = &self.return_value {
            write!(f, "Return - {return_val}")
        } else {
            write!(f, "Return - None")
        }
    }
}
impl Node for Variable {}
impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Variable")
    }
}
impl Node for Value {}
impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Raw Value")
    }
}
impl Node for DebugNode {}
impl Display for DebugNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debugging node!")
    }
}

/// Debugging function. Prints all nodes in AST to terminal. TODO: Export to file instead of printing.
pub fn export_ast(ast: &Ast<dyn Node>) {
    println!();
    traverse_ast_body(ast.body.as_slice(), 0);
    println!("\n");
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body(body: &[Box<dyn Node>], depth: i32) {
    print_branch(depth);

    for node in body.iter() {
        print!("{node}")
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
