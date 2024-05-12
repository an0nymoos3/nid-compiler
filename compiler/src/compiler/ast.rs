use super::lexer::Token;
use std::any::Any;
use std::fmt::{self, Display, Write};

#[derive(Debug, PartialEq)]
pub enum ValueEnum {
    Int(i16),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
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

#[derive(Debug, PartialEq)]
pub enum MacroType {
    PreAllocStart,
    PreAllocEnd,
}

/// Enum for easier identification of Node type
#[derive(Debug, PartialEq, Eq)]
pub enum AstType {
    Asm,
    Assignment,
    BinaryExpression,
    Block,
    Branch,
    Condition,
    Function,
    Loop,
    Return,
    Type,
    Variable,
    Value,
    Macro,
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
    fn as_any(&self) -> &dyn Any; // Method needed for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

pub struct Asm {
    pub code: Vec<Token>,
}

pub struct Assignment {
    pub type_dec: Option<Box<dyn Node>>, // Optional type specifier, used for new variables
    pub var: Box<dyn Node>,              // Var being assigned TODO: Replace with Variable instead
    // of dyn node
    pub expression: Box<dyn Node>, // Varibale or Value being assigned to var
}

pub struct BinaryExpression {
    pub left: Box<dyn Node>,
    pub op: BinaryOperator,
    pub right: Box<dyn Node>,
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
    pub params: Vec<Box<dyn Node>>, // Accept nodes as params, such as values or variables etc
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: Box<Condition>,
    pub body: Block,
}

/// Macros, used for special stuff like telling the compiler what memory it cannot touch
pub struct Macro {
    pub macro_type: MacroType,
    pub macro_value: u16,
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
    pub var_type: Option<ValueEnum>,
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
impl Asm {
    /// Merges Tokens that are the same line of assembly into one line/Token rather than multiple
    pub fn generate_proper_asm(&mut self) {
        let mut new_code: Vec<Token> = Vec::new();
        let mut asm_line: String = String::new();
        let mut i: usize = 0;

        while i < self.code.len() {
            if is_new_asm_instruction(&self.code[i].value) {
                asm_line.push_str(&self.code[i].value);

                i += 1;
                while i < self.code.len() && !is_new_asm_instruction(&self.code[i].value) {
                    asm_line.push_str(&self.code[i].value);
                    i += 1;
                }
                i -= 1; // Decrement once loop exited

                self.code[i].value = asm_line.clone(); // Change the tokens value
                new_code.push(self.code[i].clone()); // Add token to the new assembly code
                asm_line.clear(); // Clear the string for next line of assembly
            }
            i += 1;
        }
        self.code = new_code;
    }
}
impl Node for Asm {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        String::from("Asm")
    }

    fn get_type(&self) -> AstType {
        AstType::Asm
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        for inst in self.code.iter() {
            tree.add_empty_child(inst.value.to_string());
        }

        tree.end_child();
    }
}
impl Node for Assignment {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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

        if let Some(dec) = &self.type_dec {
            dec.traverse_leaves(tree);
        }
        self.var.traverse_leaves(tree);
        self.expression.traverse_leaves(tree);

        tree.end_child();
    }
}
impl Node for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        String::from("BinaryExpression")
    }

    fn get_type(&self) -> AstType {
        AstType::BinaryExpression
    }

    fn is_block(&self) -> bool {
        false
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());
        self.left.traverse_leaves(tree);

        let op: &str = match self.op {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
        };
        tree.add_empty_child(op.to_string());
        self.right.traverse_leaves(tree);
        tree.end_child();
    }
}
impl Node for Block {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
impl Function {
    fn display_params(&self) -> String {
        self.params.iter().fold(String::new(), |mut output, param| {
            if !output.is_empty()
                && (param.get_type() == AstType::Type
                    || param.get_type() == AstType::Variable
                    || param.get_type() == AstType::Value)
            {
                write!(output, ", ").unwrap();
            }
            write!(output, " {} ", param.display()).unwrap();

            output
        })
    }
}
impl Node for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        format!("{}({})", self.get_name(), self.display_params())
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
        tree.end_child();
    }
}
impl Node for Loop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
impl Node for Macro {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        String::from("Macro")
    }

    fn get_type(&self) -> AstType {
        AstType::Macro
    }

    fn has_leaves(&self) -> bool {
        true
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.add_empty_child(format!("Type: {:?}", self.macro_type));
        tree.add_empty_child(format!("Value: {:?}", self.macro_value));

        tree.end_child();
    }
}
impl Node for Return {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        format!("Type: {:?}", self.type_value)
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
impl Value {
    pub fn value_as_i16(&self) -> i16 {
        match self.value {
            ValueEnum::Int(val) => val,
            _ => panic!("Types other than 16-bit integer not currently supported!"),
        }
    }
}
impl Node for Value {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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

/// Debugging function. Prints all nodes in AST to terminal.
pub fn export_ast(ast: &Ast<dyn Node>) {
    println!("AST:");
    // Build a tree using a TreeBuilder
    let mut tree = ptree::TreeBuilder::new("program".to_string());

    for i in 0..ast.body.len() {
        if ast.body[i].get_type() == AstType::Function {
            traverse_ast_body(
                &mut tree,
                ast.body[i + 1].get_body(),
                &ast.body[i].display(),
            )
        }
    }
    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Recursive function to traverse the body of an AST
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

/// For prettier debug AST
fn is_new_asm_instruction(instruciton: &str) -> bool {
    let reserved_words: [&str; 40] = [
        "nop", "ldi", "ld", "st", "psh", "pop", "add", "addi", "sub", "subi", "cmp", "cmpi", "and",
        "andi", "or", "ori", "jmp", "jsr", "ret", "beq", "bne", "bpl", "bmi", "bge", "blt", "mvix",
        "mviy", "kbd", "byk", "bnk", "addix", "addiy", "addiix", "addiiy", "supi", "sdwi", "supii",
        "sdwii", "lflip", "rflip",
    ];
    reserved_words.contains(&instruciton)
}
