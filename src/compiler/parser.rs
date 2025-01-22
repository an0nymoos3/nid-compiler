/*
* This file handles the logic of parsing the lexed Tokens into some sort of
* Abstract Syntax Tree.
*
* If the main parser function "generate_ast()" fails at some point it will attempt
* to continue compiling the rest of the program to find more potential errors
* before returning None to compile(), which then knows to terminate the compiler.
*/

use super::ast::{self, Assignment, Node, Value, ValueEnum, Variable};
use super::lexer::{Token, TokenType};
use crate::utils::error::print_err;
use crate::utils::error_types::ParserError;
use crate::utils::lines::Line;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

/// Entry point for building AST. It takes a Dequeue of Tokens and iterates over
/// them until EOF is reached, indicating the AST its complete.
pub fn generate_ast(tokens: &mut VecDeque<Token>) -> Option<ast::Ast<dyn ast::Node>> {
    let body: Vec<Box<dyn ast::Node>> = match parse_body(tokens) {
        Ok(tree) => {
            if tree.is_empty() {
                return None;
            }
            tree
        }
        Err(_) => return None,
    };

    let mut ast: ast::Ast<dyn ast::Node> = ast::Ast::new(body);
    hash_variables(&mut ast.body, "root");

    Some(ast)
}

/// Function for being able to recursively parsing the
/// body code.
fn parse_body(tokens: &mut VecDeque<Token>) -> Result<Vec<Box<dyn ast::Node>>, ParserError> {
    let mut failed_build: bool = false;
    let mut code_body: Vec<Box<dyn ast::Node>> = Vec::new();

    while !tokens.is_empty()
        && tokens.front().expect("Internal compiler error!").token_type != TokenType::Eof
    {
        let token: Token = tokens.pop_front().expect("Internal compiler error!"); // Assume no error because of while loop
                                                                                  // above.

        // Create a new Node.
        let new_node: Option<Box<dyn ast::Node>> = match token.token_type {
            /*
             * Inner block, traversed via recursion
             */
            TokenType::OpenScope => Some(Box::new(ast::Block {
                body: parse_body(tokens).unwrap_or(Vec::new()),
            })),

            /*
             * Exit function if closing scope
             */
            TokenType::CloseScope => {
                if failed_build {
                    return Err(ParserError::ParserFailure);
                }
                return Ok(code_body);
            }

            /*
             * Inline assembly
             */
            TokenType::Asm => match parse_asm(&token, tokens) {
                Ok(asm) => Some(Box::new(asm)),
                Err(_) => {
                    failed_build = true;
                    None
                }
            },

            /*
             * Assignments
             */
            TokenType::Assignment => match parse_assigment(&token, tokens, &mut code_body) {
                Ok(assign) => Some(Box::new(assign)),
                Err(_) => {
                    failed_build = true;
                    None
                }
            },

            /*
             * A branch instruction
             */
            TokenType::Branch => match build_branch(tokens) {
                Ok(branch) => Some(Box::new(branch)),
                Err(_) => {
                    failed_build = true;
                    None
                }
            },

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
                    match build_var_or_value(&token) {
                        Some(var_or_val) => Some(var_or_val),
                        None => {
                            print_err(
                                &token.line,
                                &format!("Unknown variable name: {}", token.value),
                                None,
                            );
                            return Err(ParserError::InvalidIdentifier);
                        }
                    }
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
                    line: token.line.clone(),
                })),
                "float" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Float(0.0),
                    line: token.line.clone(),
                })),
                "string" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::String("".to_string()),
                    line: token.line.clone(),
                })),
                "char" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Char(' '),
                    line: token.line.clone(),
                })),
                "void" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Void,
                    line: token.line.clone(),
                })),
                "bool" => Some(Box::new(ast::Type {
                    type_value: ast::ValueEnum::Bool(true),
                    line: token.line.clone(),
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

    if failed_build {
        return Err(ParserError::ParserFailure);
    }
    Ok(code_body)
}

/*
 * Helper functions to break out of parse_body()
 */

/// Parses asm {} blocks found in NID lang.
fn parse_asm(token: &Token, tokens: &mut VecDeque<Token>) -> Result<ast::Asm, ParserError> {
    if tokens
        .pop_front()
        .expect("Internal compiler error!")
        .token_type
        != TokenType::OpenScope
    {
        print_err(
            &token.line,
            "Missing expected {{ after asm block declaration!",
            Some("Add missing {{ after asm."),
        );
        return Err(ParserError::MissingToken);
    }

    let mut asm: ast::Asm = ast::Asm { code: Vec::new() };
    while tokens.front().expect("Internal compiler error!").token_type != TokenType::CloseScope {
        asm.code
            .push(tokens.pop_front().expect("Internal compiler error!"));
    }

    asm.generate_proper_asm();
    tokens.pop_front().expect("Internal compiler error!");

    Ok(asm)
}

/// Parses assignments done in NID lang.
fn parse_assigment(
    token: &Token,
    tokens: &mut VecDeque<Token>,
    code_body: &mut Vec<Box<dyn ast::Node>>,
) -> Result<Assignment, ParserError> {
    let assigned_var: Box<dyn ast::Node> = code_body.pop().expect("Internal compiler error!"); // Get last node added,

    // Look for type declaration
    let type_dec: Option<Box<dyn ast::Node>>;
    if !code_body.is_empty() {
        if code_body[code_body.len() - 1].get_type() == ast::AstType::Type {
            type_dec = Some(code_body.pop().expect("Internal compiler error!"));
        } else {
            type_dec = None;
        }
    } else {
        type_dec = None;
    }

    let assign_to_var: Box<dyn ast::Node> =
        match build_var_or_value(&tokens.pop_front().expect("Internal compiler error!")) {
            Some(var_or_val) => var_or_val,
            None => {
                print_err(
                    &token.line,
                    &format!("Unknown variable name: {}", token.value),
                    None,
                );

                return Err(ParserError::InvalidIdentifier);
            }
        };

    let assigned_to: Box<dyn ast::Node> = if tokens
        .front()
        .expect("Internal compiler error!")
        .token_type
        == TokenType::BinaryOperator
    {
        let left = assign_to_var;

        let op = match tokens
            .pop_front()
            .expect("Internal compiler error!")
            .value
            .as_str()
        {
            "+" => ast::BinaryOperator::Add,
            "-" => ast::BinaryOperator::Sub,
            "*" => ast::BinaryOperator::Mul,
            "/" => ast::BinaryOperator::Div,
            _ => {
                print_err(
                    &token.line,
                    "Invalid binary operator used!",
                    Some("Consider using +, -, * or /."),
                );
                return Err(ParserError::InvalidOperator);
            }
        };

        let right = match build_var_or_value(&tokens.pop_front().expect("Internal compiler error!"))
        {
            Some(var_or_val) => var_or_val,
            None => {
                print_err(
                    &token.line,
                    &format!("Unknown variable name or value: {}", token.value),
                    None,
                );

                return Err(ParserError::UnknownVarOrVal);
            }
        };
        Box::new(ast::BinaryExpression {
            left,
            op,
            right,
            line: token.line.clone(),
        })
    } else {
        assign_to_var
    };

    // Return the assignment struct
    Ok(ast::Assignment {
        type_dec,
        var: assigned_var,
        expression: assigned_to,
        line: token.line.clone(),
    })
}

/*
* Helper functions for building the different Node types.
*/

/// Builds a branch Node at current position in tokens.
fn build_branch(tokens: &mut VecDeque<Token>) -> Result<ast::Branch, ParserError> {
    let line: Box<Line> = tokens.front().unwrap().line.clone();
    if tokens
        .pop_front()
        .expect("Internal compiler error!")
        .token_type
        != TokenType::OpenParen
    {
        print_err(
            &line,
            "Invalid if-statement! Missing opening parenthesis.",
            Some("Write if-statement as if (some condition) {...}"),
        );
        return Err(ParserError::MissingOpenParen);
    }

    let condition = build_condition(tokens);

    if tokens
        .pop_front()
        .expect("Internal compiler error!")
        .token_type
        != TokenType::OpenScope
    {
        print_err(
            &line,
            "Invalid if-statement! Missing true body.",
            Some("Add code to run on true condition if (some condition) { do something... }"),
        );
        return Err(ParserError::MissingOpenScope);
    }

    let body = match parse_body(tokens) {
        Ok(b) => b,
        Err(_) => {
            return Err(ParserError::ParserFailure);
        }
    };
    let true_body: ast::Block = ast::Block { body };

    let false_body: Option<ast::Block> =
        if tokens.front().expect("Internal compiler error!").token_type == TokenType::Branch
            && tokens.front().unwrap().value == "else"
        {
            // TODO: Make the 2 following lines check for valid else statement
            tokens.pop_front().expect("Internal compiler error!"); // Remove "else"
            tokens.pop_front().expect("Internal compiler error!"); // Remove "{"

            let body = match parse_body(tokens) {
                Ok(b) => b,
                Err(_) => {
                    return Err(ParserError::ParserFailure);
                }
            };
            Some(ast::Block { body })
        } else {
            None
        };

    Ok(ast::Branch {
        condition,
        true_body,
        false_body,
        line,
    })
}

/// Builds a Function Node at current position in tokens.
fn build_function(token: &Token, tokens: &mut VecDeque<Token>) -> Option<Box<ast::Function>> {
    let line: Box<Line> = tokens.pop_front().unwrap().line; // Get rid of ( and use its line

    // Build parameters of function
    let mut params: Vec<Box<dyn ast::Node>> = Vec::new();
    while tokens.front().unwrap().token_type != TokenType::CloseParen {
        let mut token = tokens.pop_front().unwrap();

        if token.token_type == TokenType::Seperator {
            token = tokens.pop_front().unwrap();
        }

        let param: Box<dyn ast::Node> = match token.token_type {
            TokenType::Identifier => match build_var_or_value(&token) {
                Some(var_or_val) => var_or_val,
                None => {
                    print_err(
                        &token.line,
                        &format!(
                            "Unable to parse parameter as variable or value: {}",
                            token.value
                        ),
                        None,
                    );
                    return None;
                }
            },
            TokenType::TypeIndicator => match token.value.as_str() {
                "int" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Int(0),
                    line: token.line.clone(),
                }),
                "float" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Float(0.0),
                    line: token.line.clone(),
                }),
                "string" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::String("".to_string()),
                    line: token.line.clone(),
                }),
                "char" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Char(' '),
                    line: token.line.clone(),
                }),
                "void" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Void,
                    line: token.line.clone(),
                }),
                "bool" => Box::new(ast::Type {
                    type_value: ast::ValueEnum::Bool(true),
                    line: token.line.clone(),
                }),
                &_ => {
                    print_err(
                        &token.line,
                        "Unknown type used in function declaration!",
                        Some("Use either, int, float, string, char, bool or void."),
                    );
                    return None;
                }
            },
            TokenType::Integer => build_var_or_value(&token),
            TokenType::Floating => build_var_or_value(&token),
            TokenType::String => build_var_or_value(&token),
            TokenType::Char => build_var_or_value(&token),
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
        line,
    })
}

