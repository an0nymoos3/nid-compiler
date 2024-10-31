/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
* This file essentially contains the helper functions to keep program_generator.rs
* clean.
*/

use super::memory_manager::{
    get_stack_ptr, load_const, push_to_mem_map, push_to_stack, read_from_dm, read_from_mem_map,
    write_to_dm, MemoryItem,
};
use super::{
    arithmetic,
    memory_manager::{already_in_reg, get_reg, use_reg},
    program_generator::generate_body_ass,
};
use crate::compiler::ast::{self, ConditionalOperator, Node};
use crate::compiler::stdlib::input::is_pressed;
use crate::compiler::stdlib::mem::move_to;
use crate::compiler::stdlib::utils::sleep;
use rand::distributions::Alphanumeric;
use rand::Rng;

use super::arithmetic::LATEST_RESULT;

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(assign: &ast::Assignment) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(assigned_var) = assign.var.as_any().downcast_ref::<ast::Variable>() {
        // Get a working register
        let register = get_reg(Some(assigned_var.identifier.parse::<u32>().unwrap()));

        // Case: Assigning a value
        if let Some(val) = assign.expression.as_any().downcast_ref::<ast::Value>() {
            // Push the ldi instruction
            instructions.push(load_const(register, val.value_as_i16()));

            // If variable exists in an address in DM already
            let var_addr: u16;
            if let Some(addr) = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap()) {
                instructions.push(write_to_dm(register, addr));
                var_addr = addr;
            } else {
                // If variable does not already exist in DM
                instructions.push(push_to_stack(register));
                push_to_mem_map(
                    assigned_var.identifier.parse::<u32>().unwrap(),
                    get_stack_ptr() - 1,
                );
                var_addr = get_stack_ptr() - 1;
            }

            // Add the variable as a known variable in register
            use_reg(&MemoryItem {
                var_id: assigned_var.identifier.parse::<u32>().unwrap(),
                reg: Some(register),
                addr: var_addr,
            })
        // Case: Assigning a variable
        } else if let Some(other_var) = assign.expression.as_any().downcast_ref::<ast::Variable>() {
            if let Some(addr) = read_from_mem_map(other_var.identifier.parse::<u32>().unwrap()) {
                instructions.push(read_from_dm(register, addr));

                let var_addr: u16;
                if let Some(write_addr) =
                    read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                {
                    instructions.push(write_to_dm(register, write_addr));
                    var_addr = write_addr;
                } else {
                    // If variable does not already exist in DM
                    instructions.push(push_to_stack(register));
                    push_to_mem_map(
                        assigned_var.identifier.parse::<u32>().unwrap(),
                        get_stack_ptr() - 1,
                    );
                    var_addr = get_stack_ptr() - 1;
                }

                // Add the variable as a known variable in register
                use_reg(&MemoryItem {
                    var_id: assigned_var.identifier.parse::<u32>().unwrap(),
                    reg: Some(register),
                    addr: var_addr,
                })
            }
        } else if let Some(bin_exp) = assign
            .expression
            .as_any()
            .downcast_ref::<ast::BinaryExpression>()
        {
            instructions = binary_expression_parser(bin_exp);
            let write_addr = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                .expect("Trying to write to uninitialized variable!");

            unsafe {
                instructions.push(write_to_dm(LATEST_RESULT, write_addr)); // Write result from reigster to
            }

            // variable
        } else {
            panic!("Trying to assign variable to something that is niether a value, variable or binary expression!");
        }
    } else {
        panic!("No variable to assign!");
    }

    instructions
}

/// Matches the correct builtin function with the correct ass code.
pub fn parse_builtin_functions(builtin: &ast::Builtin) -> Vec<String> {
    match builtin.identifier.as_str() {
        "sleep" => {
            if builtin.params.len() != 1 {
                panic!("Wrong number of arguments supplied to sleep()")
            }
            let time = builtin.params[0]
                .as_any()
                .downcast_ref::<ast::Value>()
                .expect("Invalid type passed as argument to sleep()!")
                .value_as_i16();
            sleep(time as u16)
        }
        "move_to" => {
            if builtin.params.len() != 2 {
                panic!("Wrong number of arguments supplied to move_to()")
            }
            let var_id = builtin.params[0]
                .as_any()
                .downcast_ref::<ast::Variable>()
                .expect("Invalid type passed as first argument to move_to()!");
            let addr = builtin.params[1]
                .as_any()
                .downcast_ref::<ast::Value>()
                .expect("Invalid type passed as second argument to move_to()!")
                .value_as_i16();

            move_to(var_id.identifier.parse::<u32>().unwrap(), addr as u16)
        }

        &_ => {
            panic!("Invalid builtin function supplied!")
        }
    }
}

