use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Number,           // A value such as "45"
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
    LogicOperator,    // &&, ||
    TypeDecleration,  // Used to declare variable type and function return
    Loop,
    Condition, // If conditions etc...
    Seperator, // for identifying seperations for things like parameters (,)
    Member,    // . representing a field for something like a struct
    Pointer,   // Same as ptrs in C and C++, points to a memory address
    Refrence,  // -- || --
    Return,    // Return statement
    Asm,       // Allows for inline assembly
    Eol,       // End of line, basically ; representing end of line.
    Eof,       // Represents the end of the code (EOF all caps appears to be a reserved
               // word of some kind)
}
#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove once fields are being read
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

/// Converts the source code from a contious string of text to a queue of tokens.
pub fn tokenize(file_content: String) -> VecDeque<Token> {
    // Returns queue with tokens.
    let mut token_queue: VecDeque<Token> = VecDeque::new();

    // Queue for source code to work on.
    let mut src_code: VecDeque<char> = VecDeque::new();

    // Prepare the source code for lexing.
    for src_char in file_content.chars() {
        src_code.push_back(src_char);
    }

    while !src_code.is_empty() {
        let current_char: &char = src_code.front().expect("Failed to get front()");
        let token: Token;

        /*
         * Parenthesis
         */
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

        /*
         * Curly braces or "blocks" or "scopes"
         */
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

        /*
         * Check for [], indicating array access
         */
        } else if *current_char == '[' {
            token = Token {
                value: String::from("["),
                token_type: TokenType::ArrayAccessOpen,
            }
        } else if *current_char == ']' {
            token = Token {
                value: String::from("]"),
                token_type: TokenType::ArrayAccessClose,
            }

        /*
         * Comma seperator, used for shit like parameters
         */
        } else if *current_char == ',' {
            token = Token {
                value: String::from(","),
                token_type: TokenType::Seperator,
            }

        /*
         * Member operator, for getting fields in structs, etc...
         */
        } else if *current_char == '.' {
            token = Token {
                value: String::from("."),
                token_type: TokenType::Member,
            }

        /*
         * End of line sign, used to symbolize a code line ending. Allows for
         * user to write multiple lines of code on one line with ; seperating.
         */
        } else if *current_char == ';' {
            token = Token {
                value: String::from(";"),
                token_type: TokenType::Eol,
            }

        /*
         * Checks whether "=" is assigning or comparing.
         */
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

        /*
         * Checking for pointer or multiplication sign.
         */
        } else if *current_char == '*' {
            src_code.pop_front();

            if is_letter(*src_code.front().unwrap()) {
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

        /*
         * Getting refrence and var_name for refrence.
         */
        } else if *current_char == '&' {
            src_code.pop_front();
            let mut token_value: String = String::from("&");
            let var_name: String = build_word(&mut src_code, false);
            token_value.push_str(&var_name);
            token = Token {
                value: token_value,
                token_type: TokenType::Refrence,
            }

        /*
         * Lexes binary operations.
         */
        } else if *current_char == '+' || *current_char == '-' || *current_char == '/' {
            token = Token {
                value: String::from(*current_char),
                token_type: TokenType::BinaryOperator,
            };

        /*
         * Finds words, such as reserved keywords, function names, variable names, etc.
         */
        } else if is_letter(*current_char) {
            let token_value: String = build_word(&mut src_code, false);

            if let Some(reserved_word) = is_reserved_keywords(&token_value) {
                token = Token {
                    value: token_value,
                    token_type: reserved_word,
                }
            } else {
                token = Token {
                    value: token_value,
                    token_type: TokenType::Identifier,
                };
            }

        /*
         * Gets the full number instead of a single digit at the time.
         */
        } else if is_num(*current_char) {
            let token_value: String = build_word(&mut src_code, true);
            token = Token {
                value: token_value,
                token_type: TokenType::Number,
            };

        /*
         * Defaults to error message.
         */
        } else {
            if *current_char != ' ' && *current_char != '\n' && *current_char != '\r' {
                println!("Invalid char supplied: {}", current_char);
            }
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
fn is_letter(cur_char: char) -> bool {
    cur_char.is_alphabetic() || cur_char == '_'
}

/// Returns whether char counts as a num.
fn is_num(cur_char: char) -> bool {
    cur_char.is_numeric()
}

/// Returns if a detected word is reserved (eg. void, int, etc)
fn is_reserved_keywords(word: &str) -> Option<TokenType> {
    let keyword_map: HashMap<&str, TokenType> = HashMap::from([
        ("void", TokenType::TypeDecleration),
        ("int", TokenType::TypeDecleration),
        ("float", TokenType::TypeDecleration),
        ("string", TokenType::TypeDecleration),
        ("char", TokenType::TypeDecleration),
        ("if", TokenType::TypeDecleration),
        ("while", TokenType::Loop),
        ("return", TokenType::Return),
        ("asm", TokenType::Asm),
    ]);

    if keyword_map.contains_key(word) {
        return Some(keyword_map[word]);
    }
    None
}

/// Builds a string (a word or number) from a series of chars.
fn build_word(src_code: &mut VecDeque<char>, looking_for_num: bool) -> String {
    let mut string_val: String = String::new();

    if !looking_for_num {
        while is_letter(*src_code.front().unwrap()) {
            string_val.push(*src_code.front().unwrap());
            src_code.pop_front();
        }
    } else {
        while is_num(*src_code.front().unwrap()) {
            string_val.push(*src_code.front().unwrap());
            src_code.pop_front();
        }
    }

    // Push back last char to not remove next char with src_code.pop() when this function Returns
    src_code.push_front(*src_code.front().unwrap());

    string_val
}
