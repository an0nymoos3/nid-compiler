/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use crate::compiler::ast;

use super::memory_manager::{
    get_stack_ptr, heap_alloc, load_const, push_to_mem_map, push_to_stack, read_from_dm,
    read_from_mem_map,
};

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(assign: &ast::Assignment) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(assigned_var) = assign.var.as_any().downcast_ref::<ast::Variable>() {
        if let Some(val) = assign.expression.as_any().downcast_ref::<ast::Value>() {
            instructions.push(load_const(4, val.value_as_i16()));

            if let Some(addr) =
                unsafe { read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap()) }
            {
                instructions.push(heap_alloc(4, addr));
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
            if let Some(addr) =
                unsafe { read_from_mem_map(other_var.identifier.parse::<u32>().unwrap()) }
            {
                instructions.push(read_from_dm(4, addr));
                let write_addr =
                    unsafe { read_from_mem_map(assigned_var.identifier.parse::<u32>().unwrap()) }
                        .expect("Trying to write to uninitialized variable!");
                instructions.push(heap_alloc(4, write_addr));
            }
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
