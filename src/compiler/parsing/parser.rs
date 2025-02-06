/*
* This file handles the logic of parsing the lexed Tokens into some sort of
* Abstract Syntax Tree.
*
* If the main parser function "generate_ast()" fails at some point it will attempt
* to continue compiling the rest of the program to find more potential errors
* before returning None to compile(), which then knows to terminate the compiler.
*
*   Integer, // A value such as "45"
    Floating,
    String,
    Char,
    Bool,
    Identifier,       // Human readable identifier, such as variable name
    Assignment,       // Assigning operator
    OpenParen,        // (
    CloseParen,       // )
    OpenScope,        // {
    CloseScope,       // }
    ArrayAccessOpen,  // [
    ArrayAccessClose, // ]
    BinaryOperator,   // +, -, *, /
    Comparison,       // ==, <=, >=
    LogicOperator,    // !, &&, ||
    TypeIndicator,    // Used to declare variable type and function return
    Loop,
    Branch,      // If conditions etc...
    Seperator,   // for identifying seperations for things like parameters (,)
    Punctuation, // . (used for accessing fields or structs)
    Pointer,     // Same as in C and C++, points to a memory address
    Reference,   // -- || --
    Return,      // Return statement
    Asm,         // Allows for inline assembly code
    Eol,         // End of line, basically ; representing end of line.
    Eof, // Represents the end of the code (EOF all caps appears to be a reserved word of some kind)
    Macro, // Basic macro functionality, such as allocating memory that the compiler is not allowed
         // to touch
*/

use crate::compiler::parsing::ast::Node;
use crate::utils::error::print_err;

use super::ast;
use super::lexer::{Token, TokenType};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    let nodes = match generate_nodes(tokens) {
        Some(n) => n,
        None => return None,
    };

    let mut ast: ast::Ast<dyn ast::Node> = ast::Ast::new(nodes);

    // Process the AST
    ast.body = parse_scopes(&mut VecDeque::from(ast.body));
    move_indicators(&mut ast);
    populate_func_fields(&mut ast);

    Some(ast)
}

