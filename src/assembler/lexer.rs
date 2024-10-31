/*
* This file contains logic for breaking down the text of a .ass program and
* turning it into tokens that the assembler can understand.
*/

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Operation,
    Amode,
    Register,
    Address,
    Constant,
    Seperator,
    Comments,
    BranchName,
    RoutineName,
    EOL,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

/// Debugging function. Prints all tokens to terminal. TODO: Export to file instead of printing.
pub fn export_tokens(tokens: &VecDeque<Token>) {
    for token in tokens {
        println!("Token: {:?}", token);
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
        new_program.push_str(trimmed_line)
    }
    new_program
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
        let current_char: char = src_code.pop_front().expect("Failed to get front()");
        let token: Token;
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
    let keyword_map: HashMap<&str, TokenType> = HashMap::from([]);

    if keyword_map.contains_key(word) {
        return Some(keyword_map[word]);
    }
    None
}

/// Builds a string (a word or number) from a series of chars.
fn build_word(src_code: &mut VecDeque<char>) -> String {
    let mut string_val: String = String::new();

    while is_letter(*src_code.front().unwrap()) {
        string_val.push(*src_code.front().unwrap());
        src_code.pop_front();
    }

    string_val
}

/// Builds a float value for the float TokenType.
fn build_num(src_code: &mut VecDeque<char>) -> String {
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
fn build_string(src_code: &mut VecDeque<char>) -> String {
    let mut string_val: String = String::new();

    while *src_code.front().unwrap() != '"' {
        string_val.push(src_code.pop_front().unwrap());
    }
    src_code.pop_front();

    string_val
}
