/*
* This file handles the logic of parsing the lexed Tokens into some sort of
* Abstract Syntax Tree.
*/

use super::ast::{self, Node, Value, ValueEnum, Variable};
use super::lexer::{Token, TokenType};
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> ast::Ast<dyn ast::Node> {
    let body: Vec<Box<dyn ast::Node>> = parse_body(tokens);
    let mut ast: ast::Ast<dyn ast::Node> = ast::Ast::new(body);
    hash_variables(&mut ast.body, "root");
    ast
}

/// Traverses AST and hashes variables based on their location in the program.
pub fn hash_variables(ast: &mut [Box<dyn Node>], path: &str) {
    for node in ast.iter_mut() {
        /*
         * Hash variables
         */
        if let Some(var) = node.as_any_mut().downcast_mut::<ast::Variable>() {
            var.identifier = variable_hasher(&var.identifier, path).to_string();

        /*
         * Hash function names
         */
        } else if let Some(func) = node.as_any_mut().downcast_mut::<ast::Function>() {
            let new_path: String = format!("{}{}", path, func.get_name());
            hash_variables(&mut func.body.body, &new_path);

        /*
         * Hash variables inside assignment statement
         */
        } else if let Some(assign) = node.as_any_mut().downcast_mut::<ast::Assignment>() {
            if let Some(var) = (*assign.var).as_any_mut().downcast_mut::<ast::Variable>() {
                var.identifier = variable_hasher(&var.identifier, path).to_string();
            }
            if let Some(other_var) = assign
                .expression
                .as_any_mut()
                .downcast_mut::<ast::Variable>()
            {
                other_var.identifier = variable_hasher(&other_var.identifier, path).to_string();
            }
            if let Some(bin_exp) = assign
                .expression
                .as_any_mut()
                .downcast_mut::<ast::BinaryExpression>()
            {
                if let Some(var) = bin_exp.right.as_any_mut().downcast_mut::<ast::Variable>() {
                    var.identifier = variable_hasher(&var.identifier, path).to_string();
                }

                if let Some(var) = bin_exp.left.as_any_mut().downcast_mut::<ast::Variable>() {
                    var.identifier = variable_hasher(&var.identifier, path).to_string();
                }
            }

        /*
         * Hash variables inside of code blocks
         */
        } else if let Some(block) = node.as_any_mut().downcast_mut::<ast::Block>() {
            hash_variables(block.body.as_mut_slice(), path);

        /*
         * Hash variables inside if-statements
         */
        } else if let Some(branch) = node.as_any_mut().downcast_mut::<ast::Branch>() {
            if let Some(left) = &mut branch.condition.left {
                if let Some(l_var) = left.as_any_mut().downcast_mut::<ast::Variable>() {
                    l_var.identifier = variable_hasher(&l_var.identifier, path).to_string();
                }
            }
            if let Some(r_var) = branch
                .condition
                .right
                .as_any_mut()
                .downcast_mut::<ast::Variable>()
            {
                r_var.identifier = variable_hasher(&r_var.identifier, path).to_string();
            }

            hash_variables(branch.true_body.body.as_mut_slice(), path);

            if let Some(false_body) = &mut branch.false_body {
                hash_variables(false_body.body.as_mut_slice(), path);
            }

        /*
         * Hash variables inside if-statements
         */
        } else if let Some(nid_loop) = node.as_any_mut().downcast_mut::<ast::Loop>() {
            if let Some(left) = &mut nid_loop.condition.left {
                if let Some(l_var) = left.as_any_mut().downcast_mut::<ast::Variable>() {
                    l_var.identifier = variable_hasher(&l_var.identifier, path).to_string();
                }
            }
            if let Some(r_var) = nid_loop
                .condition
                .right
                .as_any_mut()
                .downcast_mut::<ast::Variable>()
            {
                r_var.identifier = variable_hasher(&r_var.identifier, path).to_string();
            }
            hash_variables(nid_loop.body.body.as_mut_slice(), path);

        /*
         * Hash variables found in return statements
         */
        } else if let Some(nid_return) = node.as_any_mut().downcast_mut::<ast::Return>() {
            if let Some(return_val) = &mut nid_return.return_value {
                if let Some(var) = return_val.as_any_mut().downcast_mut::<ast::Variable>() {
                    var.identifier = variable_hasher(&var.identifier, path).to_string();
                }
            }
        } else if let Some(builtin) = node.as_any_mut().downcast_mut::<ast::Builtin>() {
            for param in builtin.params.iter_mut() {
                if let Some(var) = param.as_any_mut().downcast_mut::<ast::Variable>() {
                    var.identifier = variable_hasher(&var.identifier, path).to_string();
                }
            }
        }
    }
}