/// Parses if-statements
pub fn parse_branch_statement(branch: &ast::Branch) -> Vec<String> {
    // Generate assembly jump branches
    let skip_branch: String = random_branch_name();
    let true_branch: String = random_branch_name();

    let mut instructions: Vec<String>;

    // Add condition instructions to branch instructions
    instructions = condition_parser(&branch.condition, &true_branch, branch.false_body.is_some());

    if let Some(false_body) = &branch.false_body {
        let false_ass = generate_body_ass(false_body.get_body());
        for inst in false_ass {
            instructions.push(inst);
        }
    }

    instructions.push(format!("jmp {}", skip_branch)); // Jump past the true body if false was run
    instructions.push(true_branch);
    for inst in generate_body_ass(branch.true_body.get_body()) {
        instructions.push(inst);
    }

    instructions.push(skip_branch);

    instructions
}

/// Parses while loops
pub fn parse_loop_statement(nid_loop: &ast::Loop) -> Vec<String> {
    // Generate assembly jump branches
    let while_body: String = random_branch_name();
    let loop_branch: String = random_branch_name();
    let loop_done: String = random_branch_name();

    let mut instructions: Vec<String> = vec![loop_branch.clone()];
    // Add condition instructions to branch instructions
    let condition: Vec<String> = condition_parser(&nid_loop.condition, &while_body, false);

    if condition.is_empty() {
        return Vec::new(); // Loop will never be run
    }
    for inst in condition {
        instructions.push(inst);
    }

    instructions.push(format!("jmp {}", loop_done));
    instructions.push(while_body);

    let loop_ass = generate_body_ass(nid_loop.body.get_body());

    for inst in loop_ass {
        instructions.push(inst);
    }

    instructions.push(format!("jmp {loop_branch}"));
    instructions.push(loop_done);

    instructions
}

/// Helper function for parsing binary expressions
fn binary_expression_parser(bin_exp: &ast::BinaryExpression) -> Vec<String> {
    let mut reg1: Option<u8> = None;
    let reg2: Option<u8> = None;

    let mut addr1: Option<u16> = None;
    let mut addr2: Option<u16> = None;

    let mut const1: Option<i16> = None;
    let mut const2: Option<i16> = None;

    // Set addresses of variables used in binary expression.
    if let Some(l_var) = bin_exp.left.as_any().downcast_ref::<ast::Variable>() {
        reg1 = already_in_reg(l_var.identifier.parse::<u32>().unwrap());

        // If l_var already in register, dont need to read it's address in memory
        if reg1.is_none() {
            addr1 = read_from_mem_map(l_var.identifier.parse::<u32>().unwrap());
        }
    }

    if let Some(r_var) = bin_exp.right.as_any().downcast_ref::<ast::Variable>() {
        if addr1.is_none() {
            addr1 = read_from_mem_map(r_var.identifier.parse::<u32>().unwrap());
        } else {
            addr2 = read_from_mem_map(r_var.identifier.parse::<u32>().unwrap());
        }
    }

    // Set constant values used in binary expression
    if let Some(l_const) = bin_exp.left.as_any().downcast_ref::<ast::Value>() {
        const1 = Some(l_const.value_as_i16());
    }
    if let Some(r_const) = bin_exp.right.as_any().downcast_ref::<ast::Value>() {
        if const1.is_none() {
            const1 = Some(r_const.value_as_i16());
        } else {
            const2 = Some(r_const.value_as_i16());
        }
    }

    match bin_exp.op {
        ast::BinaryOperator::Add => arithmetic::add(reg1, reg2, addr1, addr2, const1, const2),
        ast::BinaryOperator::Sub => arithmetic::sub(reg1, reg2, addr1, addr2, const1, const2),
        ast::BinaryOperator::Mul => arithmetic::mul(reg1, reg2, addr1, addr2, const1, const2),
        ast::BinaryOperator::Div => arithmetic::div(reg1, reg2, addr1, addr2, const1, const2),
    }
}

