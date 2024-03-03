use std::collections::VecDeque;
use std::process::exit;

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
    Number,         // A value such as "45"
    Identifier,     // Human readable identifier, such as variable name
    Assignment,     // Assigning operator
    OpenParen,      // (
    CloseParen,     // )
    OpenScope,      // {
    CloseScope,     // }
    BinaryOperator, // +, -, *, /
    Comparison,     // ==, <=, >=
    LogicOperator,  // &&, ||
    Primitive,      // Used to declare variable type and function return
    Loop,
    Condition, // If conditions etc...
    Seperator, // for identifying seperations for things like parameters (,)
    Member,    // . representing a field for something like a struct
    Pointer,   // Same as ptrs in C and C++, points to a memory address
    Refrence,  // -- || --
    Eol,       // End of line, basically ; representing end of line.
    Eof,       // Represents the end of the code (EOF all caps appears to be a reserved
               // word of some kind)
}

#[derive(Debug)]
pub struct Token {
    value: String,
    token_type: TokenType,
}

/// Tokenize source code.
pub fn tokenize(file_content: String) -> VecDeque<Token> {
    let mut token_queue: VecDeque<Token> = VecDeque::new();
    let mut src_code: VecDeque<char> = VecDeque::new();

    for src_char in file_content.chars() {
        if src_char != ' ' && src_char != '\n' && src_char != '\r' {
            src_code.push_back(src_char);
        }
    }

    while !src_code.is_empty() {
        let current_char: &char = src_code.front().expect("Failed to get front()");
        let token: Token;

        if *current_char == '(' {
            token = Token {
                value: String::from("("),
                token_type: TokenType::OpenParen,
            };
        } else if *current_char == ')' {
            token = Token {
                value: String::from(")"),
                token_type: TokenType::CloseParen,
            }
        } else if *current_char == '{' {
            token = Token {
                value: String::from("{"),
                token_type: TokenType::OpenScope,
            }
        } else if *current_char == '}' {
            token = Token {
                value: String::from("}"),
                token_type: TokenType::CloseScope,
            }
        } else if *current_char == ',' {
            token = Token {
                value: String::from(","),
                token_type: TokenType::Seperator,
            }
        } else if *current_char == '.' {
            token = Token {
                value: String::from("."),
                token_type: TokenType::Member,
            }
        } else if *current_char == ';' {
            token = Token {
                value: String::from(";"),
                token_type: TokenType::Eol,
            }
        } else if *current_char == '=' {
            src_code.pop_front();

            if *src_code.front().unwrap() == '=' {
                token = Token {
                    value: String::from("=="),
                    token_type: TokenType::Comparison,
                }
            } else {
                token = Token {
                    value: String::from("="),
                    token_type: TokenType::Assignment,
                };
                src_code.push_front('='); // Push back to src_code to not pop() the next char
            }
        } else if *current_char == '*' {
            src_code.pop_front();

            if is_letter(src_code.front().unwrap()) {
                let mut token_value: String = String::from("*");
                let var_name: String = build_word(&mut src_code, false);
                token_value.push_str(&var_name);
                token = Token {
                    value: token_value,
                    token_type: TokenType::Pointer,
                }
            } else {
                token = Token {
                    value: String::from("*"),
                    token_type: TokenType::BinaryOperator,
                }
            }
        } else if *current_char == '&' {
            src_code.pop_front();
            let mut token_value: String = String::from("&");
            let var_name: String = build_word(&mut src_code, false);
            token_value.push_str(&var_name);
            token = Token {
                value: token_value,
                token_type: TokenType::Refrence,
            }
        } else if is_letter(current_char) {
            let token_value: String = build_word(&mut src_code, false);
            token = Token {
                value: token_value,
                token_type: TokenType::Identifier,
            };
        } else if is_num(current_char) {
            let token_value: String = build_word(&mut src_code, true);
            token = Token {
                value: token_value,
                token_type: TokenType::Number,
            };
        } else {
            println!("Invalid char supplied: {}", current_char);
            src_code.pop_front();
            continue;
            // TODO: Replace with error handling and error message
        }

        token_queue.push_back(token);
        src_code.pop_front();
    }

    token_queue
}

/// Returns if character counts as a letter.
fn is_letter(cur_char: &char) -> bool {
    cur_char.is_alphabetic() || *cur_char == '_'
}

/// Returns whether char counts as a num.
fn is_num(cur_char: &char) -> bool {
    cur_char.is_numeric()
}

/// Returns if a detected word is reserved (eg. void, int, etc)
fn check_reserved_keywords(word: &str) {}

/// Builds a string (or a name) from a series of chars.
fn build_word(src_code: &mut VecDeque<char>, looking_for_num: bool) -> String {
    let mut current_char = src_code
        .front()
        .expect("Failed to get front() (build_name())");
    let mut string_val: String = String::from(*current_char);

    if !looking_for_num {
        while is_letter(current_char) {
            src_code.pop_front();
            current_char = &src_code
                .front()
                .expect("Failed to get front() (build_word())");
            string_val.push(*current_char);
        }
    } else {
        while is_num(current_char) {
            src_code.pop_front();
            current_char = &src_code
                .front()
                .expect("Failed to get front() (build_word())");
            string_val.push(*current_char);
        }
    }

    // Push back last char to not remove next char with src_code.pop() when this function Returns
    src_code.push_front(*current_char);

    string_val
}
