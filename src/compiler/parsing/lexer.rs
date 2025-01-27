/*
* This file contains logic for breaking down the text of a .nid program and
* turning it into tokens that the compiler can understand and later
* parse into an AST.
*/

use crate::utils::error::print_err;
use crate::utils::lines::Line;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Integer, // A value such as "45"
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
    Branch,    // If conditions etc...
    Seperator, // for identifying seperations for things like parameters (,)
    Member,    // . representing a field for something like a struct
    Pointer,   // Same as ptrs in C and C++, points to a memory address
    Refrence,  // -- || --
    Return,    // Return statement
    Asm,       // Allows for inline assembly code
    Eol,       // End of line, basically ; representing end of line.
    Eof, // Represents the end of the code (EOF all caps appears to be a reserved word of some kind)
    Macro, // Basic macro functionality, such as allocating memory that the compiler is not allowed
    // to touch
    BuiltIn, // Built in functions, like sleep(), write_to()
}
#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub line: Box<Line>,
}

#[derive(Debug)]
struct SourceCodeContainer {
    pub lines: VecDeque<Line>,
    pub cur_line: Box<Line>,
    pub chars: VecDeque<char>,
}

impl SourceCodeContainer {
    fn pop_front(&mut self) -> Option<char> {
        self.verify_chars();
        self.chars.pop_front()
    }

    fn front(&mut self) -> Option<&char> {
        self.verify_chars();
        self.chars.front()
    }

    fn is_empty(&self) -> bool {
        self.chars.is_empty() && self.lines.is_empty()
    }

    fn verify_chars(&mut self) {
        if self.chars.is_empty() {
            if self.lines.is_empty() {
                panic!(
                    "Internal compiler error! \nTrying to get more characters when there are none!"
                );
            }

            // Do-while loop, to ensure an empty line isn't used.
            self.cur_line = Box::new(self.lines.pop_front().unwrap());
            while self.cur_line.code.is_empty() {
                self.cur_line = Box::new(self.lines.pop_front().unwrap());
            }

            self.chars = VecDeque::from(self.cur_line.code.chars().collect::<Vec<char>>());
        }
    }
}

/// Debugging function. Prints all tokens to terminal. TODO: Export to file instead of printing.
pub fn export_tokens(tokens: &VecDeque<Token>) {
    for token in tokens {
        println!(
            "Token: type: {:?} | value: {} | line: {}",
            token.token_type, token.value, token.line.line_num
        );
    }
}

/// Removes comments from program
pub fn remove_comments(file_content: &str) -> String {
    let mut new_program: String = String::new();

    for line in file_content.lines() {
        let mut trimmed_line: &str = line;
        if let Some(pos) = line.find("//") {
            trimmed_line = &trimmed_line[..pos];
        }
        new_program.push_str(&format!("{trimmed_line}\n"))
    }
    new_program
}