/// Function for being able to recursively parsing the
/// body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Vec<Box<dyn ast::Node>> {
    let mut code_body: Vec<Box<dyn ast::Node>> = Vec::new();

    while !tokens.is_empty() && tokens.front().unwrap().token_type != TokenType::Eof {
        let token: Token = tokens.pop_front().unwrap(); // Assume no error because of while loop
                                                        // above.

        // Create a new Node.
        let new_node: Option<Box<dyn ast::Node>> = match token.token_type {
            /*
             * Inner block, traversed via recursion
             */
            TokenType::OpenScope => Some(Box::new(ast::Block {
                body: parse_body(tokens),
            })),

            /*
             * Exit function if closing scope
             */
            TokenType::CloseScope => {
                return code_body;
            }

            /*
             * Inline assembly
             */
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

            /*
             * Assignemnets
             */
            TokenType::Assignment => {
                let assigned_var: Box<dyn ast::Node> = code_body.pop().unwrap(); // Get last node added,

                // Look for type decleration
                let type_dec: Option<Box<dyn ast::Node>>;
                if !code_body.is_empty() {
                    if code_body[code_body.len() - 1].get_type() == ast::AstType::Type {
                        type_dec = Some(code_body.pop().expect("Could not pop code_body!"));
                    } else {
                        type_dec = None;
                    }
                } else {
                    type_dec = None;
                }

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
                    type_dec,
                    var: assigned_var,
                    expression: assigned_to,
                }))
            }

            /*
             * A branch instruction
             */
            TokenType::Branch => Some(build_branch(tokens)),

            /*
             * Builtin function call
             */
            TokenType::BuiltIn => Some(build_builtin(&token.value, tokens)),

            /*
             * Build variables. or functions
             */
            TokenType::Identifier => {
                if is_function(tokens) {
                    Some(build_function(&token, tokens))
                } else {
                    // Return a variable
                    Some(build_var_or_value(token))
                }
            }

            /*
             * While loops
             */
            TokenType::Loop => Some(build_loop(tokens)),

            /*
             * Nid-lang macros
             */
            TokenType::Macro => Some(build_macro(&token, tokens)),

            /*
             * Return statement
             */
            TokenType::Return => Some(build_return(tokens)),

            /*
             * Parse type indicator
             */
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
                "bool" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Bool(true),
                })),

                &_ => panic!("Unknown type supplied!"),
            },

            /*
             * Not really sure what to do with EOL rn...
             */
            TokenType::Eol => None,

            /*
             * Anything else turns into Debug rn
             */
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

/// Builds a Function Node at current position in tokens.
fn build_function(token: &Token, tokens: &mut VecDeque<Token>) -> Box<ast::Function> {
    tokens.pop_front().unwrap();

    // Build parameters of function
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
                "bool" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Bool(true),
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

    // Get body of function
    tokens.pop_front().unwrap(); // Remove )
    tokens.pop_front().unwrap(); // Remove {
    let body = ast::Block {
        body: parse_body(tokens),
    };

    // Return function node
    Box::new(ast::Function {
        identifier: token.value.clone(),
        params,
        body,
    })
}

