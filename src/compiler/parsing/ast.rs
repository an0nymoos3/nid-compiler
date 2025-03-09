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
use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{self, Display, Write};
use std::ptr::write;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueEnum {
    Int(i16),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
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
    Indicator,
    Loop,
    Return,
    Variable,
    Value,
    Macro,
    Builtin,
    Empty,
}

#[derive(Debug)]
pub struct Ast {
    pub entry_point: usize, // Entry point index
    pub body: Block,
}

impl Ast {
    /// Finds the entry point of a program (main())
    pub fn new(body_vec: Vec<*mut dyn Node>) -> Self {
        let mut index: usize = 0;

        loop {
            if index >= body_vec.len() {
                let last_node_ptr = *body_vec.last().unwrap();

                if last_node_ptr.is_null() {
                    print_err(
                        &Line::new(index as u32, String::new(), String::from("N/A")),
                        "Missing main()! (Reached EOF when searching for it)",
                        Some("Add a main() function."),
                    );
                }

                unsafe {
                    let line = (*last_node_ptr).get_line().unwrap_or(Box::new(Line::new(
                        index as u32,
                        String::new(),
                        String::from("N/A"),
                    )));

                    print_err(
                        &line,
                        "Missing main()! (Reached EOF when searching for it)",
                        Some("Add a main() function."),
                    );
                }
                std::process::exit(1);
            }

            let node = body_vec[index];
            if !node.is_null() {
                unsafe {
                    if (*node).get_name() == "main" {
                        break; // Only break if it's the main decleration
                    }
                }
            }
            index += 1;
        }

        let body = Block {
            body: body_vec,
            line: Box::new(Line::new(0, String::new(), String::new())),
        };

        Self {
            body,
            entry_point: index,
        }
    }
}

pub trait Node {
    fn display(&self) -> String;

    fn get_name(&self) -> String {
        String::new()
    }

    fn get_type(&self) -> AstType;

    fn get_line(&self) -> Option<Box<Line>>;

    fn get_mem_layout(&self) -> Layout;
}
impl Display for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

/*
* NOTE: I fucking hate this solution. But this is impl
* basically just here to allow null_mut() for
* the Node trait.
*/
impl Node for u32 {
    fn display(&self) -> String {
        unreachable!()
    }

    fn get_type(&self) -> AstType {
        unreachable!()
    }

    fn get_line(&self) -> Option<Box<Line>> {
        unreachable!()
    }

    fn get_name(&self) -> String {
        unreachable!()
    }

    fn get_mem_layout(&self) -> Layout {
        unreachable!()
    }
}

/// Performs a dynamic (heap) allocation of a node
pub fn alloc_node<T: Node + 'static>(node: T) -> *mut dyn Node {
    let layout = node.get_mem_layout();
    let ptr: *mut u8;

    unsafe {
        // Allocate memory
        ptr = alloc(layout);

        // Set value of memory
        write(ptr as *mut T, node);
    }

    if ptr.is_null() {
        panic!("INTERNAL COMPILER ERROR! COULD NOT ALLOCATE NODE!")
    }

    ptr as *mut T
}

/// Deallocates node
pub fn dealloc_node<T: Node + 'static + ?Sized>(node_ptr: *mut T) {
    unsafe {
        let layout = (*node_ptr).get_mem_layout();
        dealloc(node_ptr as *mut u8, layout);
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
    pub var: *mut Variable, // Var being assigned TODO: Replace with Variable instead of dyn node
    pub expression: *mut dyn Node, // Varibale or Value being assigned to var
    pub line: Box<Line>,
}

pub struct BinaryExpression {
    pub left: *mut dyn Node,
    pub op: Option<BinaryOperator>,
    pub right: *mut dyn Node,
    pub line: Box<Line>,
}

/// Code block, essentially scopes ({...})
#[derive(Debug)]
pub struct Block {
    pub body: Vec<*mut dyn Node>,
    pub line: Box<Line>,
}

/// Branches, (if-statements)
pub struct Branch {
    pub condition: *mut Condition,
    pub true_body: *mut Block,  // If block
    pub false_body: *mut Block, // Else block
    pub line: Box<Line>,
}

/// Buildint functions
pub struct Builtin {
    pub identifier: Option<String>,
    pub params: Vec<*mut dyn Node>,
    pub line: Box<Line>,
}

/// Condition, used by branches and loops
pub struct Condition {
    pub operator: Option<ConditionalOperator>,
    pub left: *mut dyn Node,  // Variable or value
    pub right: *mut dyn Node, // Variable or value
    pub line: Box<Line>,
}

pub struct Function {
    pub identifier: String,
    pub params: Vec<*mut dyn Node>, // Accept nodes as params, such as values or variables etc
    pub body: *mut Block,
    pub return_type: Option<TypeEnum>,
    pub line: Box<Line>,
}

pub struct Indicator {
    pub var_type: TypeEnum,
    pub line: Box<Line>,
}

/// Loops, currently ony while is supported
pub struct Loop {
    pub condition: *mut Condition,
    pub body: *mut Block,
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
    pub return_value: *mut dyn Node, // Variable, Value or None
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

#[derive(Debug)]
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
    fn display(&self) -> String {
        String::from("Asm")
    }

    fn get_type(&self) -> AstType {
        AstType::Asm
    }

