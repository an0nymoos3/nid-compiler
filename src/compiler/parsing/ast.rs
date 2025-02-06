/*
* This file doesn't handle any compiler logic.
* It contains the Node type and different types of Nodes.
*
* All nodes must implement all the methods of Node to be
* able to be used by the compiler and by extension NID lang.
*/

use super::lexer::{Token, TokenType};
use crate::utils::error::print_err;
use crate::utils::lines::Line;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{self, Display, Write};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueEnum {
    Int(i16),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
    Void,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeEnum {
    Int,
    Float,
    String,
    Char,
    Bool,
    Void,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum ConditionalOperator {
    NotEq,
    Eq,
    GreatThan,
    LessThan,
    GreatEq,
    LessEq,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MacroType {
    PreAllocStart,
    PreAllocEnd,
}

/// Enum for easier identification of Node type
#[derive(Debug, PartialEq, Eq, Clone)]
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
    Builtin,
    Empty,
}

#[derive(Debug)]
pub struct Ast<T: Node + ?Sized> {
    pub entry_point: usize, // Entry point index
    pub body: Vec<Rc<RefCell<T>>>,
}

impl Ast<dyn Node> {
    /// Finds the entry point of a program (main())
    pub fn new(body: Vec<Rc<RefCell<dyn Node>>>) -> Self {
        let mut index: usize = 0;

        loop {
            if index >= body.len() {
                let last_node = body.last().unwrap().borrow_mut();
                print_err(
                    &last_node.get_line().unwrap(),
                    "Missing main()! (Reached EOF when searching for it)",
                    Some("Add a main() function."),
                );
                std::process::exit(1);
            }

            let node = body[index].borrow_mut();
            if node.get_name() == "main" {
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder);

    fn get_name(&self) -> String {
        String::new()
    }

    fn get_type(&self) -> AstType;

    fn get_line(&self) -> Option<Box<Line>>;
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

#[derive(Clone)]
pub struct Asm {
    pub code: Vec<Token>,
}

pub struct Assignment {
    pub type_dec: Option<Box<dyn Node>>, // Optional type specifier, used for new variables
    pub var: Option<Box<Variable>>, // Var being assigned TODO: Replace with Variable instead of dyn node
    pub expression: Option<Box<dyn Node>>, // Varibale or Value being assigned to var
    pub line: Box<Line>,
}

pub struct BinaryExpression {
    pub left: Option<Box<dyn Node>>,
    pub op: Option<BinaryOperator>,
    pub right: Option<Box<dyn Node>>,
    pub line: Box<Line>,
}

/// Code block, essentially scopes ({...})
pub struct Block {
    pub body: Option<Vec<Rc<RefCell<dyn Node>>>>,
    pub line: Box<Line>,
}

/// Branches, (if-statements)
pub struct Branch {
    pub condition: Option<Box<Condition>>,
    pub true_body: Option<Block>,  // If block
    pub false_body: Option<Block>, // Else block
    pub line: Box<Line>,
}

/// Buildint functions
pub struct Builtin {
    pub identifier: Option<String>,
    pub params: Vec<Box<dyn Node>>,
    pub line: Box<Line>,
}

/// Condition, used by branches and loops
pub struct Condition {
    pub operator: Option<ConditionalOperator>,
    pub left: Option<Box<dyn Node>>,  // Variable or value
    pub right: Option<Box<dyn Node>>, // Variable or value
    pub line: Box<Line>,
}

pub struct Function {
    pub identifier: String,
    pub params: Option<Vec<Rc<RefCell<dyn Node>>>>, // Accept nodes as params, such as values or variables etc
    pub body: Option<Rc<RefCell<Block>>>,
    pub return_type: Option<TypeEnum>,
    pub line: Box<Line>,
}

pub struct Indicator {
    pub var_type: TypeEnum,
    pub line: Box<Line>,
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: Option<Box<Condition>>,
    pub body: Option<Block>,
    pub line: Box<Line>,
}

/// Macros, used for special stuff like telling the compiler what memory it cannot touch
pub struct Macro {
    pub macro_type: Option<MacroType>,
    pub macro_value: Option<u16>,
    pub line: Box<Line>,
}

/// Return statement, can either contain a return value or not.
pub struct Return {
    pub return_value: Option<Box<dyn Node>>, // Variable, Value or None
    pub line: Box<Line>,
}

pub struct Type {
    pub type_value: Option<ValueEnum>,
    pub line: Box<Line>,
}

/// Variable Node
pub struct Variable {
    pub identifier: String, // Identifier (name of variable)
    pub var_type: Option<TypeEnum>,
    pub line: Box<Line>,
}

/// Value Node
pub struct Value {
    pub value: Option<ValueEnum>,
    pub line: Box<Line>,
}

pub struct EmptyNode {
    pub token_type: TokenType,
    pub line: Box<Line>,
}

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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        for inst in self.code.iter() {
            tree.add_empty_child(inst.value.to_string());
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        None
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(dec) = &self.type_dec {
            dec.traverse_leaves(tree);
        }

        if let Some(x) = &self.var {
            x.traverse_leaves(tree);
        }

        if let Some(x) = &self.expression {
            x.traverse_leaves(tree);
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());
        if let Some(x) = &self.left {
            x.traverse_leaves(tree);
        }

        if let Some(x) = &self.op {
            let op = match x {
                BinaryOperator::Add => "+",
                BinaryOperator::Sub => "-",
                BinaryOperator::Mul => "*",
                BinaryOperator::Div => "/",
            };
            tree.add_empty_child(op.to_string());
        };

        if let Some(x) = &self.right {
            x.traverse_leaves(tree);
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}
impl Block {
    fn get_body(&self) -> Vec<Rc<RefCell<dyn Node>>> {}
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

    fn get_name(&self) -> String {
        String::from("Block")
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        if let Some(x) = &self.body {
            traverse_ast_body(tree, x, &self.display())
        }
    }

    fn get_line(&self) -> Option<Box<Line>> {
        None
    }
}
impl Branch {
    fn get_true_body(&self) -> &Option<Block> {
        &self.true_body
    }
    fn get_false_body(&self) -> &Option<Block> {
        &self.false_body
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(x) = &self.condition {
            x.traverse_leaves(tree);
        }
        if let Some(x) = &self.true_body {
            x.traverse_leaves(tree);
        }
        if let Some(body) = &self.false_body {
            body.traverse_leaves(tree);
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}
impl Node for Builtin {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        String::from("Builtin")
    }

    fn get_type(&self) -> AstType {
        AstType::Builtin
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.add_empty_child(format!("Identifier: {:?}", self.identifier));

        for param in &self.params {
            tree.add_empty_child(format!("Param: {param}"));
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(left) = &self.left {
            left.traverse_leaves(tree);
        }
        tree.add_empty_child(format!("OP: {:?}", self.operator));
        if let Some(x) = &self.right {
            x.traverse_leaves(tree);
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}
impl Function {
    fn display_params(&self) -> String {
        let output = &self
            .params
            .iter()
            .flatten()
            .fold(String::new(), |mut output, param| {
                if !output.is_empty()
                    && (param.borrow().get_type() == AstType::Type
                        || param.borrow().get_type() == AstType::Variable
                        || param.borrow().get_type() == AstType::Value)
                {
                    write!(output, ", ").unwrap();
                }
                write!(output, " {} ", param.borrow().display()).unwrap();
                output
            });
        output.clone()
    }
}
impl Function {
    fn get_body(&self) -> &Option<Rc<RefCell<Block>>> {
        &self.body
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
        format!("Function: {}({})", self.get_name(), self.display_params())
    }

    fn get_type(&self) -> AstType {
        AstType::Function
    }

    fn get_name(&self) -> String {
        self.identifier.to_owned()
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());
        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}
impl Node for Indicator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        format!("Indicator: {}", self.get_name())
    }

    fn get_type(&self) -> AstType {
        AstType::Function
    }

    fn get_name(&self) -> String {
        format!("{:?}", self.var_type)
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());
        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}
impl Loop {
    fn get_body(&self) -> &Option<Block> {
        &self.body
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(x) = &self.condition {
            x.traverse_leaves(tree);
        }
        if let Some(x) = &self.body {
            x.traverse_leaves(tree);
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.add_empty_child(format!("Type: {:?}", self.macro_type));
        tree.add_empty_child(format!("Value: {:?}", self.macro_value));

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        if let Some(return_val) = &self.return_value {
            return_val.traverse_leaves(tree);
        } else {
            tree.add_empty_child("None".to_string());
        }

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.begin_child(self.display());

        tree.end_child();
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child(self.display());
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
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

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child(self.display());
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}

impl Node for EmptyNode {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn display(&self) -> String {
        String::from("Empty Node")
    }

    fn get_type(&self) -> AstType {
        AstType::Empty
    }

    fn traverse_leaves(&self, tree: &mut ptree::TreeBuilder) {
        tree.add_empty_child("EMPTY NODE!".to_string());
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }
}

/// Debugging function. Prints all nodes in AST to terminal.
pub fn export_ast(ast: &Ast<dyn Node>) {
    println!("AST:");
    // Build a tree using a TreeBuilder
    let mut tree = ptree::TreeBuilder::new("program".to_string());

    for item in ast.body.iter() {
        let node = item.borrow_mut();
        if node.get_type() == AstType::Function {
            let func_node = node.as_any().downcast_ref::<Function>().unwrap();
            match func_node.get_body().as_slice().first() {
                Some(block) => {
                    traverse_ast_body(&mut tree, block.borrow().get_body(), &func_node.get_name());
                }
                None => {
                    print_err(
                        &func_node.line,
                        &format!(
                            "No body associated with function: {}()",
                            func_node.get_name()
                        ),
                        Some("Add a function body."),
                    );
                }
            };
        }
    }
    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Recursive function to traverse the body of an AST
fn traverse_ast_body(tree: &mut ptree::TreeBuilder, body: &[Rc<RefCell<dyn Node>>], branch: &str) {
    tree.begin_child(branch.to_string());

    for item in body.iter() {
        let node = item.borrow_mut();
        match node.get_type() {
            AstType::Function => {
                let func_node = node.as_any().downcast_ref::<Function>().unwrap();
                match func_node.get_body().as_slice().first() {
                    Some(block) => {
                        traverse_ast_body(tree, block.borrow().get_body(), &func_node.get_name());
                    }
                    None => {
                        print_err(
                            &func_node.line,
                            &format!("No body associated with function: {}", func_node.get_name()),
                            Some("Add a function body."),
                        );
                    }
                };
            }
            AstType::Block => {
                let block_node = node.as_any().downcast_ref::<Block>().unwrap();
                traverse_ast_body(tree, block_node.get_body(), &node.get_name());
            }
            _ => {
                node.traverse_leaves(tree);
            }
        }
    }

    tree.end_child();
}

/// For prettier debug AST
fn is_new_asm_instruction(instruction: &str) -> bool {
    let reserved_words: [&str; 25] = [
        "nop", "ldi", "ld", "st", "psh", "pop", "add", "addi", "sub", "subi", "cmp", "cmpi", "and",
        "andi", "or", "ori", "jmp", "jsr", "ret", "beq", "bne", "bpl", "bmi", "bge", "blt",
    ];
    reserved_words.contains(&instruction)
}
