use std::collections::VecDeque;

use super::ast::{self, Value, ValueEnum, Variable};
use super::lexer::{Token, TokenType};

/// Builds an AST from a queue of tokens.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> ast::Ast<dyn ast::Node> {
    let body: Vec<Box<dyn ast::Node>> = parse_body(tokens);
    let ast: ast::Ast<dyn ast::Node> = ast::Ast::new(body);
    ast
}

/// Function for being able to recursively parsing the body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Vec<Box<dyn ast::Node>> {
    let mut code_body: Vec<Box<dyn ast::Node>> = Vec::new();

    while !tokens.is_empty() && tokens.front().unwrap().token_type != TokenType::Eof {
        let token: Token = tokens.pop_front().unwrap(); // Assume no error because of while loop
                                                        // above.

        // Create a new Node.
        let new_node: Option<Box<dyn ast::Node>> = match token.token_type {
            // Inner block, traversed via recursion
            TokenType::OpenScope => Some(Box::new(ast::Block {
                body: parse_body(tokens),
            })),

            // Exit function if closing scope
            TokenType::CloseScope => {
                return code_body;
            }

            // Inline assembly
            TokenType::Asm => {
                if tokens.pop_front().unwrap().token_type != TokenType::OpenScope {
                    panic!("Expected '{{'!");
                }
                let mut asm: ast::Asm = ast::Asm { code: Vec::new() };
                while tokens.front().unwrap().token_type != TokenType::CloseScope {
                    asm.code.push(tokens.pop_front().unwrap());
                }
                asm.generate_proper_asm();
                tokens.pop_front().unwrap();
                Some(Box::new(asm))
            }

            // Assignemnets
            TokenType::Assignment => {
                let assigned_var: Box<dyn ast::Node> = code_body.pop().unwrap(); // Get last node added,
                let assign_to_var: Box<dyn ast::Node> =
                    build_var_or_value(tokens.pop_front().unwrap());

                let assigned_to: Box<dyn ast::Node> =
                    if tokens.front().unwrap().token_type == TokenType::BinaryOperator {
                        let left = assign_to_var;
                        let op = match tokens.pop_front().unwrap().value.as_str() {
                            "+" => ast::BinaryOperator::Add,
                            "-" => ast::BinaryOperator::Sub,
                            "*" => ast::BinaryOperator::Mul,
                            "/" => ast::BinaryOperator::Div,
                            _ => panic!("Invalid token value!"),
                        };
                        let right = build_var_or_value(tokens.pop_front().unwrap());
                        Box::new(ast::BinaryExpression { left, op, right })
                    } else {
                        assign_to_var
                    };

                // Return the assignment struct
                Some(Box::new(ast::Assignment {
                    var: assigned_var,
                    expression: assigned_to,
                }))
            }

            // A branch instruction
            TokenType::Branch => Some(build_branch(tokens)),

            // Build variables. or functions
            TokenType::Identifier => {
                if is_function(tokens) {
                    tokens.pop_front().unwrap();
                    // Return a funtcion
                    let mut params: Vec<Box<dyn ast::Node>> = Vec::new();
                    while tokens.front().unwrap().token_type != TokenType::CloseParen {
                        let mut token = tokens.pop_front().unwrap();

                        if token.token_type == TokenType::Seperator {
                            token = tokens.pop_front().unwrap();
                        }

                        let param: Box<dyn ast::Node> = match token.token_type {
                            TokenType::Identifier => build_var_or_value(token),
                            TokenType::TypeIndicator => match token.value.as_str() {
                                "int" => Box::new(ast::Type {
                                    type_value: ast::ValueEnum::Int(0),
                                }),
                                "float" => Box::new(ast::Type {
                                    type_value: ast::ValueEnum::Float(0.0),
                                }),
                                "string" => Box::new(ast::Type {
                                    type_value: ast::ValueEnum::String("".to_string()),
                                }),
                                "char" => Box::new(ast::Type {
                                    type_value: ast::ValueEnum::Char(' '),
                                }),
                                "void" => Box::new(ast::Type {
                                    type_value: ast::ValueEnum::Void,
                                }),

                                &_ => panic!("Unknown type supplied!"),
                            },
                            TokenType::Integer => build_var_or_value(token),
                            TokenType::Floating => build_var_or_value(token),
                            TokenType::String => build_var_or_value(token),
                            TokenType::Char => build_var_or_value(token),
                            _ => panic!("Unexpected type: {:?}", token.token_type),
                        };
                        params.push(param);
                    }
                    tokens.pop_front().unwrap();
                    Some(Box::new(ast::Function {
                        identifier: token.value,
                        params,
                    }))
                } else {
                    // Return a variable
                    Some(build_var_or_value(token))
                }
            }

            // While loops
            TokenType::Loop => Some(build_loop(tokens)),

            // Return statement
            TokenType::Return => Some(build_return(tokens)),

            // Parse type indicator
            TokenType::TypeIndicator => match token.value.as_str() {
                "int" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Int(0),
                })),
                "float" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Float(0.0),
                })),
                "string" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::String("".to_string()),
                })),
                "char" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Char(' '),
                })),
                "void" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Void,
                })),

                &_ => panic!("Unknown type supplied!"),
            },

            // Not really sure what to do with EOL rn...
            TokenType::Eol => None,

            // Anything else turns into Debug rn
            _ => panic!(
                "Unknown TokenType supplied! TokenType: {:?}",
                token.token_type
            ),
        };

        // Push to body of current scope.
        if let Some(node) = new_node {
            code_body.push(node);
        }
    }

    code_body
}