/// First step of building AST, create nodes from Tokens.
/// These nodes are stored in the order they were parsed by the
/// tokenizer. They get grouped and sorted into a proper
/// AST in later steps.
fn generate_nodes(tokens: &mut VecDeque<Token>) -> Option<Vec<Rc<RefCell<dyn ast::Node>>>> {
    let mut nodes: Vec<Rc<RefCell<dyn ast::Node>>> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        let new_node: Rc<RefCell<dyn ast::Node>> = match token.token_type {
            TokenType::Integer => Rc::new(RefCell::new(ast::Value {
                value: Some(ast::ValueEnum::Int(token.value.parse::<i16>().unwrap())),
                line: token.line.clone(),
            })),
            TokenType::Floating => Rc::new(RefCell::new(ast::Value {
                value: Some(ast::ValueEnum::Float(token.value.parse::<f32>().unwrap())),
                line: token.line.clone(),
            })),
            TokenType::String => Rc::new(RefCell::new(ast::Value {
                value: Some(ast::ValueEnum::String(token.value.clone())),
                line: token.line.clone(),
            })),
            TokenType::Char => Rc::new(RefCell::new(ast::Value {
                value: Some(ast::ValueEnum::Char(token.value.parse::<char>().unwrap())),
                line: token.line.clone(),
            })),
            TokenType::Bool => Rc::new(RefCell::new(ast::Value {
                value: Some(ast::ValueEnum::Bool(token.value.parse::<bool>().unwrap())),
                line: token.line.clone(),
            })),
            TokenType::Identifier => {
                if tokens.get(i + 1).unwrap().token_type == TokenType::OpenParen {
                    Rc::new(RefCell::new(ast::Function {
                        identifier: token.value.clone(),
                        params: None,
                        body: None,
                        return_type: None,
                        line: token.line.clone(),
                    }))
                } else {
                    Rc::new(RefCell::new(ast::Variable {
                        identifier: token.value.clone(),
                        var_type: None,
                        line: token.line.clone(),
                    }))
                }
            }
            TokenType::Assignment => Rc::new(RefCell::new(ast::Assignment {
                type_dec: None,
                var: None,
                expression: None,
                line: token.line.clone(),
            })),
            TokenType::OpenParen => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::OpenParen,
                line: token.line.clone(),
            })),
            TokenType::CloseParen => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::CloseParen,
                line: token.line.clone(),
            })),
            TokenType::OpenScope => Rc::new(RefCell::new(ast::Block {
                body: None,
                line: token.line.clone(),
            })),
            TokenType::CloseScope => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::CloseScope,
                line: token.line.clone(),
            })),
            TokenType::ArrayAccessOpen => todo!(),
            TokenType::ArrayAccessClose => todo!(),
            TokenType::BinaryOperator => {
                let operator: ast::BinaryOperator = match token.value.as_str() {
                    "+" => ast::BinaryOperator::Add,
                    "-" => ast::BinaryOperator::Sub,
                    "*" => ast::BinaryOperator::Mul,
                    "/" => ast::BinaryOperator::Div,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) as an operator (+, -, *, /)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Rc::new(RefCell::new(ast::BinaryExpression {
                    left: None,
                    op: Some(operator),
                    right: None,
                    line: token.line.clone(),
                }))
            }
            TokenType::Comparison => {
                let operator: ast::ConditionalOperator = match token.value.as_str() {
                    "==" => ast::ConditionalOperator::Eq,
                    "!=" => ast::ConditionalOperator::NotEq,
                    ">" => ast::ConditionalOperator::GreatThan,
                    "<" => ast::ConditionalOperator::LessThan,
                    ">=" => ast::ConditionalOperator::GreatEq,
                    "<=" => ast::ConditionalOperator::LessEq,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) as an comparison (==, <=, ...)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Rc::new(RefCell::new(ast::Condition {
                    left: None,
                    operator: Some(operator),
                    right: None,
                    line: token.line.clone(),
                }))
            }
            TokenType::LogicOperator => {
                let operator: ast::ConditionalOperator = match token.value.as_str() {
                    "!" => ast::ConditionalOperator::Not,
                    "&&" => ast::ConditionalOperator::And,
                    "||" => ast::ConditionalOperator::Or,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) when expected (!, && or ||)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Rc::new(RefCell::new(ast::Condition {
                    left: None,
                    operator: Some(operator),
                    right: None,
                    line: token.line.clone(),
                }))
            }
            TokenType::TypeIndicator => {
                let var_type: ast::TypeEnum = match token.value.as_str() {
                    "int" => ast::TypeEnum::Int,
                    "float" => ast::TypeEnum::Float,
                    "string" => ast::TypeEnum::String,
                    "char" => ast::TypeEnum::Char,
                    "bool" => ast::TypeEnum::Bool,
                    "void" => ast::TypeEnum::Void,
                    _ => {
                        print_err(&token.line, &format!("Internal compiler error! Tried parsing: ({}) when expected a type (int, float, string, ...)", token.value), None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                };
                Rc::new(RefCell::new(ast::Indicator {
                    var_type,
                    line: token.line.clone(),
                }))
            }
            TokenType::Loop => Rc::new(RefCell::new(ast::Loop {
                condition: None,
                body: None,
                line: token.line.clone(),
            })),
            TokenType::Branch => Rc::new(RefCell::new(ast::Branch {
                condition: None,
                true_body: None,
                false_body: None,
                line: token.line.clone(),
            })),
            TokenType::Seperator => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::Seperator,
                line: token.line.clone(),
            })),
            TokenType::Return => Rc::new(RefCell::new(ast::Return {
                return_value: None,
                line: token.line.clone(),
            })),
            TokenType::Eol => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::Eol,
                line: token.line.clone(),
            })),
            TokenType::Eof => Rc::new(RefCell::new(ast::EmptyNode {
                token_type: TokenType::Eof,
                line: token.line.clone(),
            })),
            _ => {
                print_err(
                    &token.line,
                    &format!(
                        "Failed to parse a token: {} of type: {:?}",
                        token.value, token.token_type
                    ),
                    None,
                );
                continue;
            }
        };
        nodes.push(new_node);
    }

    Some(nodes)
}

/// Performs necessary nesting of AST for later parsing
/// such as bodies of if-statements and loops, or just
/// function bodies of regular bodies.
fn parse_scopes(
    body: &mut VecDeque<Rc<RefCell<dyn ast::Node>>>,
) -> Vec<Rc<RefCell<dyn ast::Node>>> {
    let mut new_body: Vec<Rc<RefCell<dyn ast::Node>>> = Vec::new();

    while !body.is_empty() {
        let cur_node = body.pop_front().unwrap();

        // New opening {
        if cur_node.borrow().get_type() == ast::AstType::Block {
            let mut block_node = cur_node.borrow_mut();
            let block = block_node
                .as_any_mut()
                .downcast_mut::<ast::Block>()
                .unwrap();
            if block.body.is_some() {
                print_err(
                    &block.line,
                    "Did not expect code block to already contain code!",
                    None,
                );
                panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
            }
            block.body = Some(parse_scopes(body));
        }

        // Exit early if scope/block is closed
        if cur_node.borrow().get_type() == ast::AstType::Empty
            && cur_node
                .borrow()
                .as_any()
                .downcast_ref::<ast::EmptyNode>()
                .unwrap()
                .token_type
                == TokenType::CloseScope
        {
            return new_body;
        }

        new_body.push(cur_node);
    }

    new_body
}