/// Helper function to parse the condition of if statements and loops.
/// Note: Handling ! (not) as a special case. Load 0 for comparison and run cmp, check if result is
/// 0
fn condition_parser(
    condition: &ast::Condition,
    branch_name: &str,
    false_body: bool,
) -> Vec<String> {
    // Check if is_pressed was sent as condition
    if let Some(builtin) = condition.right.as_any().downcast_ref::<ast::Builtin>() {
        if builtin.params.len() != 1 {
            panic!("Invalid number of arguments sent to is_pressed()!")
        }
        let scancode = builtin.params[0]
            .as_any()
            .downcast_ref::<ast::Value>()
            .expect("Invalid argument passed to is_pressed()!")
            .value_as_i16();

        return is_pressed(scancode as u16, branch_name);
    }

    let mut instructions: Vec<String> = Vec::new();

    let mut reg1: Option<u8> = None;
    let reg2: Option<u8> = None;

    let mut addr1: Option<u16> = None;
    let addr2: Option<u16> = None;

    let mut const1: Option<i16> = None;
    let mut const2: Option<i16> = None;

    // TODO: FIX PROPER BRANCH SELECTION, CURRENTLY SOME BUGS, LIKE <= BECOMING STRICTLY LESS THAN
    let mut op: String = get_op(&condition.operator, false_body);

    // First perform check on left operand as it's optional.
    if let Some(left_op) = &condition.left {
        // Check for constnat value
        if let Some(l_const) = left_op.as_any().downcast_ref::<ast::Value>() {
            const1 = Some(l_const.value_as_i16());
        // Check for variable (in register or addr in memory)
        } else if let Some(l_var) = left_op.as_any().downcast_ref::<ast::Variable>() {
            // Check if left var is already in a register
            reg1 = already_in_reg(l_var.identifier.parse::<u32>().unwrap());

            // Else load it into addr1
            if reg1.is_none() {
                addr1 = read_from_mem_map(l_var.identifier.parse::<u32>().unwrap());
            }

            if reg1.is_none() && addr1.is_none() {
                panic!("Didn't find addr of variable in mem_map!")
            }
        }
    }

    // Parse right operand in similar fashion as left, except take into account what left can be
    if let Some(r_const) = condition.right.as_any().downcast_ref::<ast::Value>() {
        if const1.is_none() {
            const1 = Some(r_const.value_as_i16());
        } else {
            const2 = Some(r_const.value_as_i16());
        }
    } else if let Some(r_var) = condition.right.as_any().downcast_ref::<ast::Variable>() {
        if reg1.is_none() {
            if let Some(reg) = already_in_reg(r_var.identifier.parse::<u32>().unwrap()) {
                reg1 = Some(reg);
            } else {
                // If compiler gets here, addr1 is already set. Therefore we load reg1 with what
                // would otherwise have gone into addr2
                reg1 = Some(get_reg(Some(r_var.identifier.parse::<u32>().unwrap())));
                instructions.push(read_from_dm(
                    reg1.unwrap(),
                    read_from_mem_map(r_var.identifier.parse::<u32>().unwrap()).unwrap(),
                ))
            }
        } else {
            // If reg1 was set, then addr1 cannot be set
            addr1 = read_from_mem_map(r_var.identifier.parse::<u32>().unwrap());
        }
    } else {
        panic!("Could not parse right operand!")
    }

    // Push the arithmetic instructions that need to be performed
    if const1.is_some() && const2.is_some() {
        if const1.unwrap() != const2.unwrap() {
            // Returning empty vec will tell the compiler that loop is never run
            return Vec::new();
        }
        if const1.unwrap() == const2.unwrap() {
            op = "jmp".to_string();
        }
    } else {
        for inst in arithmetic::cmp(reg1, reg2, addr1, addr2, const1, const2) {
            instructions.push(inst);
        }
    }

    // Push jump instruction
    instructions.push(format!("{op} {branch_name}"));

    instructions
}

fn get_op(operator: &ConditionalOperator, false_body: bool) -> String {
    if false_body {
        return match operator {
            ast::ConditionalOperator::Not => String::from("beq"), // The logic is to think of not in reverse. So you jmp if the variable is eq to 0.
            ast::ConditionalOperator::NotEq => String::from("bne"),
            ast::ConditionalOperator::Eq => String::from("beq"),
            ast::ConditionalOperator::LessThan => String::from("bmi"),
            ast::ConditionalOperator::LessEq => String::from("blt"),
            ast::ConditionalOperator::GreatThan => String::from("bpl"),
            ast::ConditionalOperator::GreatEq => String::from("bge"),
        };
    }
    match operator {
        ast::ConditionalOperator::Not => String::from("beq"), // The logic is to think of not in reverse. So you jmp if the variable is eq to 0.
        ast::ConditionalOperator::NotEq => String::from("bne"),
        ast::ConditionalOperator::Eq => String::from("beq"),
        ast::ConditionalOperator::LessThan => String::from("bmi"),
        ast::ConditionalOperator::LessEq => String::from("blt"),
        ast::ConditionalOperator::GreatThan => String::from("bpl"),
        ast::ConditionalOperator::GreatEq => String::from("bge"),
    }
}

/// Geenrates a random name for a branch to be used in jumps
pub fn random_branch_name() -> String {
    // Create a thread-local RNG (random number generator)
    let rng = rand::thread_rng();

    // Generate a random u16 number
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(16) // Hard coded length of 10 for now
        .map(|mut ch| {
            if !ch.is_ascii_alphabetic() {
                ch = b'a'
            }
            char::from(ch)
        })
        .collect();

    format!("#{}", random_string)
}