    fn get_line(&self) -> Option<Box<Line>> {
        None
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Assignment {
    fn display(&self) -> String {
        String::from("Assignment")
    }

    fn get_type(&self) -> AstType {
        AstType::Assignment
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for BinaryExpression {
    fn display(&self) -> String {
        String::from("BinaryExpression")
    }

    fn get_type(&self) -> AstType {
        AstType::BinaryExpression
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Block {
    pub fn remove_item(&mut self, rmv_ptr: *mut dyn Node) {
        for node in self.body.iter() {
            unsafe {
                if (**node).get_type() == AstType::Function {
                    let func_ptr = *node as *mut Function;
                    (*(*func_ptr).body).remove_item(rmv_ptr);
                }
                if (**node).get_type() == AstType::Block {
                    let block_ptr = *node as *mut Block;
                    (*block_ptr).remove_item(rmv_ptr);
                }
            }
        }

        // Remove the matching pointer
        self.body = self
            .body
            .iter()
            .filter(|ptr| !std::ptr::addr_eq(**ptr, rmv_ptr))
            .cloned()
            .collect::<Vec<*mut dyn Node>>();
    }
}
impl Node for Block {
    fn display(&self) -> String {
        String::from("Block")
    }

    fn get_type(&self) -> AstType {
        AstType::Block
    }

    fn get_name(&self) -> String {
        String::from("Block")
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Branch {
    fn display(&self) -> String {
        String::from("Branch")
    }

    fn get_type(&self) -> AstType {
        AstType::Branch
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Builtin {
    fn display(&self) -> String {
        String::from("Builtin")
    }

    fn get_type(&self) -> AstType {
        AstType::Builtin
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Condition {
    fn display(&self) -> String {
        String::from("Condition")
    }

    fn get_type(&self) -> AstType {
        AstType::Condition
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Function {
    fn display_params(&self) -> String {
        let output = self
            .params
            .iter()
            .fold(String::new(), |mut output, param| unsafe {
                if !output.is_empty()
                    && !param.is_null()
                    && ((**param).get_type() == AstType::Indicator
                        || (**param).get_type() == AstType::Variable
                        || (**param).get_type() == AstType::Value)
                {
                    write!(output, ", ").unwrap();
                }
                write!(output, " {} ", (**param).display()).unwrap();
                output
            });
        output.clone()
    }
}
impl Node for Function {
    fn display(&self) -> String {
        format!("Function: {}({})", self.get_name(), self.display_params())
    }

    fn get_type(&self) -> AstType {
        AstType::Function
    }

    fn get_name(&self) -> String {
        self.identifier.to_owned()
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Indicator {
    fn display(&self) -> String {
        format!("Indicator: {}", self.get_name())
    }

    fn get_type(&self) -> AstType {
        AstType::Indicator
    }

    fn get_name(&self) -> String {
        format!("{:?}", self.var_type)
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Loop {
    fn display(&self) -> String {
        String::from("Loop")
    }

    fn get_type(&self) -> AstType {
        AstType::Loop
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Macro {
    fn display(&self) -> String {
        String::from("Macro")
    }

    fn get_type(&self) -> AstType {
        AstType::Macro
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Return {
    fn display(&self) -> String {
        "Return".to_string()
    }

    fn get_type(&self) -> AstType {
        AstType::Return
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Variable {
    fn display(&self) -> String {
        format!("Variable: {} ({:?})", self.identifier, self.var_type)
    }

    fn get_type(&self) -> AstType {
        AstType::Variable
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}
impl Node for Value {
    fn display(&self) -> String {
        format!("Value: {:?}", self.value)
    }

    fn get_type(&self) -> AstType {
        AstType::Value
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}

impl Node for EmptyNode {
    fn display(&self) -> String {
        String::from("Empty Node")
    }

    fn get_type(&self) -> AstType {
        AstType::Empty
    }

    fn get_line(&self) -> Option<Box<Line>> {
        Some(self.line.clone())
    }

    fn get_mem_layout(&self) -> Layout {
        Layout::new::<Self>()
    }
}

/// For prettier debug AST
fn is_new_asm_instruction(instruction: &str) -> bool {
    let reserved_words: [&str; 25] = [
        "nop", "ldi", "ld", "st", "psh", "pop", "add", "addi", "sub", "subi", "cmp", "cmpi", "and",
        "andi", "or", "ori", "jmp", "jsr", "ret", "beq", "bne", "bpl", "bmi", "bge", "blt",
    ];
    reserved_words.contains(&instruction)
}

/// Debugging function. Prints all nodes in AST to terminal.
pub fn export_ast(ast: &Ast) {
    // Build a tree using a TreeBuilder
    let mut tree = ptree::TreeBuilder::new("AST".to_string());

    for item in ast.body.body.iter() {
        ast_display(*item, &mut tree);
    }

    let pretty_tree = tree.build();

    // Print out the tree using default formatting
    ptree::print_tree(&pretty_tree).expect("Failed to draw AST!");
}

/// Adds node correctly to the ptree
/// NOTE: Unsafe function
fn ast_display(node_ptr: *mut dyn Node, tree: &mut ptree::TreeBuilder) {
    unsafe {
        if (*node_ptr).get_type() == AstType::Empty {
            return;
        }

        match (*node_ptr).get_type() {
            AstType::Function => {
                let func_ptr = node_ptr as *mut Function;
                tree.begin_child((*func_ptr).display());
                ast_display((*func_ptr).body, tree);
                tree.end_child();
            }
            AstType::Block => {
                tree.begin_child((*node_ptr).display());
                for node in (*(node_ptr as *mut Block)).body.iter() {
                    ast_display(*node, tree);
                }

                tree.end_child();
            }
            AstType::Assignment => {
                tree.begin_child((*node_ptr).display());

                let assign_ptr = node_ptr as *mut Assignment;

                ast_display((*assign_ptr).var, tree);
                //ast_display((*assign_ptr).expression, tree);

                tree.end_child();
            }
            _ => {
                tree.add_empty_child((*node_ptr).display());
            }
        }
    }
}