/// Builds a builtin function node
fn build_builtin(name: &str, tokens: &mut VecDeque<Token>) -> Result<ast::Builtin, ParserError> {
    let line: Box<Line> = tokens
        .front()
        .expect("Internal compiler error!")
        .line
        .clone();

    if tokens
        .pop_front()
        .expect("Internal compiler error!")
        .token_type
        != TokenType::OpenParen
    {
        print_err(
            &line,
            "Missing ( when doing call to built in function!",
            Some(&format!(
                "Use parenthesis after name of built in function: {name}(...) "
            )),
        );
        return Err(ParserError::MissingOpenParen);
    }

    let mut params: Vec<Box<dyn Node>> = Vec::new();
    let mut token: Token = tokens.pop_front().expect("Internal compiler error!");

    while token.token_type != TokenType::CloseParen {
        if token.token_type != TokenType::Seperator {
            let var_or_value = match build_var_or_value(&token) {
                Some(var_or_val) => var_or_val,
                None => return Err(ParserError::ParserFailure),
            };
            params.push(var_or_value);
        }
        token = tokens.pop_front().expect("Internal compiler error!");
    }

    Ok(ast::Builtin {
        identifier: name.to_string(),
        params,
        line,
    })
}

/// Build a loop Node at current position in tokens.
fn build_loop(tokens: &mut VecDeque<Token>) -> Box<ast::Loop> {
    let line: Box<Line> = tokens.front().unwrap().line.clone();
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

    Box::new(ast::Loop {
        condition,
        body,
        line,
    })
}

