/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use super::{
    arithmetic,
    memory_manager::{already_in_reg, get_reg, use_reg},
    program_generator::generate_body_ass,
};
use crate::compiler::ast::{self, Node};

use super::memory_manager::{
    get_stack_ptr, load_const, push_to_mem_map, push_to_stack, read_from_dm, read_from_mem_map,
    write_to_dm, MemoryItem,
};

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
                let write_addr = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                    .expect("Trying to write to uninitialized variable!");
                instructions.push(write_to_dm(register, write_addr));
            }
        } else if let Some(bin_exp) = assign
            .expression
            .as_any()
            .downcast_ref::<ast::BinaryExpression>()
        {
            instructions = binary_expression_parser(bin_exp);
            let write_addr = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                .expect("Trying to write to uninitialized variable!");
            instructions.push(write_to_dm(register, write_addr)); // Write result from reigster to
                                                                  // variable
        } else {
            panic!("Trying to assign variable to something that is niether a value, variable or binary expression!");
        }
    } else {
        panic!("No variable to assign!");
    }

    instructions
}

/// Parses if-statements
pub fn parse_branch_statement(branch: &ast::Branch) -> Vec<String> {
    // Add condition instructions to branch instructions
    let mut instructions: Vec<String> = condition_parser(&branch.condition);
    let true_ass = generate_body_ass(branch.true_body.get_body());

    for inst in true_ass {
        instructions.push(inst);
    }

    if let Some(false_body) = &branch.false_body {
        let false_ass = generate_body_ass(false_body.get_body());
        for inst in false_ass {
            instructions.push(inst);
        }
    }

    instructions
}

/// Parses while loops
pub fn parse_loop_statement(nid_loop: &ast::Loop) -> Vec<String> {
    // Add condition instructions to branch instructions
    let mut instructions: Vec<String> = condition_parser(&nid_loop.condition);

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
fn condition_parser(condition: &ast::Condition) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    let mut reg1: Option<u8> = None;
    let reg2: Option<u8> = None;

    let mut addr1: Option<u16> = None;
    let addr2: Option<u16> = None;

    let mut const1: Option<i16> = None;
    let mut const2: Option<i16> = None;

    let op: String = match condition.operator {
        ast::ConditionalOperator::Not => String::from("beq"), // The logic is to think of not in reverse. So you jmp if the variable is eq to 0.
        ast::ConditionalOperator::NotEq => String::from("bne"),
        ast::ConditionalOperator::Eq => String::from("beq"),
        ast::ConditionalOperator::LessThan => String::from("bmi"),
        ast::ConditionalOperator::LessEq => String::from("blt"),
        ast::ConditionalOperator::GreatThan => String::from("bpl"),
        ast::ConditionalOperator::GreatEq => String::from("bge"),
    };

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
    for inst in arithmetic::cmp(reg1, reg2, addr1, addr2, const1, const2) {
        instructions.push(inst);
    }

    // Push jump instruction
    instructions.push(format!("{op}, smthn"));

    instructions
}
