use std::fmt::{self, Display};

pub enum ValueEnum {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

pub enum ConditionalOperator {
    And,
    Or,
    Not,
}
#[derive(Debug)]
pub struct Ast<T: Node + ?Sized> {
    pub entry_point: usize, // Entry point index
    pub body: Vec<Box<T>>,
}

impl Ast<dyn Node> {
    /// Finds the entry point of a program (main())
    pub fn new(body: Vec<Box<dyn Node>>) -> Self {
        let mut index: usize = 0;
        loop {
            if index >= body.len() - 1 {
                panic!("main() not found!");
            }

            // TODO: Remove the allow()
            #[allow(clippy::borrowed_box)]
            let node: &Box<dyn Node> = body.get(index).unwrap();
            if node.get_name() == "main" && body.get(index + 1).unwrap().is_block() {
                break; // Only break if it's the main decleration
            }
            index += 1;
        }

        Self {
            body,
            entry_point: index,
        }
    }
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

pub struct Assignment {
    pub var: Box<dyn Node>,          // Var being assigned
    pub var_or_value: Box<dyn Node>, // Varibale or Value being assigned to var
}

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

pub struct Function {
    pub identifier: String,
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

pub struct Type {
    pub type_value: ValueEnum,
}

/// Variable Node
pub struct Variable {
    pub identifier: String, // Identifier (name of variable)
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
impl Node for Assignment {
    fn display(&self) -> String {
        String::from("Assignment")
    }
}
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
impl Node for Function {
    fn display(&self) -> String {
        String::from("Function")
    }
    fn get_name(&self) -> String {
        self.identifier.clone()
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
impl Node for Type {
    fn display(&self) -> String {
        String::from("Type indicator")
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
    println!("AST:");
    // Build a tree using a TreeBuilder
    let mut tree = ptree::TreeBuilder::new("program".to_string());

    traverse_ast_body(
        &mut tree,
        ast.body.get(ast.entry_point + 1).unwrap().get_body(),
        &ast.body.get(ast.entry_point).unwrap().get_name(),
        1,
    );
    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Recursive function to traverse the body of an AST
#[allow(clippy::only_used_in_recursion)] // Ingore the recursion parameter warning.
fn traverse_ast_body(
    tree: &mut ptree::TreeBuilder,
    body: &[Box<dyn Node>],
    branch: &str,
    depth: i32,
) {
    tree.begin_child(branch.to_string());

    for node in body.iter() {
        if node.is_block() {
            traverse_ast_body(tree, node.get_body(), &node.get_name(), depth + 1);
        } else {
            tree.add_empty_child(node.display());
        }
    }
    tree.end_child();
}
