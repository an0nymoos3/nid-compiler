/*
* This file is responsible for parsing tokens as a program.
*/

use std::collections::{HashMap, VecDeque};

use super::lexer::{Token, TokenType};

/// Converts a Dequeu of Tokens into a Vec of strings, representing
/// the binary code of the program.
pub fn parse_tokens(tokens: &mut VecDeque<Token>) -> Vec<String> {
    let mut program: Vec<String> = Vec::new();

    let mut line: String = String::new();
    for (i, token) in tokens.iter().enumerate() {
        // Push the binary representation of the operation
        if token.token_type == TokenType::Operation {
            line.push_str(&op_to_bin(&token.value));

            // A-mode always come after an op.
            // If no a-mode is given, default to 00.
            // tokens.len() is used for bounds checking in the case of
            // instructions that are only an op, such as nop.
            if tokens.len() > i + 1 && tokens.get(i + 1).unwrap().token_type != TokenType::Amode {
                line.push_str("00")
            }
        }

        if token.token_type == TokenType::Amode {
            line.push_str(&token.value);
        }

        if token.token_type == TokenType::Register {
            // Cursed conversion of string representation of decimal number to integer back to
            // binary representation of said number
            line.push_str(format!("{val:04b}", val = token.value.parse::<i32>().unwrap()).as_str());
        }

        if token.token_type == TokenType::Numeric {
            line.push_str(&token.value);
        }

        // At end of line, push to the program and clear
        // for next line of ASS.
        if token.token_type == TokenType::Eol {
            program.push(line.clone());
            line.clear();
        }
    }

    program
}

/// Converts operation name to binary
fn op_to_bin(op_name: &str) -> String {
    let ops = HashMap::from([
        ("nop", "000000"),
        ("ld", "000001"),
        ("ldi", "000010"),
        ("st", "000011"),
        ("psh", "000100"),
        ("pop", "000101"),
        ("add", "000110"),
        ("addi", "000111"),
        ("sub", "001000"),
        ("subi", "001001"),
        ("cmp", "001010"),
        ("cmpi", "001011"),
        ("mul", "001100"),
        ("muli", "001101"),
        ("div", "001110"),
        ("divi", "001111"),
        ("and", "010000"),
        ("andi", "010001"),
        ("or", "010010"),
        ("ori", "010011"),
        ("not", "010100"),
        ("xor", "010101"),
        ("xori", "010110"),
        ("call", "010111"),
        ("ret", "011000"),
        ("jmp", "011001"),
        ("jmpi", "011010"),
        ("beq", "011011"),
        ("bne", "011100"),
        ("bpr", "011101"),
        ("bnr", "011110"),
        ("bge", "011111"),
        ("blt", "100000"),
    ]);

    String::from(*ops.get(op_name).unwrap())
}