/// Converts the source code from a continuous string of text to a queue of tokens.
pub fn tokenize(code: Vec<Line>) -> Option<VecDeque<Token>> {
    // Returns queue with tokens.
    let mut token_queue: VecDeque<Token> = VecDeque::new();
    let mut code_dequeue: VecDeque<Line> = VecDeque::from(code);

    // Get the line from file_content
    let line: Box<Line> = Box::new(code_dequeue.pop_front().unwrap());

    // Queue for source code to work on.
    let mut src_code: SourceCodeContainer = SourceCodeContainer {
        lines: code_dequeue,
        cur_line: line.clone(),
        chars: VecDeque::from(line.code.chars().collect::<Vec<char>>()),
    };

    while !src_code.is_empty() {
        let current_char: char = src_code.pop_front().expect("Failed to get front()");
        let token: Token;

        /*
         * Parenthesis
         */
        if current_char == '(' {
            token = Token {
                value: String::from("("),
                token_type: TokenType::OpenParen,
                line: src_code.cur_line.clone(),
            };
        } else if current_char == ')' {
            token = Token {
                value: String::from(")"),
                token_type: TokenType::CloseParen,
                line: src_code.cur_line.clone(),
            }

        /*
         * Curly braces or "blocks" or "scopes"
         */
        } else if current_char == '{' {
            token = Token {
                value: String::from("{"),
                token_type: TokenType::OpenScope,
                line: src_code.cur_line.clone(),
            }
        } else if current_char == '}' {
            token = Token {
                value: String::from("}"),
                token_type: TokenType::CloseScope,
                line: src_code.cur_line.clone(),
            }

        /*
         * Check for [], indicating array access
         */
        } else if current_char == '[' {
            token = Token {
                value: String::from("["),
                token_type: TokenType::ArrayAccessOpen,
                line: src_code.cur_line.clone(),
            }
        } else if current_char == ']' {
            token = Token {
                value: String::from("]"),
                token_type: TokenType::ArrayAccessClose,
                line: src_code.cur_line.clone(),
            }

        /*
         * Comma seperator, used for shit like parameters
         */
        } else if current_char == ',' {
            token = Token {
                value: String::from(","),
                token_type: TokenType::Seperator,
                line: src_code.cur_line.clone(),
            }

        /*
         * Member operator, for getting fields in structs, etc...
         */
        } else if current_char == '.' {
            token = Token {
                value: String::from("."),
                token_type: TokenType::Member,
                line: src_code.cur_line.clone(),
            }

        /*
         * End of line sign, used to symbolize a code line ending. Allows for
         * user to write multiple lines of code on one line with ; seperating.
         */
        } else if current_char == ';' {
            token = Token {
                value: String::from(";"),
                token_type: TokenType::Eol,
                line: src_code.cur_line.clone(),
            }

        /*
         * Checks whether "=" is assigning or comparing.
         */
        } else if current_char == '=' {
            if *src_code.front().unwrap() == '=' {
                token = Token {
                    value: String::from("=="),
                    token_type: TokenType::Comparison,
                    line: src_code.cur_line.clone(),
                };
                src_code.pop_front();
            } else {
                token = Token {
                    value: String::from("="),
                    token_type: TokenType::Assignment,
                    line: src_code.cur_line.clone(),
                };
            }

        /*
         * Checking for pointer or multiplication sign.
         */
        } else if current_char == '*' {
            if is_letter(*src_code.front().unwrap()) {
                let mut token_value: String = String::from("*");
                let var_name: String = build_word(&mut src_code);
                token_value.push_str(&var_name);
                token = Token {
                    value: token_value,
                    token_type: TokenType::Pointer,
                    line: src_code.cur_line.clone(),
                }
            } else {
                token = Token {
                    value: String::from("*"),
                    token_type: TokenType::BinaryOperator,
                    line: src_code.cur_line.clone(),
                }
            }

        /*
         * Getting refrence and var_name for refrence.
         */
        } else if current_char == '&' {
            src_code.pop_front();
            let mut token_value: String = String::from("&");
            let var_name: String = build_word(&mut src_code);
            token_value.push_str(&var_name);
            token = Token {
                value: token_value,
                token_type: TokenType::Refrence,
                line: src_code.cur_line.clone(),
            }

        /*
         * Check for not / not equal
         */
        } else if current_char == '!' {
            let value: String;
            if *src_code.front().unwrap() == '=' {
                value = String::from("!=");
                src_code.pop_front();
            } else {
                value = String::from("!");
            }
            token = Token {
                value,
                token_type: TokenType::LogicOperator,
                line: src_code.cur_line.clone(),
            }

        /*
         * Check for greater / greater equal
         */
        } else if current_char == '>' {
            let value: String;
            if *src_code.front().unwrap() == '=' {
                value = String::from(">=");
                src_code.pop_front();
            } else {
                value = String::from(">");
            }
            token = Token {
                value,
                token_type: TokenType::LogicOperator,
                line: src_code.cur_line.clone(),
            }

        /*
         * Check for less / less equal
         */
        } else if current_char == '<' {
            let value: String;
            if *src_code.front().unwrap() == '=' {
                value = String::from("<=");
                src_code.pop_front();
            } else {
                value = String::from("<");
            }
            token = Token {
                value,
                token_type: TokenType::LogicOperator,
                line: src_code.cur_line.clone(),
            }

        /*
         * Check for OR
         */
        } else if current_char == '|' {
            if src_code.pop_front().unwrap() != '|' {
                print_err(
                    &src_code.cur_line.clone(),
                    "Missing second '|' in logical OR operation!",
                    Some("Add second '|'"),
                )
            }

            token = Token {
                value: String::from("||"),
                token_type: TokenType::LogicOperator,
                line: src_code.cur_line.clone(),
            }

        /*
         * Lexes binary operations.
         */
        } else if current_char == '+' || current_char == '-' || current_char == '/' {
            token = Token {
                value: String::from(current_char),
                token_type: TokenType::BinaryOperator,
                line: src_code.cur_line.clone(),
            };
        } else if current_char == '"' {
            let string_value: String = build_string(&mut src_code);
            token = Token {
                value: string_value,
                token_type: TokenType::String,
                line: src_code.cur_line.clone(),
            }
        } else if current_char == '\'' {
            token = Token {
                value: String::from(src_code.pop_front().unwrap()),
                token_type: TokenType::Char,
                line: src_code.cur_line.clone(),
            };
            if src_code.pop_front().unwrap() != '\'' {
                print_err(
                    &src_code.cur_line.clone(),
                    "More than one '\' is not allowed!",
                    None,
                );
            }

        /*
         * Find nid-lang macros
         */
        } else if current_char == '#' {
            let macro_value: String = build_word(&mut src_code);

            token = Token {
                value: macro_value,
                token_type: TokenType::Macro,
                line: src_code.cur_line.clone(),
            };

            /*
             * Finds words, such as reserved keywords, function names, variable names, etc.
             */
        } else if is_letter(current_char) {
            let mut token_value: String = String::from(current_char);
            token_value.push_str(&build_word(&mut src_code));

            if let Some(reserved_word) = is_reserved_keywords(&token_value) {
                token = Token {
                    value: token_value,
                    token_type: reserved_word,
                    line: src_code.cur_line.clone(),
                }
            } else if let Some(builtin) = is_builtin(&token_value) {
                token = Token {
                    value: token_value,
                    token_type: builtin,
                    line: src_code.cur_line.clone(),
                }
            } else {
                token = Token {
                    value: token_value,
                    token_type: TokenType::Identifier,
                    line: src_code.cur_line.clone(),
                };
            }

        /*
         * Gets the full number instead of a single digit at the time.
         */
        } else if is_num(current_char) {
            let mut token_value: String = String::from(current_char);
            token_value.push_str(&build_num(&mut src_code));
            let mut token_type = TokenType::Integer;

            for num in token_value.chars() {
                if num == '.' {
                    token_type = TokenType::Floating;
                }
            }

            token = Token {
                value: token_value,
                token_type,
                line: src_code.cur_line.clone(),
            };

        /*
         * Defaults to error message.
         */
        } else {
            if current_char != ' ' && current_char != '\n' && current_char != '\r' {
                print_err(
                    &src_code.cur_line.clone(),
                    &format!("Invalid character detected! | {current_char}"),
                    None,
                );
            }
            continue; // If not invalid character, jump to next loop
                      // TODO: Replace with error handling and error message
        }
        token_queue.push_back(token);
    }

    Some(token_queue)
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
        ("void", TokenType::TypeIndicator),
        ("int", TokenType::TypeIndicator),
        ("float", TokenType::TypeIndicator),
        ("string", TokenType::TypeIndicator),
        ("char", TokenType::TypeIndicator),
        ("bool", TokenType::TypeIndicator),
        ("true", TokenType::Bool),
        ("false", TokenType::Bool),
        ("if", TokenType::Branch),
        ("else", TokenType::Branch),
        ("while", TokenType::Loop),
        ("return", TokenType::Return),
        ("asm", TokenType::Asm),
    ]);

    if keyword_map.contains_key(word) {
        return Some(keyword_map[word]);
    }
    None
}

