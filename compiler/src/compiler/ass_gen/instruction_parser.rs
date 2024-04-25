/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use crate::compiler::ast::{self, Node};

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(
    var: ast::Variable,
    other_var: Option<ast::Variable>,
    value: Option<ast::Value>,
    type_specifier: Option<ast::ValueEnum>,
) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(o_var) = other_var {
        // Var assigned to var
        if let Some(var_type) = type_specifier {
            if var.var_type != var_type {
                // Warn the user!
                println!(
                    "Warning: Type of {} does not match {}",
                    var.get_name(),
                    o_var.get_name()
                )
            }
        }
    } else if let Some(val) = value {
        // Value assigned to var
        if val.value != var.var_type {
            // Warn the user!
            println!(
                "Warning: Type of {} does not match {}",
                var.get_name(),
                val.get_name()
            )
        }
    }

    instructions
}
