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

pub trait Node {
    fn display(&self) -> String;
    fn is_block(&self) -> bool {
        false
    }
    fn get_body(&self) -> &[Box<dyn Node>] {
        &[]
    }
    fn get_name(&self) -> String {
        String::new()
    }
}
impl Display for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
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
    pub condition: Box<Condition>,
    pub true_body: Block,          // If block
    pub false_body: Option<Block>, // Else block
}

/// Condition, used by branches and loops
pub struct Condition {
    pub operator: ConditionalOperator,
    pub left_operand: Option<Box<dyn Node>>, // Variable or value
    pub right_operand: Box<dyn Node>,        // Variable or value
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: Box<Condition>,
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
impl Node for Block {
    fn display(&self) -> String {
        String::from("Block")
    }

    fn is_block(&self) -> bool {
        true
    }

    fn get_body(&self) -> &[Box<dyn Node>] {
        &self.body
    }

    fn get_name(&self) -> String {
        String::from("Block")
    }
}
impl Node for Branch {
    fn display(&self) -> String {
        String::from("Branch")
    }
}
impl Node for Condition {
    fn display(&self) -> String {
        String::from("Condition")
    }
}
impl Node for Loop {
    fn display(&self) -> String {
        String::from("Loop")
    }
}
impl Node for Return {
    fn display(&self) -> String {
        if let Some(return_val) = &self.return_value {
            return format!("Return - {return_val}");
        }
        String::from("Return - None")
    }
}
impl Node for Variable {
    fn display(&self) -> String {
        String::from("Variable")
    }
}
impl Node for Value {
    fn display(&self) -> String {
        String::from("Value")
    }
}
impl Node for DebugNode {
    fn display(&self) -> String {
        String::from("Debugging Node")
    }
}

/// Debugging function. Prints all nodes in AST to terminal. TODO: Export to file instead of printing.
pub fn export_ast(ast: &Ast<dyn Node>) {
    // Build a tree using a TreeBuilder
    let mut tree = ptree::TreeBuilder::new("main()".to_string());
    traverse_ast_body(&mut tree, &ast.body, "program", 1);
    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body(
    tree: &mut ptree::TreeBuilder,
    body: &[Box<dyn Node>],
    branch: &str,
    depth: i32,
) {
    tree.begin_child(branch.to_string());

    for node in body.iter() {
        println!("{node}");
        if node.is_block() {
            traverse_ast_body(tree, node.get_body(), &node.get_name(), depth + 1);
        } else {
            tree.add_empty_child(node.display());
        }
    }
    tree.end_child();
}