/// Returns if a detected word is a builtin function
fn is_builtin(word: &str) -> Option<TokenType> {
    let keyword_map: HashMap<&str, TokenType> = HashMap::from([
        ("sleep", TokenType::BuiltIn),
        ("move_to", TokenType::BuiltIn),
        ("is_pressed", TokenType::BuiltIn),
    ]);

    if keyword_map.contains_key(word) {
        return Some(keyword_map[word]);
    }
    None
}

/// Builds a string (a word or number) from a series of chars.
fn build_word(src_code: &mut SourceCodeContainer) -> String {
    let mut string_val: String = String::new();

    while is_letter(*src_code.front().unwrap()) {
        string_val.push(*src_code.front().unwrap());
        src_code.pop_front();
    }

    string_val
}

/// Builds a float value for the float TokenType.
fn build_num(src_code: &mut SourceCodeContainer) -> String {
    let mut float_string: String = String::new();
    let found_decimal_points: i8 = 0;

    while is_num(*src_code.front().unwrap()) || *src_code.front().unwrap() == '.' {
        if found_decimal_points >= 2 {
            panic!("Too many decimal points found!");
        }
        float_string.push(src_code.pop_front().unwrap());
    }

    float_string
}

/// Builds a string value for the string TokenType.
fn build_string(src_code: &mut SourceCodeContainer) -> String {
    let mut string_val: String = String::new();

    while *src_code.front().unwrap() != '"' {
        string_val.push(src_code.pop_front().unwrap());
    }
    src_code.pop_front();

    string_val
}