/// Builds a builtin function node
fn build_builtin(name: &str, tokens: &mut VecDeque<Token>) -> Box<ast::Builtin> {
    if tokens.pop_front().unwrap().token_type != TokenType::OpenParen {
        panic!("Expected parenthesis after builtin identifier!")
    }

    let mut params: Vec<Box<dyn Node>> = Vec::new();
    let mut token: Token = tokens.pop_front().unwrap();

    while token.token_type != TokenType::CloseParen {
        if token.token_type != TokenType::Seperator {
            params.push(build_var_or_value(token));
        }
        token = tokens.pop_front().unwrap();
    }

    Box::new(ast::Builtin {
        identifier: name.to_string(),
        params,
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
            value: ValueEnum::Int(token.value.parse::<i16>().unwrap()),
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
        TokenType::Bool => Value {
            value: ValueEnum::Bool(token.value.parse::<bool>().unwrap()),
        },
        _ => panic!("Invalid TokenType!"),
    };

    Box::new(val)
}

/// Helper function used to build conditions for both Branches and Loops
fn build_condition(tokens: &mut VecDeque<Token>) -> Box<ast::Condition> {
    // If is_pressed() was sent
    if tokens.front().unwrap().token_type == TokenType::BuiltIn {
        let token = tokens.pop_front().unwrap();
        if token.value != "is_pressed" {
            panic!("Invalid builtin function sent as condition!")
        }
        let builtin = build_builtin(&token.value, tokens);
        tokens.pop_front().unwrap(); // Remove the closing paren
        return Box::new(ast::Condition {
            operator: ast::ConditionalOperator::Eq,
            left: None,
            right: builtin,
        });
    }

    // If only one token was sent as param (eg. while(true) or while(x))
    if tokens[1].token_type == TokenType::CloseParen {
        if tokens.front().unwrap().token_type == TokenType::Bool
            || tokens.front().unwrap().token_type == TokenType::Identifier
        {
            let working_token = tokens.pop_front().unwrap();
            tokens.pop_front().unwrap(); // Remove the closing paren
            return Box::new(ast::Condition {
                operator: ast::ConditionalOperator::Eq,
                left: Some(build_var_or_value(working_token)),
                right: Box::new(Value {
                    value: ValueEnum::Int(1),
                }),
            });
        }
        panic!(
            "Invalid type supplied alone to while loop! Type: {:?}",
            tokens.front().unwrap()
        )
    }

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
        left: left_op,
        right: right_op,
    })
}

/// Performs some checks and returns a Macro type if a valid existed.
fn build_macro(token: &Token, tokens: &mut VecDeque<Token>) -> Box<ast::Macro> {
    if let Some(macro_type) = get_macro_type(&token.value) {
        if tokens.front().unwrap().token_type == TokenType::Assignment {
            tokens.pop_front().unwrap();
            if let Some(value) = tokens.pop_front() {
                return Box::new(ast::Macro {
                    macro_type,
                    macro_value: value
                        .value
                        .parse::<u16>()
                        .expect("Expected u16 as value for macro!"),
                });
            }
        } else {
            panic!("Expected assingment after macro decleration!")
        }
    }
    panic!("Invalid macro found!")
}

/// Returns whether or not an identifier is for function.
fn is_function(tokens: &mut VecDeque<Token>) -> bool {
    tokens.front().unwrap().token_type == TokenType::OpenParen
}

/// Hashes variables in the AST so that each variable gets a unique hash
fn variable_hasher(var_name: &str, branch_path: &str) -> u32 {
    let mut hasher = DefaultHasher::new();
    var_name.hash(&mut hasher);
    branch_path.hash(&mut hasher);
    hasher.finish() as u32
}

/// Returns the valid macro type of Token or None if invalid
fn get_macro_type(m_type: &str) -> Option<ast::MacroType> {
    match m_type {
        "PREALLOCSTART" => Some(ast::MacroType::PreAllocStart),
        "PREALLOCEND" => Some(ast::MacroType::PreAllocEnd),
        _ => None,
    }
}
