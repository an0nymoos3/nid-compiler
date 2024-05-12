/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use super::arithmetic;
use crate::compiler::ast;

use super::memory_manager::{
    get_stack_ptr, load_const, push_to_mem_map, push_to_stack, read_from_dm, read_from_mem_map,
    write_to_dm,
};

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(assign: &ast::Assignment) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(assigned_var) = assign.var.as_any().downcast_ref::<ast::Variable>() {
        if let Some(val) = assign.expression.as_any().downcast_ref::<ast::Value>() {
            instructions.push(load_const(4, val.value_as_i16()));

            if let Some(addr) = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap()) {
                instructions.push(write_to_dm(4, addr));
            } else {
                unsafe {
                    instructions.push(push_to_stack(4));
                    push_to_mem_map(
                        assigned_var.identifier.parse::<u32>().unwrap(),
                        get_stack_ptr() - 1, // Subtract one as push_to_stack() increments by one
                    );
                }
            }
        } else if let Some(other_var) = assign.expression.as_any().downcast_ref::<ast::Variable>() {
            if let Some(addr) = read_from_mem_map(other_var.identifier.parse::<u32>().unwrap()) {
                instructions.push(read_from_dm(4, addr));
                let write_addr = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                    .expect("Trying to write to uninitialized variable!");
                instructions.push(write_to_dm(4, write_addr));
            }
        } else if let Some(bin_exp) = assign
            .expression
            .as_any()
            .downcast_ref::<ast::BinaryExpression>()
        {
            instructions = binary_expression_parser(bin_exp);
            let write_addr = read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap())
                .expect("Trying to write to uninitialized variable!");
            instructions.push(write_to_dm(0, write_addr)); // Write result from reigster to
                                                           // variable
        } else {
            panic!("Trying to assign variable to something that is niether a value, variable or binary expression!");
        }
    } else {
        panic!("No variable to assign!");
    }

    instructions
}

/// Helper function for parsing binary expressions
fn binary_expression_parser(bin_exp: &ast::BinaryExpression) -> Vec<String> {
    let reg1: Option<u8> = None;
    let reg2: Option<u8> = None;

    let mut addr1: Option<u16> = None;
    let mut addr2: Option<u16> = None;

    let mut const1: Option<i16> = None;
    let mut const2: Option<i16> = None;

    // Set addresses of variables used in binary expression.
    if let Some(l_var) = bin_exp.left.as_any().downcast_ref::<ast::Variable>() {
        addr1 = read_from_mem_map(l_var.identifier.parse::<u32>().unwrap());
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
