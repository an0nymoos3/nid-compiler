/*
* This file is responsible for parsing tokens as a program.
*/

use super::lexer::{Token, TokenType};
use std::collections::{HashMap, VecDeque};

struct BinInst {
    op: String,
    reg: Option<String>,
    amode: Option<String>,
    val: Option<String>,
}

/// Converts a Dequeu of Tokens into a Vec of strings, representing
/// the binary code of the program.
pub fn parse_tokens(tokens: &mut VecDeque<Token>) -> Vec<u32> {
    let mut program: Vec<String> = Vec::new();
    validate_entry(tokens);
    convert_jumps(tokens);

    let mut inst: BinInst = BinInst {
        op: String::new(),
        reg: None,
        amode: None,
        val: None,
    };
    for (i, token) in tokens.iter().enumerate() {
        // Skip if first token happens to be a newline
        if i == 0 && token.token_type == TokenType::Eol {
            continue;
        }
        // Push the binary representation of the operation
        if token.token_type == TokenType::Operation {
            inst.op = op_to_bin(&token.value);
        } else if token.token_type == TokenType::Amode {
            inst.amode = Some(token.value.clone());
        } else if token.token_type == TokenType::Register {
            // Cursed conversion of string representation of decimal number to integer back to
            // binary representation of said number
            inst.reg = Some(format!(
                "{val:04b}",
                val = token.value.parse::<i32>().unwrap()
            ));
        } else if token.token_type == TokenType::Numeric {
            inst.val = Some(token.value.clone());
        } else if token.token_type == TokenType::Eol {
            // At end of line, push to the program and clear
            // for next line of ASS.
            let mut bin_str: String = inst.op.clone();
            bin_str.push_str(&inst.reg.clone().unwrap_or("0000".to_string()));
            bin_str.push_str(&inst.amode.clone().unwrap_or("00".to_string()));
            bin_str.push_str(&inst.val.clone().unwrap_or("0000000000000000".to_string()));
            program.push(format!("0000{bin_str}")); // Pad with 4 leading 0s to make it a nice and round 32-bits

            // Since an operation is guaranteed, we just clear register, amode and value/address
            inst.reg = None;
            inst.amode = None;
            inst.val = None;
        } else {
            panic!("Invalid token supplied! | {:?}", token);
        }
    }

    program_as_bytes(&program)
}

/// Returns the Vec<String> which is the string representation of the binary
/// as actual bytes. NOTE: The bytes are grouped together in groups of 4
/// to build 32 bits, aka 32 bits per instruction
fn program_as_bytes(program: &[String]) -> Vec<u32> {
    let mut program_bytes: Vec<u32> = Vec::new();
    for inst in program {
        program_bytes.push(u32::from_str_radix(inst, 2).unwrap());
    }
    program_bytes
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
        ("lsr", "010111"),
        ("lsl", "011000"),
        ("call", "011001"),
        ("ret", "011010"),
        ("jmp", "011011"),
        ("jmpi", "011100"),
        ("beq", "011101"),
        ("bne", "011110"),
        ("bpr", "011111"),
        ("bnr", "100000"),
        ("bge", "100001"),
        ("blt", "100010"),
    ]);

    String::from(*ops.get(op_name).unwrap())
}

/// If the first token in the assembly is not "main" it adds a jump
/// instruction to ensure program starts its execution at main.
fn validate_entry(tokens: &mut VecDeque<Token>) {
    if tokens.front().unwrap().value != "main:" {
        tokens.push_front(Token {
            token_type: TokenType::Eol,
            value: String::from(""),
        });

        tokens.push_front(Token {
            token_type: TokenType::Operation,
            value: String::from("jmp"),
        });

        tokens.push_front(Token {
            token_type: TokenType::RoutineName,
            value: String::from("main"),
        });
    }
}

/// Converts the jumps from labels to addresses in PM.
fn convert_jumps(tokens: &mut VecDeque<Token>) {
    let mut jump_map: HashMap<String, usize> = HashMap::new();

    // Find all the jump points
    let mut line_nr: usize = 1;
    for token in tokens.iter_mut() {
        if token.token_type == TokenType::RoutineName && token.value.contains(":") {
            token.value.remove(token.value.len() - 1); // Remove the :
            jump_map.insert(token.value.clone(), line_nr);
        }
        if token.token_type == TokenType::Eol {
            line_nr += 1;
        }
    }

    // Remove from code
    for index in jump_map.values() {
        tokens.remove(*index - 1);
    }

    // Change the jump point names to the program memory address
    for token in tokens.iter_mut() {
        if token.token_type == TokenType::RoutineName {
            token.value = format!("{index:016b}", index = jump_map.get(&token.value).unwrap());
            token.token_type = TokenType::Numeric;
        }
    }
}
