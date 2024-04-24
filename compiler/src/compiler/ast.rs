use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ValueEnum {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[derive(Debug)]
pub enum ConditionalOperator {
    Not,
    NotEq,
    Eq,
    GreatThan,
    LessThan,
    GreatEq,
    LessEq,
}

/// Enum for easier identification of Node type
#[derive(PartialEq, Eq)]
pub enum AstType {
    Assignment,
    Block,
    Branch,
    Condition,
    Function,
    Loop,
    Return,
    Type,
    Variable,
    Value,
    Debug,
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

    fn has_leaves(&self) -> bool;

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder);

    fn is_block(&self) -> bool {
        false
    }

    fn get_body(&self) -> &[Box<dyn Node>] {
        &[]
    }

    fn get_name(&self) -> String {
        String::new()
    }

    fn get_type(&self) -> AstType;
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

    fn get_type(&self) -> AstType {
        AstType::Assignment
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        self.var.traverse_leaves(tree);
        self.var_or_value.traverse_leaves(tree);

        tree.end_child();
    }
}
impl Node for Block {
    fn display(&self) -> String {
        String::from("Block")
    }

    fn get_type(&self) -> AstType {
        AstType::Block
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

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        traverse_ast_body(tree, &self.body, &self.display())
    }
}
impl Node for Branch {
    fn display(&self) -> String {
        String::from("Branch")
    }

    fn get_type(&self) -> AstType {
        AstType::Branch
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        self.condition.traverse_leaves(tree);
        self.true_body.traverse_leaves(tree);
        if let Some(body) = &self.false_body {
            body.traverse_leaves(tree);
        }

        tree.end_child();
    }
}
impl Node for Condition {
    fn display(&self) -> String {
        String::from("Condition")
    }

    fn get_type(&self) -> AstType {
        AstType::Condition
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(left) = &self.left_operand {
            left.traverse_leaves(tree);
        }
        tree.add_empty_child(format!("OP: {:?}", self.operator));
        self.right_operand.traverse_leaves(tree);

        tree.end_child();
    }
}
impl Node for Function {
    fn display(&self) -> String {
        String::from("Function")
    }

    fn get_type(&self) -> AstType {
        AstType::Function
    }

    fn get_name(&self) -> String {
        self.identifier.clone()
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.add_empty_child(format!("Function: {}", self.identifier));

        tree.end_child();
    }
}
impl Node for Loop {
    fn display(&self) -> String {
        String::from("Loop")
    }

    fn get_type(&self) -> AstType {
        AstType::Loop
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        self.condition.traverse_leaves(tree);
        self.body.traverse_leaves(tree);

        tree.end_child();
    }
}
impl Node for Return {
    fn display(&self) -> String {
        "Return".to_string()
    }

    fn get_type(&self) -> AstType {
        AstType::Return
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(return_val) = &self.return_value {
            return_val.traverse_leaves(tree);
        } else {
            tree.add_empty_child("None".to_string());
        }

        tree.end_child();
    }
}
impl Node for Type {
    fn display(&self) -> String {
        String::from("Type indicator")
    }

    fn get_type(&self) -> AstType {
        AstType::Type
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.end_child();
    }
}
impl Node for Variable {
    fn display(&self) -> String {
        format!("Variable: {}", self.identifier)
    }

    fn get_type(&self) -> AstType {
        AstType::Variable
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child(self.display());
    }
}
impl Node for Value {
    fn display(&self) -> String {
        format!("Value: {:?}", self.value)
    }

    fn get_type(&self) -> AstType {
        AstType::Value
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child(self.display());
    }
}
impl Node for DebugNode {
    fn display(&self) -> String {
        String::from("Debugging Node")
    }

    fn get_type(&self) -> AstType {
        AstType::Debug
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child("DEBUGGING NODE!".to_string());
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
    );
    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Recursive function to traverse the body of an AST
#[allow(clippy::only_used_in_recursion)] // Ingore the recursion parameter warning.
fn traverse_ast_body(tree: &mut ptree::TreeBuilder, body: &[Box<dyn Node>], branch: &str) {
    tree.begin_child(branch.to_string());

    for node in body.iter() {
        if node.is_block() {
            traverse_ast_body(tree, node.get_body(), &node.get_name());
        } else if node.has_leaves() {
            node.traverse_leaves(tree);
        }
    }
    tree.end_child();
}