/// Moves indicators to the next nodes in the Vec of nodes.
fn move_indicators(tree: &mut ast::Ast<dyn ast::Node>) {
    let mut remove_indexes: Vec<usize> = Vec::new();

    for i in 0..tree.body.len() {
        let is_indicator: bool = tree.body[i]
            .borrow()
            .as_any()
            .downcast_ref::<ast::Indicator>()
            .is_some();

        if is_indicator {
            let node_type: ast::AstType = tree.body[i + 1].borrow().get_type();
            let indicator_type = tree.body[i]
                .borrow()
                .as_any()
                .downcast_ref::<ast::Indicator>()
                .unwrap()
                .var_type
                .clone();

            match node_type {
                ast::AstType::Function => {
                    let mut func_node = tree.body[i + 1].borrow_mut();
                    let func = func_node
                        .as_any_mut()
                        .downcast_mut::<ast::Function>()
                        .unwrap();
                    if func.return_type.is_some() {
                        print_err(&func.line,
                            "Trying to set new type to a function that has already been declared with a type before!",
                            None);
                        panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                    }
                    func.return_type = Some(indicator_type);
                    remove_indexes.push(i);
                }
                ast::AstType::Variable => {
                    let mut var_node = tree.body[i + 1].borrow_mut();
                    let var = var_node
                        .as_any_mut()
                        .downcast_mut::<ast::Variable>()
                        .unwrap();

                    if var.var_type.is_some() {
                        print_err(&var.line,
                            "Trying to set new type to a variable that has already been declared with a type before!",
                            Some("Remove the type indicator in front of variable!"));
                    }

                    var.var_type = Some(indicator_type);
                    remove_indexes.push(i);
                }
                _ => {
                    print_err(
                        &tree.body[i + 1].borrow().get_line().unwrap(),
                        &format!(
                            "Expected function or variable after type indicator! Found: {:?}",
                            tree.body[i + 1].borrow().get_type()
                        ),
                        None,
                    );
                    continue;
                }
            }
        }
    }

    // Remove the indicators from the code body
    for (iter, index) in remove_indexes.iter().enumerate() {
        tree.body.remove(index - iter);
    }
}

/// Finds parameters and function bodies to populate the fields of
/// function nodes.
fn populate_func_fields(tree: &mut ast::Ast<dyn ast::Node>) {
    let mut remove_indexes: Vec<usize> = Vec::new();

    for (i, item) in tree.body.iter().enumerate() {
        let mut node = item.borrow_mut();

        if node.get_type() == ast::AstType::Function {
            let func_node = node.as_any_mut().downcast_mut::<ast::Function>().unwrap();
            let mut new_params: Vec<Rc<RefCell<dyn ast::Node>>> = Vec::new();

            if func_node.params.is_some() {
                print_err(
                    &func_node.line,
                    &format!(
                        "Function: {}() already has parsed parameters!",
                        func_node.get_name()
                    ),
                    None,
                );
                panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
            }

            // Adds parameters to function node
            let mut closing_paren_index: Option<usize> = None;
            for j in i + 1..tree.body.len() {
                let cur_node = tree.body[j].clone();
                let borrowed_node = cur_node.borrow();

                if borrowed_node.get_type() == ast::AstType::Empty {
                    let empty_node = borrowed_node
                        .as_any()
                        .downcast_ref::<ast::EmptyNode>()
                        .unwrap();

                    if empty_node.token_type == TokenType::CloseParen {
                        remove_indexes.push(j);
                        func_node.params = Some(new_params);
                        closing_paren_index = Some(j);
                        break;
                    } else if empty_node.token_type == TokenType::Seperator {
                        remove_indexes.push(j);
                    }
                } else if borrowed_node.get_type() == ast::AstType::Value
                    || borrowed_node.get_type() == ast::AstType::Variable
                {
                    new_params.push(cur_node.clone());
                    remove_indexes.push(j);
                } else {
                    print_err(
                        &borrowed_node.get_line().unwrap(),
                        &format!(
                            "Unknown node passed into function parameters! | Type: {:?}, Value: {}",
                            borrowed_node.get_type(),
                            borrowed_node.get_name()
                        ),
                        None,
                    );
                }
            }

            if closing_paren_index.is_none() {
                print_err(
                    &func_node.line,
                    &format!(
                        "Function: {}() missing closing parenthesis!",
                        func_node.get_name()
                    ),
                    Some("Consider closing parenthesis after function decleration."),
                );
            }

            // Adds fucntion body
            let body_node = tree.body[closing_paren_index.unwrap() + 1].clone();
            let borrowed_body = body_node.borrow();

            if func_node.return_type.is_some() {
                if borrowed_body.get_type() != ast::AstType::Block {
                    print_err(
                        &borrowed_body.get_line().unwrap(),
                        "Expected a function body after function decleration!",
                        Some("Consider adding a function body!"),
                    );
                } else {
                    unsafe {
                        // NOTE: Funky pointer coercion, if parsing goes wrong, this can be one of the
                        // first places it happens
                        let body: Rc<RefCell<ast::Block>> =
                            Rc::from_raw(body_node.as_ptr() as *mut RefCell<ast::Block>);
                        func_node.body = Some(body);
                    }
                    remove_indexes.push(closing_paren_index.unwrap() + 1);
                }
            } else if func_node.return_type.is_none()
                && borrowed_body.get_type() == ast::AstType::Block
            {
                print_err(
                    &borrowed_body.get_line().unwrap(),
                    "Did not expect a function body as this is not a function decleration!",
                    Some("Remove the function body or add a ; to show that the body is on a new line."),
                );
            }
        }
    }

    // Remove the indicators from the code body
    for (iter, index) in remove_indexes.iter().enumerate() {
        tree.body.remove(index - iter);
    }
}
