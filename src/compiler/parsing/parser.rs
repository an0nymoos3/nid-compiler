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

use super::ast::{self, Variable};
use super::ast::{alloc_node, dealloc_node};
use super::lexer::{Token, TokenType};
use super::scope_stack::ParserStack;
use std::collections::VecDeque;
use std::ptr::null_mut;

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    let nodes = generate_nodes(tokens)?;

    let mut ast: ast::Ast<dyn ast::Node> = ast::Ast::new(nodes);

    // Process the AST
    ast.body = parse_scopes(&mut VecDeque::from(ast.body));
    move_indicators(&mut ast.body);
    populate_func_fields(&mut ast);
    infer_types(&ast);

    if parse_assignments(&mut ast).is_err() {
        return None;
    }

    Some(ast)
}

/// First step of building AST, create nodes from Tokens.
/// These nodes are stored in the order they were parsed by the
/// tokenizer. They get grouped and sorted into a proper
/// AST in later steps.
fn generate_nodes(tokens: VecDeque<Token>) -> Option<Vec<*mut dyn ast::Node>> {
    let mut failed_ast: bool = false;
    let mut nodes: Vec<*mut dyn ast::Node> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        let new_node: *mut dyn ast::Node = match token.token_type {
            TokenType::Integer => alloc_node(ast::Value {
                value: Some(ast::ValueEnum::Int(token.value.parse::<i16>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Floating => alloc_node(ast::Value {
                value: Some(ast::ValueEnum::Float(token.value.parse::<f32>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::String => alloc_node(ast::Value {
                value: Some(ast::ValueEnum::String(token.value.clone())),
                line: token.line.clone(),
            }),
            TokenType::Char => alloc_node(ast::Value {
                value: Some(ast::ValueEnum::Char(token.value.parse::<char>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Bool => alloc_node(ast::Value {
                value: Some(ast::ValueEnum::Bool(token.value.parse::<bool>().unwrap())),
                line: token.line.clone(),
            }),
            TokenType::Identifier => {
                if tokens.get(i + 1).unwrap().token_type == TokenType::OpenParen {
                    alloc_node(ast::Function {
                        identifier: token.value.clone(),
                        params: Vec::new(),
                        body: null_mut(),
                        return_type: None,
                        line: token.line.clone(),
                    })
                } else {
                    alloc_node(ast::Variable {
                        identifier: token.value.clone(),
                        var_type: None,
                        line: token.line.clone(),
                    })
                }
            }
            TokenType::Assignment => alloc_node(ast::Assignment {
                var: null_mut(),
                expression: null_mut::<u32>(),
                line: token.line.clone(),
            }),
            TokenType::OpenParen => alloc_node(ast::EmptyNode {
                token_type: TokenType::OpenParen,
                line: token.line.clone(),
            }),
            TokenType::CloseParen => alloc_node(ast::EmptyNode {
                token_type: TokenType::CloseParen,
                line: token.line.clone(),
            }),
            TokenType::OpenScope => alloc_node(ast::Block {
                body: Vec::new(),
                line: token.line.clone(),
            }),
            TokenType::CloseScope => alloc_node(ast::EmptyNode {
                token_type: TokenType::CloseScope,
                line: token.line.clone(),
            }),
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
                        failed_ast = true;
                        ast::BinaryOperator::Add // Use a default value to allow the compiler to
                                                 // catch more issues while parsing.
                    }
                };
                alloc_node(ast::BinaryExpression {
                    left: null_mut::<u32>(),
                    op: Some(operator),
                    right: null_mut::<u32>(),
                    line: token.line.clone(),
                })
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
                        failed_ast = true;
                        ast::ConditionalOperator::Eq // Use a default value to allow the compiler to
                                                     // catch more issues while parsing.
                    }
                };
                alloc_node(ast::Condition {
                    left: null_mut::<u32>(),
                    operator: Some(operator),
                    right: null_mut::<u32>(),
                    line: token.line.clone(),
                })
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
                alloc_node(ast::Condition {
                    left: null_mut::<u32>(),
                    operator: Some(operator),
                    right: null_mut::<u32>(),
                    line: token.line.clone(),
                })
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
                        failed_ast = true;
                        ast::TypeEnum::Int // Use a default value to allow the compiler to
                                           // catch more issues while parsing.
                    }
                };
                alloc_node(ast::Indicator {
                    var_type,
                    line: token.line.clone(),
                })
            }
            TokenType::Loop => alloc_node(ast::Loop {
                condition: null_mut(),
                body: null_mut(),
                line: token.line.clone(),
            }),
            TokenType::Branch => alloc_node(ast::Branch {
                condition: null_mut(),
                true_body: null_mut(),
                false_body: null_mut(),
                line: token.line.clone(),
            }),
            TokenType::Seperator => alloc_node(ast::EmptyNode {
                token_type: TokenType::Seperator,
                line: token.line.clone(),
            }),
            TokenType::Return => alloc_node(ast::Return {
                return_value: null_mut::<u32>(),
                line: token.line.clone(),
            }),
            TokenType::Asm => alloc_node(ast::Asm { code: Vec::new() }),
            TokenType::Eol => alloc_node(ast::EmptyNode {
                token_type: TokenType::Eol,
                line: token.line.clone(),
            }),

            TokenType::Eof => alloc_node(ast::EmptyNode {
                token_type: TokenType::Eof,
                line: token.line.clone(),
            }),
            _ => {
                print_err(
                    &token.line,
                    &format!(
                        "Failed to parse a token: {} of type: {:?}",
                        token.value, token.token_type
                    ),
                    None,
                );
                failed_ast = true;
                continue;
            }
        };
        nodes.push(new_node);
    }

    if failed_ast {
        return None;
    }

    Some(nodes)
}

/// Performs necessary nesting of AST for later parsing
/// such as bodies of if-statements and loops, or just
/// function bodies of regular bodies.
fn parse_scopes(body: &mut VecDeque<*mut dyn ast::Node>) -> Vec<*mut dyn ast::Node> {
    let mut new_body: Vec<*mut dyn ast::Node> = Vec::new();

    while !body.is_empty() {
        let cur_node_ptr = body.pop_front().unwrap();

        unsafe {
            // New opening {
            if (*cur_node_ptr).get_type() == ast::AstType::Block {
                let block_ptr = cur_node_ptr as *mut ast::Block;
                if !(*block_ptr).body.is_empty() {
                    print_err(
                        &(*block_ptr).line,
                        "Did not expect code block to already contain code!",
                        None,
                    );
                    panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                }

                (*(cur_node_ptr as *mut ast::Block)).body = parse_scopes(body);
            }

            // Exit early if scope/block is closed
            if (*cur_node_ptr).get_type() == ast::AstType::Empty
                && (*(cur_node_ptr as *mut ast::EmptyNode)).token_type == TokenType::CloseScope
            {
                new_body.push(cur_node_ptr);
                return new_body;
            }
        }

        new_body.push(cur_node_ptr);
    }

    new_body
}

/// Moves indicators to the next nodes in the Vec of nodes.
fn move_indicators(block: &mut Vec<*mut dyn Node>) {
    let mut remove_indexes: Vec<usize> = Vec::new();

    unsafe {
        for i in 0..block.len() {
            if (*block[i]).get_type() == ast::AstType::Block {
                move_indicators(&mut (*(block[i] as *mut ast::Block)).body)
            }

            if (*block[i]).get_type() == ast::AstType::Indicator {
                let node_type: ast::AstType = (*block[i + 1]).get_type();
                let indicator_type = (*(block[i] as *mut ast::Indicator)).var_type.clone();

                match node_type {
                    ast::AstType::Function => {
                        let func_node_ptr = block[i + 1] as *mut ast::Function;
                        if (*func_node_ptr).return_type.is_some() {
                            print_err(&(*func_node_ptr).line,
                            "Trying to set new type to a function that has already been declared with a type before!",
                            None);
                            panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                        }
                        (*func_node_ptr).return_type = Some(indicator_type);
                        remove_indexes.push(i);

                        // Since we only care about using the Enum field we can now deallocate the
                        // indicator node.
                        dealloc_node(block[i]);
                    }
                    ast::AstType::Variable => {
                        let var_node_ptr = block[i + 1] as *mut ast::Variable;
                        if (*var_node_ptr).var_type.is_some() {
                            print_err(&(*var_node_ptr).line,
                            "Trying to set new type to a variable that has already been declared with a type before!",
                            Some("Remove the type indicator in front of variable!"));
                        }

                        (*var_node_ptr).var_type = Some(indicator_type);
                        remove_indexes.push(i);

                        // Since we only care about using the Enum field we can now deallocate the
                        // indicator node.
                        dealloc_node(block[i]);
                    }
                    _ => {
                        print_err(
                            &(*block[i + 1]).get_line().unwrap(),
                            &format!(
                                "Expected function or variable after type indicator! Found: {:?}",
                                (*block[i + 1]).get_type()
                            ),
                            None,
                        );
                        continue;
                    }
                }
            }
        }
    }

    // Remove the indicators from the code body
    for (iter, index) in remove_indexes.iter().enumerate() {
        block.remove(index - iter);
    }
}

/// Finds parameters and function bodies to populate the fields of
/// function nodes.
fn populate_func_fields(tree: &mut ast::Ast<dyn ast::Node>) {
    let mut remove_indexes: Vec<usize> = Vec::new();

    for (i, item) in tree.body.iter().enumerate() {
        // Add parameters to function
        unsafe {
            if (**item).get_type() == ast::AstType::Function {
                let func_ptr = *item as *mut ast::Function;
                let mut new_params: Vec<*mut dyn ast::Node> = Vec::new();

                if !(*func_ptr).params.is_empty() {
                    print_err(
                        &(*func_ptr).line,
                        &format!(
                            "Function: {}() already has parsed parameters!",
                            (*func_ptr).get_name()
                        ),
                        None,
                    );
                    panic!("INTERNAL COMPILER ERROR! SEE ERROR ABOVE!")
                }

                // Adds parameters to function node
                let mut closing_paren_index: Option<usize> = None;
                for j in i + 1..tree.body.len() {
                    let cur_node_ptr = tree.body[j];

                    if (*cur_node_ptr).get_type() == ast::AstType::Empty {
                        let empty_node_ptr = cur_node_ptr as *mut ast::EmptyNode;

                        if (*empty_node_ptr).token_type == TokenType::CloseParen {
                            remove_indexes.push(j);
                            (*func_ptr).params = new_params;
                            closing_paren_index = Some(j);
                            break;
                        } else if (*empty_node_ptr).token_type == TokenType::Seperator {
                            remove_indexes.push(j);
                        }
                    } else if (*cur_node_ptr).get_type() == ast::AstType::Value
                        || (*cur_node_ptr).get_type() == ast::AstType::Variable
                    {
                        new_params.push(cur_node_ptr);
                        remove_indexes.push(j);
                    } else {
                        print_err(
                            &(*cur_node_ptr).get_line().unwrap(),
                            &format!(
                            "Unknown node passed into function parameters! | Type: {:?}, Value: {}",
                                (*cur_node_ptr).get_type(),
                                (*cur_node_ptr).get_name()
                        ),
                            None,
                        );
                    }
                }

                if closing_paren_index.is_none() {
                    print_err(
                        &(*func_ptr).line,
                        &format!(
                            "Function: {}() missing closing parenthesis!",
                            (*func_ptr).get_name()
                        ),
                        Some("Consider closing parenthesis after function decleration."),
                    );
                }

                // Adds fucntion body
                let body_node_ptr = tree.body[closing_paren_index.unwrap() + 1] as *mut ast::Block;

                if (*func_ptr).return_type.is_some() {
                    if (*body_node_ptr).get_type() != ast::AstType::Block {
                        print_err(
                            &(*body_node_ptr).get_line().unwrap(),
                            "Expected a function body after function decleration!",
                            Some("Consider adding a function body!"),
                        );
                    } else {
                        (*func_ptr).body = body_node_ptr;
                        remove_indexes.push(closing_paren_index.unwrap() + 1);
                    }
                } else if (*func_ptr).return_type.is_none()
                    && (*body_node_ptr).get_type() == ast::AstType::Block
                {
                    print_err(
                    &(*body_node_ptr).get_line().unwrap(),
                    "Did not expect a function body as this is not a function decleration!",
                    Some("Remove the function body or add a ; to show that the body is on a new line."),
                );
                }
            }
        }
    }

    // Remove the indicators from the code body
    for (iter, index) in remove_indexes.iter().enumerate() {
        tree.body.remove(index - iter);
    }
}

/// Infers the types of variables throughout the program
fn infer_types(tree: &ast::Ast<dyn ast::Node>) {
    let mut stack: ParserStack = ParserStack::new();
    stack.add_scope();

    let mut body = VecDeque::from(tree.body.clone());

    while !body.is_empty() {
        let ptr = body.pop_front().unwrap();

        unsafe {
            // Variables that have a type already can be added to the stack, while
            // variables lacking have to find an identifier in the stack to
            // get their inference from.
            if (*ptr).get_type() == ast::AstType::Variable {
                let var_ptr = ptr as *mut ast::Variable;

                // If variable has type indicator
                if (*var_ptr).var_type.is_some() {
                    stack
                        .push_var(var_ptr)
                        .expect("INTERNAL COMPILER ERROR WHILE INFERING VARIABLE TYPES!");
                    continue;
                }

                // If variable does not have a type indicator
                match stack.find_var(&(*var_ptr).identifier) {
                    Some(other_var_ptr) => (*var_ptr).var_type = (*other_var_ptr).var_type.clone(),
                    None => {
                        print_err(
                            &(*var_ptr).line,
                            &format!("Can't infer the type of {}", (*var_ptr).identifier),
                            Some(&format!(
                                "Consider adding a type indicator in front of {}, e.g. int {}",
                                (*var_ptr).identifier,
                                (*var_ptr).identifier
                            )),
                        );
                    }
                }
            }

            // For blocks we just append the body to the program we're parsing
            if (*ptr).get_type() == ast::AstType::Block {
                // When new block starts, create a new scope, for potentially new declared
                // variables
                stack.add_scope();

                let block_ptr = ptr as *mut ast::Block;
                let mut block_body = VecDeque::from((*block_ptr).body.clone());

                while !block_body.is_empty() {
                    body.push_front(block_body.pop_back().unwrap());
                }
            }

            // In the function Node 2 things have to be done.
            // 1. Any parameters have to be pushed.
            // 2. The body has to be appended to the program body we're parsing
            if (*ptr).get_type() == ast::AstType::Function {
                // When new block starts, create a new scope, for potentially new declared
                // variables
                stack.add_scope();

                let func_ptr = ptr as *mut ast::Function;

                // Add parameter variables
                for param_ptr in (*func_ptr).params.iter() {
                    if (**param_ptr).get_type() == ast::AstType::Variable {
                        let var_ptr = *param_ptr as *mut Variable;

                        if (*var_ptr).var_type.is_some() {
                            stack
                                .push_var(var_ptr)
                                .expect("INTERNAL COMPILER ERROR WHILE INFERING VARIABLE TYPES!");
                        } else {
                            print_err(
                                &(*var_ptr).line,
                                &format!(
                                    "Parameter {} is missing type decleration!",
                                    (*var_ptr).identifier
                                ),
                                Some(&format!(
                                    "Consider adding a type indicator in front of {}, e.g. int {}",
                                    (*var_ptr).identifier,
                                    (*var_ptr).identifier
                                )),
                            );
                        }
                    }
                }

                // Append the body to the program
                let mut func_body = VecDeque::from((*(*func_ptr).body).body.clone());
                while !func_body.is_empty() {
                    body.push_front(func_body.pop_back().unwrap());
                }
            }

            // Remove a scope when moving out of a body
            if (*ptr).get_type() == ast::AstType::Empty
                && (*(ptr as *mut ast::EmptyNode)).token_type == TokenType::CloseScope
            {
                stack.remove_scope();
            }
        }
    }
}

fn parse_assignments(tree: &mut ast::Ast<dyn ast::Node>) -> Result<(), ()> {
    let mut failed_parsing: bool = false;
    let mut body: VecDeque<*mut dyn ast::Node> = VecDeque::from(tree.body.clone());
    let mut last_node_ptr: *mut dyn ast::Node = null_mut::<u32>();

    let mut remove_indexes: Vec<usize> = Vec::new();

    let mut i: usize = 0;
    while !body.is_empty() {
        let ptr = body.pop_front().unwrap();

        unsafe {
            if (*ptr).get_type() == ast::AstType::Assignment {
                if (*last_node_ptr).get_type() == ast::AstType::Value {
                    print_err(
                        &(*ptr).get_line().unwrap(),
                        "Cannot assign a new value to a constant!",
                        None,
                    );
                    failed_parsing = true;
                }

                println!("{:?}", (*last_node_ptr).get_type());
                if (*last_node_ptr).get_type() == ast::AstType::Variable {
                    let assign_ptr = ptr as *mut ast::Assignment;
                    let var_ptr = last_node_ptr as *mut ast::Variable;

                    (*assign_ptr).var = var_ptr;
                    remove_indexes.push(i - 1);
                }
            }

            // If block was found, copy all pointers over to body
            if (*ptr).get_type() == ast::AstType::Block {
                let block_ptr = ptr as *mut ast::Block;
                for node_ptr in (*block_ptr).body.iter() {
                    body.push_back(*node_ptr);
                }
            }

            // If function was found, copy all pointers over to body
            if (*ptr).get_type() == ast::AstType::Block {
                let func_ptr = ptr as *mut ast::Function;
                let block_ptr = (*func_ptr).body;
                for node_ptr in (*block_ptr).body.iter() {
                    body.push_back(*node_ptr);
                }
            }
        }

        last_node_ptr = ptr;
        i += 1;
    }

    // Remove pointers to variables, expressions, etc.. that have been moved into assignment statements
    for (i, idx) in remove_indexes.iter().enumerate() {
        tree.body.remove(idx - i);
    }

    if failed_parsing {
        return Err(());
    }
    Ok(())
}
