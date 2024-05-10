/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use crate::compiler::ast;

use super::memory_manager::{load_const, push_to_stack};

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(assign: &ast::Assignment) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(_assigned_var) = assign.var.as_any().downcast_ref::<ast::Variable>() {
        if let Some(val) = assign.expression.as_any().downcast_ref::<ast::Value>() {
            instructions.push(load_const(4, val.value_as_i16()));
            unsafe {
                instructions.push(push_to_stack(4));
            }
        } else if let Some(other_var) = assign.expression.as_any().downcast_ref::<ast::Variable>() {
        } else if let Some(bin_exp) = assign
            .expression
            .as_any()
            .downcast_ref::<ast::BinaryExpression>()
        {
        } else {
            panic!("Trying to assign variable to something that is niether a value, variable or binary expression!");
        }
    } else {
        panic!("No variable to assign!");
    }

    instructions
}