/// Builds a return Node at current position in tokens.
fn build_return(tokens: &mut VecDeque<Token>) -> Box<ast::Return> {
    let token = tokens.pop_front().unwrap();
    let line: Box<Line> = token.line.clone();

    let return_value: Option<Box<dyn ast::Node>> = if token.token_type == TokenType::Eol {
        None
    } else {
        Some(build_var_or_value(token))
    };

    // Make sure user doesn't try to return anything else, and didn't forget about ';'
    if return_value.is_some() && tokens.pop_front().unwrap().token_type != TokenType::Eol {
        panic!("Missing ;");
    }

    Box::new(ast::Return { return_value, line })
}

/// Helper function for parsing if token is a variable or value
fn build_var_or_value(token: &Token) -> Option<Box<dyn ast::Node>> {
    // Check for identifier, indicating Variable
    if token.token_type == TokenType::Identifier {
        return Some(Box::new(Variable {
            identifier: token.value.clone(),
            var_type: None,
            line: token.line.clone(),
        }));
    };

    // Else assume, Value
    let val = match token.token_type {
        TokenType::Integer => Value {
            value: ValueEnum::Int(token.value.parse::<i16>().unwrap()),
            line: token.line.clone(),
        },
        TokenType::Floating => Value {
            value: ValueEnum::Float(token.value.parse::<f32>().unwrap()),
            line: token.line.clone(),
        },
        TokenType::Char => Value {
            value: ValueEnum::Char(token.value.parse::<char>().unwrap()),
            line: token.line.clone(),
        },
        TokenType::String => Value {
            value: ValueEnum::String(token.value.clone()),
            line: token.line.clone(),
        },
        TokenType::Bool => Value {
            value: ValueEnum::Bool(token.value.parse::<bool>().unwrap()),
            line: token.line.clone(),
        },
        _ => return None,
    };

    Some(Box::new(val))
}

/// Helper function used to build conditions for both Branches and Loops
fn build_condition(tokens: &mut VecDeque<Token>) -> Box<ast::Condition> {
    let line: Box<Line> = tokens.front().unwrap().line.clone();
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
            line,
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
                    line: line.clone(),
                }),
                line,
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
        line,
    })
}

/// Performs some checks and returns a Macro type if a valid existed.
fn build_macro(token: &Token, tokens: &mut VecDeque<Token>) -> Box<ast::Macro> {
    let line: Box<Line> = token.line.clone();
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
                    line,
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

/// Traverses AST and hashes variables based on their location in the program.
/// TODO: Rewrite to generate better variable names, using nested naming, based on
/// tree path used to traverse to it.
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