/*
* Helper functions for building the different Node types.
*/

/// Builds a branch Node at current position in tokens.
fn build_branch(tokens: &mut VecDeque<Token>) -> Box<ast::Branch> {
    if tokens.pop_front().unwrap().token_type != TokenType::OpenParen {
        panic!("Invalid If-statement! No parenthesis!");
    }

    let condition = build_condition(tokens);

    if tokens.pop_front().unwrap().token_type != TokenType::OpenScope {
        panic!("Missing branch body!")
    }

    let true_body: ast::Block = ast::Block {
        body: parse_body(tokens),
    };

    let false_body: Option<ast::Block> = if tokens.front().unwrap().token_type == TokenType::Branch
        && tokens.front().unwrap().value == "else"
    {
        tokens.pop_front().unwrap(); // Remove "else"
        tokens.pop_front().unwrap(); // Remove "{"
        Some(ast::Block {
            body: parse_body(tokens),
        })
    } else {
        None
    };

    Box::new(ast::Branch {
        condition,
        true_body,
        false_body,
    })
}

/// Build a loop Node at current position in tokens.
fn build_loop(tokens: &mut VecDeque<Token>) -> Box<ast::Loop> {
    if tokens.pop_front().unwrap().token_type != TokenType::OpenParen {
        panic!("Invalid If-statement! No parenthesis!");
    }

    let condition = build_condition(tokens);

    if tokens.pop_front().unwrap().token_type != TokenType::OpenScope {
        panic!("Missing loop body!");
    }

    let body: ast::Block = ast::Block {
        body: parse_body(tokens),
    };

    Box::new(ast::Loop { condition, body })
}

/// Builds a return Node at current position in tokens.
fn build_return(tokens: &mut VecDeque<Token>) -> Box<ast::Return> {
    let token = tokens.pop_front().unwrap();

    let return_value: Option<Box<dyn ast::Node>> = if token.token_type == TokenType::Eol {
        None
    } else {
        Some(build_var_or_value(token))
    };

    // Make sure user doesn't try to return anything else, and didn't forget about ';'
    if return_value.is_some() && tokens.pop_front().unwrap().token_type != TokenType::Eol {
        panic!("Missing ;");
    }

    Box::new(ast::Return { return_value })
}

/// Helper function for parsing if token is a variable or value
fn build_var_or_value(token: Token) -> Box<dyn ast::Node> {
    // Check for identifier, indicating Variable
    if token.token_type == TokenType::Identifier {
        return Box::new(Variable {
            identifier: token.value,
            var_type: None,
        });
    };

    // Else assume, Value
    let val = match token.token_type {
        TokenType::Integer => Value {
            value: ValueEnum::Int(token.value.parse::<i32>().unwrap()),
        },
        TokenType::Floating => Value {
            value: ValueEnum::Float(token.value.parse::<f32>().unwrap()),
        },
        TokenType::Char => Value {
            value: ValueEnum::Char(token.value.parse::<char>().unwrap()),
        },
        TokenType::String => Value {
            value: ValueEnum::String(token.value),
        },
        _ => panic!("Invalid TokenType!"),
    };

    Box::new(val)
}

/// Helper function used to build conditions for both Branches and Loops
fn build_condition(tokens: &mut VecDeque<Token>) -> Box<ast::Condition> {
    let left_op: Option<Box<dyn ast::Node>> = if tokens.front().unwrap().token_type
        != TokenType::Comparison
        && tokens.front().unwrap().token_type != TokenType::LogicOperator
    {
        Some(build_var_or_value(tokens.pop_front().unwrap()))
    } else {
        None
    };

    let operator: ast::ConditionalOperator = match tokens.pop_front().unwrap().value.as_str() {
        "!" => ast::ConditionalOperator::Not,
        "!=" => ast::ConditionalOperator::NotEq,
        "==" => ast::ConditionalOperator::Eq,
        ">" => ast::ConditionalOperator::GreatThan,
        "<" => ast::ConditionalOperator::LessThan,
        ">=" => ast::ConditionalOperator::GreatEq,
        "<=" => ast::ConditionalOperator::LessEq,
        _ => panic!("Invalid operaor!"),
    };

    let right_op = build_var_or_value(tokens.pop_front().unwrap());

    if tokens.pop_front().unwrap().token_type != TokenType::CloseParen {
        panic!("No closing paren!")
    }

    Box::new(ast::Condition {
        operator,
        left_operand: left_op,
        right_operand: right_op,
    })
}

/// Returns whether or not an identifier is for function.
fn is_function(tokens: &mut VecDeque<Token>) -> bool {
    tokens.front().unwrap().token_type == TokenType::OpenParen
}
